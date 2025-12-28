use crate::plugins::game_menu::GameState;
use bevy::prelude::*;

const NORMAL_BUTTON: Color = Color::linear_rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::linear_rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::linear_rgb(0.35, 0.75, 0.35);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), setup_button)
            .add_systems(Update, button_system.run_if(in_state(GameState::Game)))
            .add_message::<ResetMapEvent>();
    }
}

#[derive(Event, Message)]
pub struct ResetMapEvent;

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut events: MessageWriter<ResetMapEvent>,
    mut mouse_buttons: ResMut<ButtonInput<MouseButton>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                mouse_buttons.clear_just_pressed(MouseButton::Left);
                *color = PRESSED_BUTTON.into();
                events.write(ResetMapEvent);
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

pub fn setup_button(mut commands: Commands) {
    commands
        .spawn(Node {
            right: Val::Px(10.0),
            top: Val::Px(10.0),
            position_type: PositionType::Absolute,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(80.0),
                        height: Val::Px(45.0),
                        border: UiRect::all(Val::Px(3.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|parent| {
                    parent.spawn((Text::new("Exit"), TextColor(Color::srgb(0.9, 0.9, 0.9))));
                });
        });
}
