use macroquad::prelude::*;

const MOUSE_POSITION_BOX_OFFSET: f32 = 18.;
const BOX_COLOR: Color = Color::new(255., 255., 255., 0.8);
const FONT_SIZE: u16 = 16;
const FONT_SCALE: f32 = 1.;

const CIRCLE_RADIUS: f32 = 8.;
const CIRCLE_COLOR: Color = WHITE;

fn get_mouse_vector() -> Vec2 {
    let (x, y) = mouse_position();

    return Vec2::new(x, y);
}

fn draw_mouse_position() {
    show_mouse(false);

    let mouse: Vec2 = get_mouse_vector();

    // Draw an empty circle to represent the position of the mouse
    draw_circle_lines(mouse.x, mouse.y, CIRCLE_RADIUS, 3., CIRCLE_COLOR);

    // Draw the mouse position (round to one decimal place)
    let mouse_text = format!("{}, {}", mouse.x, mouse.y);

    let text_size = measure_text(&mouse_text, None, FONT_SIZE, FONT_SCALE);

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

    let box_corner = Vec2::new(
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

#[macroquad::main("FRC Auto Generator")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_mouse_position();

        next_frame().await
    }
}
