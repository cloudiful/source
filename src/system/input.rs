use bevy::ecs::schedule::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::util::text::my_text_style;

pub(crate) struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup)
            .add_systems(Update, (input_handling, update_text).chain())
            .add_event::<CameraMoveEvent>();
    }
}

#[derive(Component)]
struct InputText;

#[derive(Event)]
pub(crate) struct CameraMoveEvent {
    pub(crate) delta: Vec3,
}

fn input_handling(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut event_camera_move: EventWriter<CameraMoveEvent>,
    mut event_mouse_motion: EventReader<MouseMotion>,
) {
    // mouse drag motion
    if !event_mouse_motion.is_empty() && mouse_input.pressed(MouseButton::Left) {
        let mut mouse_motion;
        for e in event_mouse_motion.read() {
            mouse_motion = e.delta;
            event_camera_move.send(CameraMoveEvent {
                delta: Vec3::new(-mouse_motion.x, mouse_motion.y, 0.0),
            });
        }
    }

    // up
    if keyboard_input.just_pressed(KeyCode::KeyW) || keyboard_input.just_pressed(KeyCode::ArrowUp) {
        event_camera_move.send(CameraMoveEvent { delta: Vec3::new(0.0, 50.0, 0.0) });
    }
    // down
    else if keyboard_input.just_pressed(KeyCode::KeyS) || keyboard_input.just_pressed(KeyCode::ArrowDown)
    {
        event_camera_move.send(CameraMoveEvent { delta: Vec3::new(0.0, -50.0, 0.0) });
    }
    // left
    else if keyboard_input.just_pressed(KeyCode::KeyA) || keyboard_input.just_pressed(KeyCode::ArrowLeft)
    {
        event_camera_move.send(CameraMoveEvent { delta: Vec3::new(-50.0, 0.0, 0.0) });
    }
    // right
    else if keyboard_input.just_pressed(KeyCode::KeyD) || keyboard_input.just_pressed(KeyCode::ArrowRight)
    {
        event_camera_move.send(CameraMoveEvent { delta: Vec3::new(50.0, 0.0, 0.0) });
    }
}

// update text when key pressed
fn update_text(
    mut texts: Query<&mut Text, With<InputText>>,
    mut event_camera_move: EventReader<CameraMoveEvent>,
) {
    for ev in event_camera_move.read() {
        for mut q in &mut texts {
            q.sections[0].value = ev.delta.clone().to_string();
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        InputText,
        TextBundle::from_section("none", my_text_style(600, asset_server))
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.),
                left: Val::Px(10.),
                ..default()
            }),
    ));
}