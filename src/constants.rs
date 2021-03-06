use macroquad::color::*;

pub const BACKGROUND: Color = BLACK;

pub const CURSOR_THICKNESS: f32 = 3.;
pub const SELECTED_COLOR: Color = GRAY;
pub const MOUSE_POSITION_BOX_OFFSET: f32 = 18.;
pub const BOX_COLOR: Color = Color::new(255., 255., 255., 0.8);
pub const FONT_SIZE: u16 = 17; // Text has inconsistent appearance with an even font size
pub const FONT_SCALE: f32 = 1.;

pub const CIRCLE_RADIUS: f32 = 6.;
pub const CIRCLE_COLOR: Color = WHITE;
pub const CIRCLE_DARK_COLOR: Color = GRAY;

pub const POSE_DIRECTION_COLOR: Color = ORANGE;
pub const POSE_DIRECTION_LENGTH: f32 = 22.;

pub const BUTTON_X_OFFSET: f32 = 80.;
pub const BUTTON_Y_OFFSET: f32 = 20.;

pub const EDITBOX_X_OFFSET: f32 = 200.;
pub const EDITBOX_Y_OFFSET: f32 = 20.;
pub const EDITBOX_WIDTH: f32 = 100.;
pub const EDITBOX_HEIGHT: f32 = 20.;

pub const BACKGROUND_IMAGE_PATH: &str = "./assets/field_image.png";
pub const FIELD_LENGTH: f32 = 16.4592; // 54 ft
pub const FIELD_WIDTH: f32 = 8.2296; // 27 ft
