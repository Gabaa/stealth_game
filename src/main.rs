use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<GameState>()
        .add_startup_system(setup)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(game::GamePlugin)
        .run();
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Menu,
    Levels,
    Game,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

mod menu {
    use bevy::{app::AppExit, prelude::*};

    use crate::GameState;

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_system(setup_menu.in_schedule(OnEnter(GameState::Menu)));
            app.add_system(destroy_menu.in_schedule(OnExit(GameState::Menu)));
            app.add_system(button_system.in_set(OnUpdate(GameState::Menu)));
        }
    }

    #[derive(Component)]
    struct MenuScreenRoot;

    #[derive(Component, Clone, Copy, Debug)]
    enum MenuButtonAction {
        Start,
        Editor,
        Quit,
    }

    impl MenuButtonAction {
        fn apply(
            &self,
            game_state: &mut ResMut<NextState<GameState>>,
            exit: &mut EventWriter<AppExit>,
        ) {
            match self {
                MenuButtonAction::Start => game_state.set(GameState::Game),
                MenuButtonAction::Editor => todo!(),
                MenuButtonAction::Quit => exit.send(AppExit),
            }
        }
    }

    fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
        commands
            .spawn((
                MenuScreenRoot,
                NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                button(parent, MenuButtonAction::Start, &asset_server);
                button(parent, MenuButtonAction::Editor, &asset_server);
                button(parent, MenuButtonAction::Quit, &asset_server);
            });
    }

    fn destroy_menu(entities: Query<Entity, With<MenuScreenRoot>>, mut commands: Commands) {
        for entity in &entities {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut game_state: ResMut<NextState<GameState>>,
        mut exit: EventWriter<AppExit>,
    ) {
        for (interaction, mut color, button_option) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    *color = PRESSED_BUTTON.into();
                    button_option.apply(&mut game_state, &mut exit);
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }

    fn button(
        parent: &mut ChildBuilder,
        button_option: MenuButtonAction,
        asset_server: &AssetServer,
    ) {
        parent
            .spawn((
                button_option,
                ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    format!("{button_option:?}"),
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    }
}

mod levels {
    use bevy::{app::AppExit, prelude::*};

    use crate::GameState;

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    pub struct LevelsPlugin;

    impl Plugin for LevelsPlugin {
        fn build(&self, app: &mut App) {
            app.add_system(setup_levels.in_schedule(OnEnter(GameState::Levels)));
            app.add_system(destroy_levels.in_schedule(OnExit(GameState::Levels)));
            app.add_system(button_system.in_set(OnUpdate(GameState::Levels)));
        }
    }

    #[derive(Component)]
    struct LevelsScreenRoot;

    fn setup_levels(mut commands: Commands, asset_server: Res<AssetServer>) {
        asset_server.load("levels")
        fn get_all_level_names() -> Vec<String> {
            let levels_directory = Path::new("levels");
            fs::read_dir(levels_directory)
                .expect("Could not read levels")
                .map(|level| {
                    level
                        .expect("Failed to read level")
                        .file_name()
                        .into_string()
                        .expect("Could not read level name")
                })
                .map(|name| name.replace(".json", ""))
                .collect()
        }

        commands
            .spawn((
                LevelsScreenRoot,
                NodeBundle {
                    style: Style {
                        size: Size::width(Val::Percent(100.0)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                },
            ))
            .with_children(|parent| {
                button(parent, MenuButtonAction::Start, &asset_server);
                button(parent, MenuButtonAction::Editor, &asset_server);
                button(parent, MenuButtonAction::Quit, &asset_server);
            });
    }

    fn destroy_levels(entities: Query<Entity, With<MenuScreenRoot>>, mut commands: Commands) {
        for entity in &entities {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut game_state: ResMut<NextState<GameState>>,
        mut exit: EventWriter<AppExit>,
    ) {
        for (interaction, mut color, button_option) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    *color = PRESSED_BUTTON.into();
                    button_option.apply(&mut game_state, &mut exit);
                }
                Interaction::Hovered => {
                    *color = HOVERED_BUTTON.into();
                }
                Interaction::None => {
                    *color = NORMAL_BUTTON.into();
                }
            }
        }
    }

    fn button(
        parent: &mut ChildBuilder,
        button_option: MenuButtonAction,
        asset_server: &AssetServer,
    ) {
        parent
            .spawn((
                button_option,
                ButtonBundle {
                    style: Style {
                        size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: NORMAL_BUTTON.into(),
                    ..default()
                },
            ))
            .with_children(|parent| {
                parent.spawn(TextBundle::from_section(
                    format!("{button_option:?}"),
                    TextStyle {
                        font: asset_server.load("fonts/Roboto-Regular.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                ));
            });
    }
}

mod game {
    use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

    use crate::GameState;

    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_system(setup_game.in_schedule(OnEnter(GameState::Game)));
            app.add_system(destroy_game.in_schedule(OnExit(GameState::Game)));
            app.add_system(move_system.in_set(OnUpdate(GameState::Game)));
        }
    }

    #[derive(Component)]
    pub struct Game;

    fn setup_game(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn((
            Game,
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ..default()
            },
        ));
    }

    fn destroy_game(entities: Query<Entity, With<Game>>, mut commands: Commands) {
        for entity in &entities {
            commands.entity(entity).despawn_recursive();
        }
    }

    fn move_system(mut query: Query<&mut Transform, With<Game>>, time: Res<Time>) {
        for mut transform in &mut query {
            transform.translation.x += 150. * time.delta_seconds();
        }
    }
}
