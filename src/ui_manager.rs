use crate::constants::*;
use crate::mouse::get_mouse_pose;
use crate::pose::Pose;
use macroquad::prelude::*;
use macroquad::ui::root_ui;
use macroquad::ui::widgets::Editbox;

pub struct UIManager {
    clear_points: bool,
    output_text: String,
    output_text_hovered: bool,
    show_gui: bool,
}

impl UIManager {
    pub fn new() -> UIManager {
        return UIManager {
            clear_points: false,
            output_text: String::new(),
            output_text_hovered: false,
            show_gui: true,
        };
    }

    pub fn update_ui(&mut self) {
        // Render nothing if show gui is toggled off
        if !self.show_gui {
            return;
        }

        // Render a button that when clicked will clear all points
        self.clear_points = root_ui().button(
            Vec2::new(screen_width() - BUTTON_X_OFFSET, BUTTON_Y_OFFSET),
            "Clear",
        );

        // Render any output text to an edit box
        let editbox = Editbox::new(1, Vec2::new(EDITBOX_WIDTH, EDITBOX_HEIGHT)).position(
            Vec2::new(screen_width() - EDITBOX_X_OFFSET, EDITBOX_Y_OFFSET),
        );

        self.output_text_hovered = editbox.ui(&mut root_ui(), &mut self.output_text)
            || (UIManager::is_in_bounds(
                get_mouse_pose(),
                Vec2::new(screen_width() - EDITBOX_X_OFFSET, EDITBOX_Y_OFFSET),
                Vec2::new(EDITBOX_WIDTH, EDITBOX_HEIGHT),
            ) && is_mouse_button_down(MouseButton::Left));
    }

    pub fn clear_points_clicked(&self) -> bool {
        // Returns whether or not we should clear the points
        self.clear_points
    }

    pub fn output_text_selected(&self) -> bool {
        // Returns if the output text box is currently selected
        self.output_text_hovered
    }

    pub fn set_output_text(&mut self, text: String) {
        self.output_text = text;
    }

    pub fn toggle_gui(&mut self) {
        self.show_gui = !self.show_gui;
    }

    fn is_in_bounds(pose: Pose, corner: Vec2, dimensions: Vec2) -> bool {
        return pose.x >= corner.x
            && pose.y >= corner.y
            && pose.x <= corner.x + dimensions.x
            && pose.y <= corner.y + dimensions.y;
    }
}
