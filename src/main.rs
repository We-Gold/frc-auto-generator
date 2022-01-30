use macroquad::prelude::*;
mod constants;
mod mouse;
mod point_manager;
mod ui_manager;

#[macroquad::main("FRC Auto Generator")]
async fn main() {
    let mut point_manager = point_manager::create_point_manager();
    let mut ui_manager = ui_manager::create_ui_manager();

    loop {
        clear_background(constants::BACKGROUND);

        // Draw the mouse information
        mouse::draw_mouse_position();

        // Update the GUI
        ui_manager.update_ui();

        // Add, remove, or clear points depending on the user input
        if ui_manager.clear_points_clicked() {
            point_manager.remove_all_points();
        } else {
            if is_mouse_button_pressed(MouseButton::Left) || is_key_pressed(KeyCode::Space) {
                point_manager.add_point(&mouse::get_mouse_vector());
            } else if is_key_pressed(KeyCode::Z) {
                point_manager.remove_last_point();
            }
        }

        // Render the points and the lines connecting them
        point_manager.draw_all_points();

        next_frame().await
    }
}
