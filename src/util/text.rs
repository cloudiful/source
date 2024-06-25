use bevy::prelude::{default, AssetServer, Res, TextStyle};

// return my custom TextStyle according to font_weight
pub fn my_text_style(font_weight: i32, asset_server: &Res<AssetServer>) -> TextStyle {
    let font_path = String::from("SourceHanSansSC-ExtraLight.otf");

    return TextStyle {
        font: asset_server.load(font_path),
        font_size: 32.,
        ..default()
    };
}
