use bevy::ecs::schedule::*;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_rapier2d::prelude::{NoUserData, QueryFilter, RapierContext, RapierDebugRenderPlugin, RapierPhysicsPlugin};

use crate::data::default;
use crate::util::text::my_text_style;

pub(crate) struct InputHandlePlugin;

impl Plugin for InputHandlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
            .add_plugins(RapierDebugRenderPlugin::default())
            .add_systems(Startup, setup)
            .add_systems(Update, (input_handling, update_text).chain())
            .add_event::<StoryPageMoveEvent>();
    }
}

#[derive(Component)]
struct InputText;

#[derive(Event)]
pub(crate) struct StoryPageMoveEvent {
    pub(crate) delta: Vec3,
}

fn input_handling(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut event_camera_move: EventWriter<StoryPageMoveEvent>,
    mut event_mouse_motion: EventReader<MouseMotion>,
    mut event_mouse_scroll: EventReader<MouseWheel>,
    mut event_mouse_cursor: EventReader<CursorMoved>,
    rapier_context: Res<RapierContext>,
    window: Query<&Window>,
) {
    let vel_story_point = default::Velocity::new().story_page;
    let mut mouse_cursor_pos;

    for ev in event_mouse_cursor.read() {
        mouse_cursor_pos = ev.position - Vec2::new(window.single().resolution.width() / 2.0,
                                                   window.single().resolution.height() / 2.0);

        let point = mouse_cursor_pos;
        let solid = true;
        let filter = QueryFilter::default();

        // if let Some((entity, projection)) = rapier_context.project_point(
        //     point, solid, filter
        // ) {
        //     // The collider closest to the point has this `handle`.
        //     println!("Projected point on entity {:?}. Point projection: {}", entity, projection.point);
        //     println!("Point was inside of the collider shape: {}", projection.is_inside);
        // }

        rapier_context.intersections_with_point(
            point, filter, |entity| {
                // Callback called on each collider with a shape containing the point.
                println!("The entity {:?} contains the point.", entity);
                // Return `false` instead if we want to stop searching for other colliders containing this point.
                true
            },
        );
    }

    // mouse drag motion
    if !event_mouse_motion.is_empty() && mouse_input.pressed(MouseButton::Left) {
        for e in event_mouse_motion.read() {
            let mouse_motion = e.delta;
            event_camera_move.send(StoryPageMoveEvent {
                delta: Vec3::new(-mouse_motion.x, mouse_motion.y, 0.0),
            });
        }
    }

    // mouse scroll
    use bevy::input::mouse::MouseScrollUnit;
    for ev in event_mouse_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                event_camera_move.send(StoryPageMoveEvent { delta: Vec3::new(-ev.x * vel_story_point, ev.y * vel_story_point, 0.0) });
            }
            MouseScrollUnit::Pixel => {
                event_camera_move.send(StoryPageMoveEvent { delta: Vec3::new(-ev.x, ev.y, 0.0) });
            }
        }
    }

    // keyboard press
    if keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp) {
        event_camera_move.send(StoryPageMoveEvent { delta: Vec3::new(0.0, vel_story_point, 0.0) });
    } else if keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown)
    {
        event_camera_move.send(StoryPageMoveEvent { delta: Vec3::new(0.0, -vel_story_point, 0.0) });
    } else if keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft)
    {
        event_camera_move.send(StoryPageMoveEvent { delta: Vec3::new(-vel_story_point, 0.0, 0.0) });
    } else if keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight)
    {
        event_camera_move.send(StoryPageMoveEvent { delta: Vec3::new(vel_story_point, 0.0, 0.0) });
    }
}

// update text when key pressed
fn update_text(
    mut texts: Query<&mut Text, With<InputText>>,
    mut event_camera_move: EventReader<StoryPageMoveEvent>,
) {
    for ev in event_camera_move.read() {
        for mut q in &mut texts {
            q.sections[0].value = ev.delta.clone().to_string();
        }
    }
}

fn setup(mut commands: Commands,
         asset_server: Res<AssetServer>)
{
    commands.spawn((
        InputText,
        TextBundle::from_section("none", my_text_style(600, &asset_server))
            .with_text_justify(JustifyText::Center)
            .with_style(Style {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            }),
    ));
}
