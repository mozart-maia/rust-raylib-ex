// use raylib::ffi::Color;
use raylib::prelude::*;
use raylib::consts::KeyboardKey::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;

struct Ball {
    position: Vector2,
    speed: f32,
    radius: f32,
    color: Color
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Move the ball")
        .build();

    let mut ball = Ball {
        position: Vector2::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
        speed: 0.3,
        radius: 40.0,
        color: Color::GREEN
    };
     
    while !rl.window_should_close() {
        /* UPDATE */
        if rl.is_key_down(KEY_RIGHT) { ball.position.x += ball.speed; }
        if rl.is_key_down(KEY_DOWN) { ball.position.y += ball.speed; }
        if rl.is_key_down(KEY_LEFT) { ball.position.x -= ball.speed; }
        if rl.is_key_down(KEY_UP) { ball.position.y -= ball.speed; }

        if rl.is_key_pressed(KEY_SPACE){
            if ball.color == Color::GREEN {
                ball.color = Color::YELLOW;
            } else {
                ball.color = Color::GREEN;
            }
        }

        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::BLACK);
        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
        d.draw_circle_v(ball.position, ball.radius, ball.color);
    }
}