use crate::constants::*;
use crate::pose::Pose;
use macroquad::prelude::*;

pub struct FieldImageManager {
    texture: Texture2D,
    aspect_ratio: f32,
}

impl FieldImageManager {
    pub async fn new(image_path: &str) -> FieldImageManager {
        // Load in the image
        let image_result = load_image(image_path).await;
        let image = image_result.unwrap();
        let (image_width, image_height) = (image.width() as f32, image.height() as f32);
        let texture = Texture2D::from_image(&image);

        FieldImageManager {
            texture: texture,
            aspect_ratio: FieldImageManager::calculate_aspect_ratio(image_width, image_height),
        }
    }

    pub fn render_image(&self) {
        let (x, y, width, height) = self.calculate_image_size();

        // Store parameters for correctly drawing the image/texture
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(width, height)),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        // Render the texture
        draw_texture_ex(self.texture, x, y, GRAY, params);
    }

    pub fn is_point_in_field(&self, point: &Pose) -> bool {
        let (x, y, width, height) = self.calculate_image_size();

        // Check if the point is within the field.
        point.x >= x && point.y >= y && point.x <= x + width && point.y <= y + height
    }

    // Source: https://stackoverflow.com/questions/67016985/map-numeric-range-rust
    // Maps a number from a given range to a different range
    fn map_to_range(num: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
        to_range.0
            + (num - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
    }

    /// Converts the given pose to a field-relative pose
    pub fn to_field_pose(&self, point: &Pose) -> Pose {
        let x = FieldImageManager::map_to_range(
            point.x,
            (0., screen_width()),
            (-FIELD_LENGTH / 2., FIELD_LENGTH / 2.),
        );

        let y = FieldImageManager::map_to_range(
            point.y,
            (0., screen_height()),
            (FIELD_WIDTH / 2., -FIELD_WIDTH / 2.),
        );

        Pose::new_with_theta(x, y, point.theta)
    }

    /// Converts the given pose to a screen-relative pose
    pub fn from_field_pose(&self, point: &Pose) -> Pose {
        let x = FieldImageManager::map_to_range(
            point.x,
            (-FIELD_LENGTH / 2., FIELD_LENGTH / 2.),
            (0., screen_width()),
        );

        let y = FieldImageManager::map_to_range(
            point.y,
            (FIELD_WIDTH / 2., -FIELD_WIDTH / 2.),
            (0., screen_height()),
        );

        Pose::new_with_theta(x, y, point.theta)
    }

    pub fn calculate_image_size(&self) -> (f32, f32, f32, f32) {
        // Calculate the screen aspect ratio
        let screen_aspect_ratio =
            FieldImageManager::calculate_aspect_ratio(screen_width(), screen_height());

        // Calculate the height and width of the image
        let (width, height) = if screen_aspect_ratio < self.aspect_ratio {
            (screen_width(), screen_width() / self.aspect_ratio)
        } else {
            (screen_height() * self.aspect_ratio, screen_height())
        };

        // Calculate the coordinates of the image
        let (x, y) = (
            (screen_width() - width) / 2.,
            (screen_height() - height) / 2.,
        );

        return (x, y, width, height);
    }

    pub fn calculate_aspect_ratio(width: f32, height: f32) -> f32 {
        width / height
    }
}
