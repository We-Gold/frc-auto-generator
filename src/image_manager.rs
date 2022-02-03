use macroquad::prelude::*;

pub struct ImageManager {
    image: Image,
    texture: Texture2D,
    aspect_ratio: f32,
    width: f32,
    height: f32,
}

impl ImageManager {
    pub async fn new(image_path: &str) -> ImageManager {
        // Load in the image
        let image_result = load_image(image_path).await;
        let image = image_result.unwrap();
        let (image_width, image_height) = (image.width() as f32, image.height() as f32);
        let texture = Texture2D::from_image(&image);

        ImageManager {
            image: image,
            texture: texture,
            aspect_ratio: ImageManager::calculate_aspect_ratio(image_width, image_height),
            width: image_width,
            height: image_height,
        }
    }

    pub fn render_image(&self) {
        // Calculate the screen aspect ratio
        let screen_aspect_ratio =
            ImageManager::calculate_aspect_ratio(screen_width(), screen_height());

        // Calculate the height and width of the image
        let (_width, _height) = if screen_aspect_ratio < self.aspect_ratio {
            (screen_width(), screen_width() / self.aspect_ratio)
        } else {
            (screen_height() * self.aspect_ratio, screen_height())
        };

        // Store parameters for correctly drawing the image/texture
        let params = DrawTextureParams {
            dest_size: Some(Vec2::new(_width, _height)),
            source: None,
            rotation: 0.,
            flip_x: false,
            flip_y: false,
            pivot: None,
        };

        let (x, y) = (
            (screen_width() - _width) / 2.,
            (screen_height() - _height) / 2.,
        );

        // Render the texture
        draw_texture_ex(self.texture, x, y, GRAY, params);
    }

    pub fn calculate_aspect_ratio(width: f32, height: f32) -> f32 {
        width / height
    }
}
