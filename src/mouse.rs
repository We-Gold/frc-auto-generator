use crate::constants::*;
use crate::pose::Pose;
use macroquad::prelude::*;

pub fn get_mouse_pose() -> Pose {
    let (x, y) = mouse_position();

    return Pose::new(x, y);
}

pub fn draw_mouse_position() {
    show_mouse(false);

    let mouse: Pose = get_mouse_pose();

    // Draw an empty circle to represent the position of the mouse
    draw_circle_lines(
        mouse.x,
        mouse.y,
        CIRCLE_RADIUS,
        CURSOR_THICKNESS,
        CIRCLE_COLOR,
    );

    // Draw the mouse position (round to one decimal place)
    let mouse_text = format!("{}, {}", mouse.x, mouse.y);

    // Find the dimensions of the text
    let text_size = measure_text(&mouse_text, None, FONT_SIZE, FONT_SCALE);

    // Draw the mouse position info text
    draw_text(
        &mouse_text,
        mouse.x - text_size.width / 2.0,
        mouse.y + MOUSE_POSITION_BOX_OFFSET + text_size.height,
        FONT_SIZE.into(),
        BOX_COLOR,
    );

    // Draw a box that will contain the mouse position text
    let box_width = text_size.width + 4.;
    let box_height = text_size.height + 4.;
    let box_corner = Pose::new(
        mouse.x - (box_width / 2.),
        mouse.y + MOUSE_POSITION_BOX_OFFSET,
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
