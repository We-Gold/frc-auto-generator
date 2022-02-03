use macroquad::prelude::*;
mod constants;
mod image_manager;
mod keyboard_manager;
mod mouse;
mod point_manager;
mod pose;
mod ui_manager;

#[macroquad::main("FRC Auto Generator")]
async fn main() {
    let mut point_manager = point_manager::PointManager::new();
    let mut ui_manager = ui_manager::UIManager::new();
    let image_manager = image_manager::ImageManager::new(constants::BACKGROUND_IMAGE_PATH).await;

    loop {
        clear_background(constants::BACKGROUND);

        // Render the background image
        image_manager.render_image();

        // Draw the mouse information
        mouse::draw_mouse_position();

        // Update the GUI
        ui_manager.update_ui();

        // Add, remove, or clear points depending on the user input
        if ui_manager.clear_points_clicked() {
            point_manager.remove_all_points();
        } else if is_key_pressed(KeyCode::I) {
            // Toggle the GUI
            ui_manager.toggle_gui();
        } else if !ui_manager.output_text_selected() {
            keyboard_manager::check_point_control(&mut point_manager);
        }

        // Render the points and the lines connecting them
        point_manager.draw_all_points();

        // Update the json output text box
        ui_manager.set_output_text(point_manager.to_json());

        next_frame().await
    }
}
