use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

use crate::system::input::StoryPageMoveEvent;
use crate::util::text::my_text_style;

pub(crate) struct StoryBoardPlugin;

impl Plugin for StoryBoardPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, draw_story_points)
            .add_systems(Update, update_story_points);
    }
}

#[derive(Component)]
struct StoryPoint;

fn draw_story_points(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    for i in 1..10 {
        let story_index = i as f32;
        let pos = Vec3::new(0.0, -100.0 + 100.0 * &story_index, 100.0);
        let pos_text = pos + Vec3::new(50.0, 0.0, 100.0);
        commands.spawn((
            StoryPoint,
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: 16.0 })),
                material: materials.add(Color::GRAY),
                transform: Transform::from_translation(pos),
                ..default()
            },
        ));
        commands.spawn((
            StoryPoint,
            Text2dBundle {
                text: Text::from_section(format!("{}", story_index), my_text_style(500, &asset_server))
                    .with_justify(JustifyText::Center),
                transform: Transform::from_translation(pos_text),
                ..default()
            },
        ));
    }
}

fn update_story_points(mut query: Query<&mut Transform, With<StoryPoint>>,
                       mut event_camera_move: EventReader<StoryPageMoveEvent>) {
    if !event_camera_move.is_empty() {
        for ev in event_camera_move.read() {
            for mut q in &mut query {
                q.translation -= ev.delta;
            }
        }
    }
}