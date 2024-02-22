use bevy::app::Startup;
use bevy::DefaultPlugins;
use bevy::prelude::{App, Camera2dBundle, Commands, Component};

mod system {
    pub(crate) mod input;
    pub(crate) mod storyboard;
}

mod util {
    pub(crate) mod text;
}

mod data{
    pub(crate) mod default;
}

#[derive(Component)]
pub(crate) struct MainCamera;

fn setup(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        Camera2dBundle::default()
    ));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            system::input::InputPlugin,
            system::storyboard::StoryBoardPlugin,
        ))
        .add_systems(Startup, setup)
        .run();
}

