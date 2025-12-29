use bevy::prelude::*;
const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
// TODO: Move to separate file
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
    Setup,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum Hue {
    RED,
    BLUE,
    GREEN,
    YELLOW,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(pub u32);

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub mod splash {
    use bevy::prelude::*;

    use super::GameState;

    pub fn splash_plugin(app: &mut App) {
        app.add_systems(OnEnter(GameState::Splash), splash_setup)
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)));
    }

    #[derive(Component)]
    struct OnSplashScreen;

    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("branding/icon.png");
        commands.spawn((
            DespawnOnExit(GameState::Splash),
            Node {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: percent(100),
                height: percent(100),
                ..default()
            },
            OnSplashScreen,
            children![(
                ImageNode::new(icon),
                Node {
                    width: px(200),
                    ..default()
                },
            )],
        ));
        commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }

    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).is_finished() {
            game_state.set(GameState::Menu);
        }
    }
}

pub mod game {
    use bevy::prelude::*;

    use crate::generate_world;
    use crate::plugins::camera::CameraPlugin;
    use crate::plugins::ui::UiPlugin;
    use crate::{cleanup, plugins::ui::ResetMapEvent};

    use crate::reset;

    use super::GameState;

    use crate::plugins::world_generator::{
        SeededRng, log_tile, move_player, setup, spawn_fake_player, update_tilemap,
        update_tileset_image,
    };

    //            .add_systems(Update, button_system.run_if(in_state(GameState::Game)))
    pub fn game_plugin(app: &mut App) {
        app //.add_systems(OnEnter(GameState::Game), generate_world)
            .add_systems(OnEnter(GameState::Game), (setup, spawn_fake_player).chain())
            .add_systems(
                Update,
                (
                    update_tileset_image.run_if(in_state(GameState::Game)),
                    update_tilemap.run_if(in_state(GameState::Game)),
                    move_player.run_if(in_state(GameState::Game)),
                    log_tile.run_if(in_state(GameState::Game)),
                ),
            )
            //.add_plugins(CameraPlugin)
            //.add_plugins(UiPlugin)
            .add_message::<ResetMapEvent>()
            .add_systems(Update, reset.run_if(on_message::<ResetMapEvent>))
            .add_systems(OnExit(GameState::Game), cleanup);
    }
}

pub mod menu {
    use bevy::{
        app::AppExit,
        color::palettes::css::CRIMSON,
        ecs::spawn::{SpawnIter, SpawnWith},
        prelude::*,
    };

    use super::{GameState, Hue, TEXT_COLOR, Volume};

