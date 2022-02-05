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
    let mut image_manager =
        image_manager::FieldImageManager::new(constants::BACKGROUND_IMAGE_PATH).await;

    loop {
        // Run all of the app logic
        run(&mut point_manager, &mut ui_manager, &mut image_manager);

        next_frame().await
    }
}

fn run(
    point_manager: &mut point_manager::PointManager,
    ui_manager: &mut ui_manager::UIManager,
    image_manager: &mut image_manager::FieldImageManager,
) {
    clear_background(constants::BACKGROUND);

    // Calculate the dimensions for the background image
    image_manager.calculate_image_dimensions_for_frame();

    // Render the background image
    image_manager.render_image();

    // Draw the mouse information
    if image_manager.is_point_in_field(&mouse::get_mouse_pose()) {
        mouse::draw_mouse_position(&image_manager);
    }

    // Draw the mouse cursor
    mouse::draw_cursor(&image_manager);

    // Update the GUI
    ui_manager.update_ui();

    // Add, remove, or clear points depending on the user input
    if ui_manager.clear_points_clicked() {
        point_manager.remove_all_points();
    } else if is_key_pressed(KeyCode::I) {
        // Toggle the GUI
        ui_manager.toggle_gui();
    } else if !ui_manager.output_text_selected() {
        keyboard_manager::check_point_control(point_manager, &image_manager);
    }

    // Render the points and the lines connecting them
    point_manager.draw_all_points(&image_manager);

    // Update the json output text box
    ui_manager.set_output_text(point_manager.to_json());
}
