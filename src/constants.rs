use ratatui::style::Color;


pub static INSTANCE_FILE_NAME: &str = "instances.json";

pub static INSTANCE_COLOR: Color = Color::Rgb(41, 174, 255);
pub static INDEX_COLOR: Color = Color::Rgb(255, 205, 170);
pub static ACTION_MODE_COLOR: Color = Color::Rgb(0, 135, 81);
pub static DELETE_MODE_COLOR: Color = Color::Red;

pub static APP_NAME_ASCII: &str = r#"╔╦╗┌─┐┬┬  ┬╔═╗┬┌┐┌┌┬┐┌─┐┬─┐
║║║├┤ ││  │╠╣ ││││ ││├┤ ├┬┘
╩ ╩└─┘┴┴─┘┴╚  ┴┘└┘─┴┘└─┘┴└─"#;