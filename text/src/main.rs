use raylib::prelude::*;

const SCREEN_WIDTH: f32 = 640.0;
const SCREEN_HEIGHT: f32 = 480.0;


fn main() {
    let (mut rl, thread) = raylib::init()
        .size(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32)
        .title("Random values")
        .build();

    rl.set_target_fps(60);

    let mut value: i32 = get_random_value(-100, 100);
    let mut frame_count = 0;
     
    while !rl.window_should_close() {
        /* UPDATE */
        frame_count += 1;
        if frame_count % 60 == 0 {
            value = get_random_value(-100, 100);
            frame_count = 0;
        }
        /* DRAW */
        let mut d = rl.begin_drawing(&thread);
         
        d.clear_background(Color::BLACK);
        draw_text_center(
            &mut d,
            "A cada segundo um valor aleatorio novo é gerado: ",
            (SCREEN_HEIGHT as i32) / 2 - 40, 
            20, 
            Color::WHITE);

        draw_text_center(
            &mut d,
            format!("{}", value).as_str(),
            (SCREEN_HEIGHT as i32) / 2, 
            20, 
            Color::BLUE);
        // d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}


fn draw_text_center(
    d: &mut RaylibDrawHandle,
    text: &str,
    y: i32,
    font_size: i32,
    color: Color
 ) {
    let text_length = measure_text(text, font_size);
    d.draw_text(
        text,
        (SCREEN_WIDTH as i32) / 2 - (text_length / 2),
        y,
        font_size,
        color
    )
 }