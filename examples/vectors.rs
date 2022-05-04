// Basic boat visualization
//

use std::f32::consts::PI;
use macroquad::prelude::*;
use sailboat_physics::{goemetry::Vec2d, apparent_wind};


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;


fn print_vector_info(name: &str, v: &Vec2d, pos: f32) {
    let phi = (v.phi() / PI * 180.0).round() as i32;
    let r = v.r();
    let text = format!("{name}: r={r}, dir={phi}");
    draw_text(&text, 20.0, pos, 30.0, DARKGRAY);
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Sailboat".to_owned(),
        fullscreen: false,
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {

    // 5m/s from North
    let mut wind = Vec2d::from_polar(3.0, -3.0*PI/4.0);
    let mut boat = Vec2d::from_polar(3.0, 0.0);
    let mut sail = Vec2d::from_polar(1.0, PI/4.0);

    loop {

        let dt = get_frame_time();

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }
        // Wind
        if is_key_down(KeyCode::Left) {
            wind = wind.rotate(-dt / 10.0)
        }
        if is_key_down(KeyCode::Right) {
            wind = wind.rotate(dt / 10.0)
        }
        if is_key_down(KeyCode::Up) {
            wind = wind.increase(dt)
        }
        if is_key_down(KeyCode::Down) {
            wind = wind.increase(-dt)
        }
        // Boat
        if is_key_down(KeyCode::D) {
            boat = boat.rotate(dt / 10.0)
        }
        if is_key_down(KeyCode::A) {
            boat = boat.rotate(-dt / 10.0)
        }
        if is_key_down(KeyCode::W) {
            boat = boat.increase(dt)
        }
        if is_key_down(KeyCode::S) {
            boat = boat.increase(-dt)
        }
        // Sail
        if is_key_down(KeyCode::LeftBracket) {
            sail = sail.rotate(-dt / 10.0)
        }
        if is_key_down(KeyCode::RightBracket) {
            sail = sail.rotate(dt / 10.0)
        }

        // Draw 
        clear_background(SKYBLUE);
        // Draw vector info
        print_vector_info("Wind", &wind, 20.0);
        print_vector_info("Boat", &boat, 45.0);
        print_vector_info("Sail", &sail, 70.0);

        let apparent_wind = apparent_wind(&boat, &wind);
        // let lift = Vec2d::from_polar(1.0, 0.);
        // let drag = Vec2d::from_polar(1.0, 0.);

        print_vector_info("Apparent wind", &apparent_wind, 110.0);
        // print_vector_info("Lift", &lift, 135.0);
        // print_vector_info("Drag", &drag, 160.0);
        
        next_frame().await
    }
}