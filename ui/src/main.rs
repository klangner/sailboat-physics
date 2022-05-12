// Basic boat visualization
//

mod mqh;

use std::f32::consts::PI;
use macroquad::prelude::*;
use sailboat_physics::{Sailboat, aerodynamics};
use sailboat_physics::geometry::Vec2d;


const WINDOW_WIDTH: i32 = 1024;
const WINDOW_HEIGHT: i32 = 800;

#[derive(PartialEq, Copy, Clone, Debug)]
enum View {
    MainView,
    AparentWindView,
    LiftAndDragView,
}

fn boat_shape() -> Vec<Vec2d> {
    vec![
        Vec2d::new(-20., 80.), 
        Vec2d::new(20., 80.), 
        Vec2d::new(30., 10.), 
        Vec2d::new(0., -100.),
        Vec2d::new(-30., 10.),
    ] 
}


// Print vector info on the screen
fn print_vector_info(name: &str, v: &Vec2d, x: f32, y: f32, color: Color) {
    let phi = (v.phi() / PI * 180.0).round() as i32;
    let r = v.r();
    let text = format!("{name}: r={r:.2}, dir={phi}");
    draw_text(&text, x, y, 25.0, color);
}

// Draw vector (Convert coords)
fn draw_vector(x: f32, y: f32, v: &Vec2d, color: Color){
    mqh::draw_arrow(x, y, v.y, -v.x, 3.0, color)
}

// Draw Wind widget
fn draw_wind_widget(wind: &Vec2d) {
    let phi = (wind.phi() / PI * 180.0).round() as i32;
    let r = wind.r();
    draw_text(&format!("Wind: {r:.2}m/s"), 850.0, 30.0, 30.0, DARKGRAY);
    draw_circle(940.0, 100.0, 50.0, LIGHTGRAY);
    let arr = Vec2d::from_polar(45.0, wind.phi());
    draw_vector(940.0, 100.0, &arr,DARKBLUE);
    draw_text(&format!("{phi}deg"), 900.0, 180.0, 30.0, DARKGRAY);
}

// Draw boat at the center of the screen
fn draw_boat(boat: &Sailboat) {
    let cx = WINDOW_WIDTH as f32/2.0;
    let cy = WINDOW_HEIGHT as f32/2.0;
    let shape = boat_shape();
    mqh::draw_shape(cx, cy, &shape, boat.velocity.phi(), 2., WHITE);
    // draw sail
    let sail_vec = Vec2d::from_polar(130.0, boat.sail_angle + PI);
    let sail_from = Vec2d::new(0., -60.0).rotate(boat.velocity.phi());
    let sail_to = Vec2d::new(sail_vec.y, -60.0-sail_vec.x).rotate(boat.velocity.phi());
    draw_line(cx+sail_from.x, cy+sail_from.y, cx+sail_to.x, cy+sail_to.y, 3.0, WHITE);
    // draw_line(cx, cy, cx+sail_vec.y, cy-sail_vec.x, 3.0, WHITE);
}

// Main view
fn main_view(boat: &Sailboat, lift: &Vec2d, drag: &Vec2d, aw: &Vec2d) {
    let total_force = lift.add(drag);
    let bv = boat.velocity.multiply(50.0);
    let t = total_force.multiply(10.0);
    let a = aw.multiply(50.0);
    let cx = WINDOW_WIDTH as f32/2.0;
    let cy = WINDOW_HEIGHT as f32/2.0;

    print_vector_info("Boat velocity", &aw, 20.0, 30.0, LIGHTGRAY);
    print_vector_info("Apparent wind", &aw, 20.0, 55.0, DARKBLUE);
    print_vector_info("Total force", &t, 20.0, 80.0, LIME);
    draw_vector(cx, cy, &bv, LIGHTGRAY);
    draw_vector(cx, cy, &a, DARKBLUE);
    draw_vector(cx, cy, &t, LIME);
}


