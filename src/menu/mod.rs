use crate::AppState;
use bevy::app::AppExit;
use bevy::prelude::*;

#[derive(Component)]
pub enum MenuButton {
    Play,
    Quit,
}

#[derive(Component)]
struct MainMenuData {
    camera_entity: Entity,
    ui_root: Entity,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MenuMaterials>()
            .add_system(button_system.label("Button_System"))
            .add_system(button_press_system.label("Button_Press_System"))
            .add_system_set(SystemSet::on_update(AppState::InGame).with_system(back_to_main_menu_controls.label("Backing to menu control system")))
            //.add_system_set(SystemSet:
            .add_system_set(SystemSet::on_exit(AppState::MainMenu).with_system(cleanup))
            .add_system_set(SystemSet::on_enter(AppState::MainMenu).with_system(setup))
            ;
    }
}

struct MenuMaterials {
    root: UiColor,
    border: UiColor,
    menu: UiColor,
    button: UiColor,
    button_hovered: UiColor,
    button_pressed: UiColor,
    button_text: Color,
}

impl FromWorld for MenuMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        MenuMaterials {
            root: UiColor(Color::NONE.into()),
            border: UiColor(Color::rgb(0.65, 0.65, 0.65)),
            menu: UiColor(Color::rgb(0.15, 0.15, 0.15)),
            button: UiColor(Color::rgb(0.15, 0.15, 0.15)),
            button_hovered: UiColor(Color::rgb(0.15, 0.25, 0.35)),
            button_pressed: UiColor(Color::rgb(0.25, 0.15, 0.05)),
            button_text: Color::WHITE,
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, materials: Res<MenuMaterials>) {
    let camera_entity = commands.spawn_bundle(UiCameraBundle::default()).id();

    let ui_root = commands
        .spawn_bundle(root(&materials))
        .with_children(|parent| {
            // left vertical fill (border)
            parent
                .spawn_bundle(border(&materials))
                .with_children(|parent| {
                    // left vertical fill (content)
                    parent
                        .spawn_bundle(menu_background(&materials))
                        .with_children(|parent| {
                            parent.spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&asset_server, &materials, "New Game"));
                                })
                                .insert(MenuButton::Play)
                                ;
                            parent.spawn_bundle(button(&materials))
                                .with_children(|parent| {
                                    parent.spawn_bundle(button_text(&asset_server, &materials, "Quit"));
                                })
                                .insert(MenuButton::Quit)
                                
                                ;
                        });
                });
        }).id();
        commands.insert_resource(MainMenuData {
            camera_entity,
            ui_root,
        });
}

fn root(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.root.clone(),
        ..Default::default()
    }
}

fn button(materials: &Res<MenuMaterials>) -> ButtonBundle {
    ButtonBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: materials.button.clone(),
        ..Default::default()
    }
}

fn button_system(
    materials: Res<MenuMaterials>,
    mut buttons: Query<(&Interaction, &mut UiColor), (Changed<Interaction>, With<Button>)>,
) {
    for (interaction, mut material) in buttons.iter_mut() {
        match *interaction {
            Interaction::Clicked => *material = materials.button_pressed.clone(),
            Interaction::Hovered => *material = materials.button_hovered.clone(),
            Interaction::None    => *material = materials.button.clone(),
        }
    }
}
fn button_press_system(
    buttons: Query<(&Interaction, &MenuButton), (Changed<Interaction>, With<Button>)>,
    mut state: ResMut<State<crate::AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button) in buttons.iter() {
        if *interaction == Interaction::Clicked {
            match button {
                MenuButton::Play => state
                    .set(crate::AppState::InGame)
                    .expect("Couldn't switch state to InGame"),
                MenuButton::Quit => exit.send(AppExit),
            };
        }
    }
}

fn button_text(
    asset_server: &Res<AssetServer>,
    materials: &Res<MenuMaterials>,
    label: &str,
) -> TextBundle {
    return TextBundle {
        style: Style {
            margin: Rect::all(Val::Px(10.0)),
            ..Default::default()
        },
        text: Text::with_section(
            label,
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: materials.button_text.clone(),
            },
            Default::default(),
        ),
        ..Default::default()
    };
}

fn border(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Px(400.0), Val::Auto),
            border: Rect::all(Val::Px(8.0)),
            ..Default::default()
        },
        color: materials.border.clone(),
        ..Default::default()
    }
}

fn menu_background(materials: &Res<MenuMaterials>) -> NodeBundle {
    NodeBundle {
        style: Style {
            size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::ColumnReverse,
            padding: Rect::all(Val::Px(5.0)),
            ..Default::default()
        },
        color: materials.menu.clone(),
        ..Default::default()
    }
}
fn back_to_main_menu_controls(
    mut keys: ResMut<Input<KeyCode>>,
    mut app_state: ResMut<State<crate::AppState>>,
) {
    if *app_state.current() == crate::AppState::InGame {
        if keys.just_pressed(KeyCode::Escape) {
            app_state.set(crate::AppState::MainMenu).unwrap();
            keys.reset(KeyCode::Escape);
        }
    }
}

fn cleanup(mut commands: Commands, menu_data: Res<MainMenuData>) {
    commands.entity(menu_data.ui_root).despawn_recursive();
    commands.entity(menu_data.camera_entity).despawn_recursive();
}