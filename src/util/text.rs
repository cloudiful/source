use bevy::prelude::{AssetServer, default, Res, TextStyle};

// return my custom TextStyle according to font_weight
pub fn my_text_style(font_weight: i32, asset_server: &Res<AssetServer>) -> TextStyle {
    let font_path;
    if font_weight > 0 && font_weight < 300 {
        font_path = String::from("SourceHanSansSC-ExtraLight.otf");
    } else if font_weight >= 300 && font_weight < 400 {
        font_path = String::from("SourceHanSansSC-Light.otf");
    } else if font_weight >= 400 && font_weight < 500 {
        font_path = String::from("SourceHanSansSC-Normal.otf");
    } else if font_weight >= 500 && font_weight < 700 {
        font_path = String::from("SourceHanSansSC-Medium.otf");
    } else {
        font_path = String::from("SourceHanSansSC-Bold.otf");
    }

    return TextStyle {
        font: asset_server.load(font_path),
        font_size: 32.,
        ..default()
    };
}