// draw vectors for verification of apparent wind
fn apparent_wind_view(boat: &Sailboat, wind: &Vec2d, aw: &Vec2d) {
    let bv = boat.velocity.multiply(50.0);
    let h = boat.velocity.neg().multiply(50.0);
    let w = wind.multiply(50.0);
    let a = aw.multiply(50.0);
    let cx = WINDOW_WIDTH as f32/2.0;
    let cy = WINDOW_HEIGHT as f32/2.0;

    print_vector_info("Boat velocity", &aw, 20.0, 30.0, LIGHTGRAY);
    print_vector_info("Head wind", &aw, 20.0, 55.0, BLUE);
    print_vector_info("Apparent wind", &aw, 20.0, 80.0, DARKBLUE);
    draw_vector(cx, cy, &bv, LIGHTGRAY);
    draw_vector(cx, cy, &h, BLUE);
    draw_vector(cx, cy, &w, BLUE);
    draw_vector(cx, cy, &a, DARKBLUE);
}

// Visualize lift and drag
fn liftanddrag_view(lift: &Vec2d, drag: &Vec2d, aw: &Vec2d) {
    let total_force = lift.add(drag);
    let l = lift.multiply(10.0);
    let d = drag.multiply(10.0);
    let t = total_force.multiply(10.0);
    let a = aw.multiply(50.0);
    let cx = WINDOW_WIDTH as f32/2.0;
    let cy = WINDOW_HEIGHT as f32/2.0;

    print_vector_info("Apparent wind", &aw, 20.0, 30.0, DARKBLUE);
    print_vector_info("total force", &t, 20.0, 55.0, LIME);
    draw_vector(cx, cy, &a, DARKBLUE);
    draw_vector(cx, cy, &l, GREEN);
    draw_vector(cx, cy, &d, GREEN);
    draw_vector(cx, cy, &t, LIME);
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
    let mut wind = Vec2d::from_polar(3.0, -PI/2.0);
    let mut boat = Sailboat::new(Vec2d::from_polar(3.0, 0.0), PI/16.0);

    let mut mode: View = View::MainView;

    loop {

        let dt = get_frame_time();

        // Process input
        #[cfg(not(target_arch = "wasm32"))]
        if is_key_down(KeyCode::Q) | is_key_down(KeyCode::Escape) {
            break;
        }
        // change mode
        if is_key_released(KeyCode::Key1) {
            mode = View::MainView
        }
        else if is_key_released(KeyCode::Key2) {
            mode = View::AparentWindView
        }
        else if is_key_released(KeyCode::Key3) {
            mode = View::LiftAndDragView
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
            boat = boat.turn(dt / 5.0)
        }
        if is_key_down(KeyCode::A) {
            boat = boat.turn(-dt / 5.0)
        }
        if is_key_down(KeyCode::W) {
            boat = boat.push(dt)
        }
        if is_key_down(KeyCode::S) {
            boat = boat.push(-dt)
        }
        // Sail
        if is_key_down(KeyCode::LeftBracket) {
            boat = boat.rotate_sail(dt / 10.0)
        }
        if is_key_down(KeyCode::RightBracket) {
            boat = boat.rotate_sail(-dt / 10.0)
        }

        // Update state
        let aw = boat.apparent_wind(&wind);
        let lift = aerodynamics::lift(&aw, 1.0, boat.velocity.phi()+boat.sail_angle);
        let drag = aerodynamics::drag(&aw, 1.0, boat.velocity.phi()+boat.sail_angle);

        // Draw 
        clear_background(SKYBLUE);
        draw_wind_widget(&wind);
        draw_boat(&boat);
        if mode == View::MainView {
            main_view(&boat, &lift, &drag, &aw);
        } else if mode == View::AparentWindView {
            apparent_wind_view(&boat, &wind, &aw);
        } else if mode == View::LiftAndDragView {
            liftanddrag_view(&lift, &drag, &aw);
        }

        next_frame().await
    }
}