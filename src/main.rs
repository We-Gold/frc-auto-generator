use macroquad::input::*;
use macroquad::prelude::*;

struct Circle {
    pos: Vec2,
    radius: f32,
    speed: f32,
    color: Color,
}

impl Circle {
    fn translate(&mut self, vec: Vec2) {
        self.pos += vec;
    }

    fn draw(&self) {
        draw_circle(self.pos.x, self.pos.y, self.radius, self.color);
    }

    fn calculate_velocity(&self, delta_t: f32) -> Vec2 {
        let mut vel = Vec2::new(0., 0.);

        // Check for vertical movement
        if is_key_down(KeyCode::Up) {
            vel.y -= self.speed * delta_t;
        } else if is_key_down(KeyCode::Down) {
            vel.y += self.speed * delta_t;
        }

        // Check for horizontal movement
        if is_key_down(KeyCode::Left) {
            vel.x -= self.speed * delta_t;
        } else if is_key_down(KeyCode::Right) {
            vel.x += self.speed * delta_t;
        }

        return vel;
    }
}

#[macroquad::main("Moving Circle")]
async fn main() {
    let mut circle = Circle {
        pos: Vec2::new(screen_width() / 2., screen_height() / 2.),
        radius: 20.,
        speed: 200.,
        color: WHITE,
    };

    loop {
        clear_background(BLACK);

        let delta_t = get_frame_time();

        // Calculate the current circle velocity
        // and move the circle by that velocity
        circle.translate(circle.calculate_velocity(delta_t));

        circle.draw();

        next_frame().await
    }
}