    pub fn menu_plugin(app: &mut App) {
        app.init_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
            .add_systems(
                OnEnter(MenuState::SettingsDisplay),
                display_settings_menu_setup,
            )
            .add_systems(
                Update,
                (setting_button::<Hue>.run_if(in_state(MenuState::SettingsDisplay)),),
            )
            .add_systems(OnEnter(MenuState::SettingsSound), sound_settings_menu_setup)
            .add_systems(
                Update,
                setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
            )
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            );
    }

    // State used for the current menu screen
    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        SettingsDisplay,
        SettingsSound,
        #[default]
        Disabled,
    }

    // Tag component used to tag entities added on the main menu screen
    #[derive(Component)]
    struct OnMainMenuScreen;

    // Tag component used to tag entities added on the settings menu screen
    #[derive(Component)]
    struct OnSettingsMenuScreen;

    // Tag component used to tag entities added on the display settings menu screen
    #[derive(Component)]
    struct OnDisplaySettingsMenuScreen;

    // Tag component used to tag entities added on the sound settings menu screen
    #[derive(Component)]
    struct OnSoundSettingsMenuScreen;

    const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

    // Tag component used to mark which setting is currently selected
    #[derive(Component)]
    struct SelectedOption;

    // All actions that can be triggered from a button click
    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Settings,
        SettingsDisplay,
        SettingsSound,
        BackToMainMenu,
        BackToSettings,
        Quit,
    }

    // This system handles changing all buttons color based on mouse interaction
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut background_color, selected) in &mut interaction_query {
            *background_color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    // This system updates the settings when a new value for a setting is selected, and marks
    // the button as the one currently selected
    fn setting_button<T: Resource + Component + PartialEq + Copy>(
        interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
        selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
        mut commands: Commands,
        mut setting: ResMut<T>,
    ) {
        let (previous_button, mut previous_button_color) = selected_query.into_inner();
        for (interaction, button_setting, entity) in &interaction_query {
            if *interaction == Interaction::Pressed && *setting != *button_setting {
                *previous_button_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);
                *setting = *button_setting;
            }
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Common style for all buttons on the screen
        let button_node = Node {
            width: px(300),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_icon_node = Node {
            width: px(30),
            // This takes the icons out of the flexbox flow, to be positioned exactly
            position_type: PositionType::Absolute,
            // The icon will be close to the left border of the button
            left: px(10),
            ..default()
        };
        let button_text_font = TextFont {
            font_size: 33.0,
            ..default()
        };

        let right_icon = asset_server.load("textures/Game Icons/right.png");
        let wrench_icon = asset_server.load("textures/Game Icons/wrench.png");
        let exit_icon = asset_server.load("textures/Game Icons/exitRight.png");

        commands.spawn((
            DespawnOnExit(MenuState::Main),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnMainMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                children![
                    // Display the game name
                    (
                        Text::new("Project Hanamar"),
                        TextFont {
                            font_size: 67.0,
                            ..default()
                        },
                        TextColor(TEXT_COLOR),
                        Node {
                            margin: UiRect::all(px(50)),
                            ..default()
                        },
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Play,
                        children![
                            (ImageNode::new(right_icon), button_icon_node.clone()),
                            (
                                Text::new("New Game"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Settings,
                        children![
                            (ImageNode::new(wrench_icon), button_icon_node.clone()),
                            (
                                Text::new("Settings"),
                                button_text_font.clone(),
                                TextColor(TEXT_COLOR),
                            ),
                        ]
                    ),
                    (
                        Button,
                        button_node,
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::Quit,
                        children![
                            (ImageNode::new(exit_icon), button_icon_node),
                            (Text::new("Quit"), button_text_font, TextColor(TEXT_COLOR),),
                        ]
                    ),
                ]
            )],
        ));
    }

    fn settings_menu_setup(mut commands: Commands) {
        let button_node = Node {
            width: px(200),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_text_style = (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        );

        commands.spawn((
            DespawnOnExit(MenuState::Settings),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                Children::spawn(SpawnIter(
                    [
                        (MenuButtonAction::SettingsDisplay, "Display"),
                        (MenuButtonAction::SettingsSound, "Sound"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ]
                    .into_iter()
                    .map(move |(action, text)| {
                        (
                            Button,
                            button_node.clone(),
                            BackgroundColor(NORMAL_BUTTON),
                            action,
                            children![(Text::new(text), button_text_style.clone())],
                        )
                    })
                ))
            )],
        ));
    }

    fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<Hue>) {
        fn button_node() -> Node {
            Node {
                width: px(200),
                height: px(65),
                margin: UiRect::all(px(20)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }
        }
        fn button_text_style() -> impl Bundle {
            (
                TextFont {
                    font_size: 33.0,
                    ..default()
                },
                TextColor(TEXT_COLOR),
            )
        }

        let display_quality = *display_quality;
        commands.spawn((
            DespawnOnExit(MenuState::SettingsDisplay),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnDisplaySettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                children![
                    (
                        Node {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(CRIMSON.into()),
                        Children::spawn((
                            // Display a label for the current setting
                            Spawn((Text::new("Color"), button_text_style())),
                            SpawnWith(move |parent: &mut ChildSpawner| {
                                for quality_setting in
                                    [Hue::RED, Hue::BLUE, Hue::GREEN, Hue::YELLOW]
                                {
                                    let mut entity = parent.spawn((
                                        Button,
                                        Node {
                                            width: px(150),
                                            height: px(65),
                                            ..button_node()
                                        },
                                        BackgroundColor(NORMAL_BUTTON),
                                        quality_setting,
                                        children![(
                                            Text::new(format!("{quality_setting:?}")),
                                            button_text_style(),
                                        )],
                                    ));
                                    if display_quality == quality_setting {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            })
                        ))
                    ),
                    (
                        Button,
                        button_node(),
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::BackToSettings,
                        children![(Text::new("Back"), button_text_style())]
                    )
                ]
            )],
        ));
    }

    fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
        let button_node = Node {
            width: px(200),
            height: px(65),
            margin: UiRect::all(px(20)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };
        let button_text_style = (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(TEXT_COLOR),
        );

        let volume = *volume;
        let button_node_clone = button_node.clone();
        commands.spawn((
            DespawnOnExit(MenuState::SettingsSound),
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            OnSoundSettingsMenuScreen,
            children![(
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(CRIMSON.into()),
                children![
                    (
                        Node {
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BackgroundColor(CRIMSON.into()),
                        Children::spawn((
                            Spawn((Text::new("Volume"), button_text_style.clone())),
                            SpawnWith(move |parent: &mut ChildSpawner| {
                                for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                    let mut entity = parent.spawn((
                                        Button,
                                        Node {
                                            width: px(30),
                                            height: px(65),
                                            ..button_node_clone.clone()
                                        },
                                        BackgroundColor(NORMAL_BUTTON),
                                        Volume(volume_setting),
                                    ));
                                    if volume == Volume(volume_setting) {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            })
                        ))
                    ),
                    (
                        Button,
                        button_node,
                        BackgroundColor(NORMAL_BUTTON),
                        MenuButtonAction::BackToSettings,
                        children![(Text::new("Back"), button_text_style)]
                    )
                ]
            )],
        ));
    }

    fn menu_action(
        interaction_query: Query<
            (&Interaction, &MenuButtonAction),
            (Changed<Interaction>, With<Button>),
        >,
        mut app_exit_writer: MessageWriter<AppExit>,
        mut menu_state: ResMut<NextState<MenuState>>,
        mut game_state: ResMut<NextState<GameState>>,
    ) {
        for (interaction, menu_button_action) in &interaction_query {
            if *interaction == Interaction::Pressed {
                match menu_button_action {
                    MenuButtonAction::Quit => {
                        app_exit_writer.write(AppExit::Success);
                    }
                    MenuButtonAction::Play => {
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                    MenuButtonAction::SettingsDisplay => {
                        menu_state.set(MenuState::SettingsDisplay);
                    }
                    MenuButtonAction::SettingsSound => {
                        menu_state.set(MenuState::SettingsSound);
                    }
                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                    MenuButtonAction::BackToSettings => {
                        menu_state.set(MenuState::Settings);
                    }
                }
            }
        }
    }
}
