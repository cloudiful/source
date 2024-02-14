use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component)]
struct StoryPoint {
    text: String,
}

#[derive(Component)]
struct Pos {
    x: f32,
    y: f32,
}

fn add_story_points(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        StoryPoint { text: String::from("hello") },
        Pos {
            x: 100.0,
            y: 100.0,
        }
    ));
    commands.spawn((
        StoryPoint { text: String::from("world") },
        Pos {
            x: 100.0,
            y: 200.0,
        }
    ));
}

fn draw_story_points(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, query: Query<(&StoryPoint, &Pos)>) {
    let text_style = TextStyle {
        font: default(),
        font_size: 32.,
        color: Color::WHITE,
    };

    for (story, pos) in &query {
        let circle_pos = Transform::from_translation(Vec3::new(pos.x, pos.y, 100.));
        let text_pos = Transform::from_translation(Vec3::new(pos.x + 60.0, pos.y, 100.));

        commands.spawn(MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Circle::default())).into(),
            transform: circle_pos.with_scale(Vec3::splat(32.)),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            ..default()
        });

        commands.spawn((Text2dBundle {
            text: Text::from_section(&story.text, text_style.clone()).with_alignment(TextAlignment::Center),
            transform: text_pos,
            ..default()
        },
        ));
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (add_story_points))
        .add_systems(Update, (draw_story_points))
        .run();
}
