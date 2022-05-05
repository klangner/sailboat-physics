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
    let text = format!("{name}: r={r:.2}, dir={phi}");
    draw_text(&text, 20.0, pos, 25.0, DARKGRAY);
}

fn draw_arrow(x: f32, y: f32, dx: f32, dy: f32, color: Color) {
    let ax = 0.1 * dx;
    let ay = 0.1 * dy;
    draw_line(x, y, x + dx, y + dy, 3.0, color);
    draw_line( x + dx, y + dy, x + 0.9*dx+ay, y + 0.9*dy-ax,  3.0, color);
    draw_line( x + dx, y + dy, x + 0.9*dx-ay, y + 0.9*dy+ax,  3.0, color);
}

fn draw_vector(x: f32, y: f32, v: &Vec2d, color: Color){
    draw_arrow(x, y, v.y, -v.x, color)
}


fn draw_wind_widget(wind: &Vec2d) {
    let phi = (wind.phi() / PI * 180.0).round() as i32;
    let r = wind.r();
    draw_text(&format!("Wind: {r:.2}m/s"), 850.0, 30.0, 30.0, DARKGRAY);
    draw_circle(940.0, 100.0, 50.0, LIGHTGRAY);
    let arr = Vec2d::from_polar(45.0, wind.phi());
    draw_vector(940.0, 100.0, &arr,DARKBLUE);
    draw_text(&format!("{phi}deg"), 900.0, 180.0, 30.0, DARKGRAY);
}


fn draw_boat(boat: &Vec2d) {
    let bv = Vec2d::from_polar(50.0*boat.r(), boat.phi());
    let cx = WINDOW_WIDTH as f32/2.0;
    let cy = WINDOW_HEIGHT as f32/2.0;
    draw_vector(cx, cy, &bv, DARKGRAY);
    print_vector_info("Boat", &boat, 45.0);
    // print_vector_info("Sail", &sail, 70.0);
}


fn draw_apparent_wind(boat_velocity: &Vec2d, wind: &Vec2d, aw: &Vec2d) {
    let h = Vec2d::from_polar(50.0*boat_velocity.r(), boat_velocity.neg().phi());
    let w = Vec2d::from_polar(50.0*wind.r(), wind.phi());
    let a = Vec2d::from_polar(50.0*aw.r(), aw.phi());
    let cx = WINDOW_WIDTH as f32/2.0;
    let cy = WINDOW_HEIGHT as f32/2.0;

    print_vector_info("Apparent wind", &aw, 70.0);
    draw_vector(cx, cy, &h, LIGHTGRAY);
    draw_vector(cx, cy, &w, BLUE);
    draw_vector(cx, cy, &a, RED);
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
            wind = wind.rotate(-dt / 5.0)
        }
        if is_key_down(KeyCode::Right) {
            wind = wind.rotate(dt / 5.0)
        }
        if is_key_down(KeyCode::Up) {
            wind = wind.increase(dt)
        }
        if is_key_down(KeyCode::Down) {
            wind = wind.increase(-dt)
        }
        // Boat
        if is_key_down(KeyCode::D) {
            boat = boat.rotate(dt / 5.0)
        }
        if is_key_down(KeyCode::A) {
            boat = boat.rotate(-dt / 5.0)
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
        draw_wind_widget(&wind);
        draw_boat(&boat);
        let aw = apparent_wind(&boat, &wind);
        draw_apparent_wind(&boat, &wind, &aw);

        next_frame().await
    }
}