use crate::constants::*;
use crate::image_manager::FieldImageManager;
use crate::pose::Pose;
use macroquad::prelude::*;

pub fn get_mouse_pose() -> Pose {
    let (x, y) = mouse_position();

    return Pose::new(x, y);
}

pub fn draw_cursor(image_manager: &FieldImageManager) {
    // Hide the default cursor
    show_mouse(false);

    let mouse_screen_pose = get_mouse_pose();

    let within_field = image_manager.is_point_in_field(&mouse_screen_pose);

    let color = if within_field {
        CIRCLE_COLOR
    } else {
        CIRCLE_DARK_COLOR
    };

    // Draw an empty circle to represent the position of the mouse
    draw_circle_lines(
        mouse_screen_pose.x,
        mouse_screen_pose.y,
        CIRCLE_RADIUS,
        CURSOR_THICKNESS,
        color,
    );
}

pub fn draw_mouse_position(image_manager: &FieldImageManager) {
    let mouse_screen_pose = get_mouse_pose();
    let mouse_field_pose = image_manager.to_field_pose(&mouse_screen_pose);

    // Draw the mouse position
    let mouse_text = format!("{:.2}, {:.2}", mouse_field_pose.x, mouse_field_pose.y);

    // Find the dimensions of the text
    let text_size = measure_text(&mouse_text, None, FONT_SIZE, FONT_SCALE);

    // Draw the mouse position info text
    draw_text(
        &mouse_text,
        mouse_screen_pose.x - text_size.width / 2.0,
        mouse_screen_pose.y + MOUSE_POSITION_BOX_OFFSET + text_size.height,
        FONT_SIZE.into(),
        BOX_COLOR,
    );

    // Draw a box that will contain the mouse position text
    let box_width = text_size.width + 4.;
    let box_height = text_size.height + 4.;
    let box_corner = Pose::new(
        mouse_screen_pose.x - (box_width / 2.),
        mouse_screen_pose.y + MOUSE_POSITION_BOX_OFFSET,
    );

    draw_rectangle_lines(
        box_corner.x,
        box_corner.y,
        box_width,
        box_height,
        1.5,
        BOX_COLOR,
    );
}

pub fn draw_mouse_rotation(angle: f32) {
    let mouse_screen_pose = get_mouse_pose();

    // Draw the mouse position
    let rotation_text = format!("{:.1}", angle);

    // Find the dimensions of the text
    let text_size = measure_text(&rotation_text, None, FONT_SIZE, FONT_SCALE);

    // Draw the mouse position info text
    draw_text(
        &rotation_text,
        mouse_screen_pose.x - text_size.width / 2.0,
        mouse_screen_pose.y - MOUSE_POSITION_BOX_OFFSET, // - text_size.height,
        FONT_SIZE.into(),
        BOX_COLOR,
    );

    // Draw a box that will contain the mouse position text
    let box_width = text_size.width + 4.;
    let box_height = text_size.height + 4.;
    let box_corner = Pose::new(
        mouse_screen_pose.x - (box_width / 2.),
        mouse_screen_pose.y - MOUSE_POSITION_BOX_OFFSET - text_size.height - 1.5,
    );

    draw_rectangle_lines(
        box_corner.x,
        box_corner.y,
        box_width,
        box_height,
        1.5,
        BOX_COLOR,
    );
}
