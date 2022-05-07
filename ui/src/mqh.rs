//! Helper functions for drawing shapes in microquad apps

use macroquad::prelude::*;
use sailboat_physics::goemetry::Vec2d;


// draw arrow
pub fn draw_arrow(x: f32, y: f32, dx: f32, dy: f32, thickness: f32, color: Color) {
    let ax = 0.1 * dx;
    let ay = 0.1 * dy;
    draw_line(x, y, x + dx, y + dy, 3.0, color);
    draw_line( x + dx, y + dy, x + 0.9*dx+ay, y + 0.9*dy-ax,  thickness, color);
    draw_line( x + dx, y + dy, x + 0.9*dx-ay, y + 0.9*dy+ax,  thickness, color);
}

/// draw shape based on the given set of vetices
pub fn draw_shape(cx: f32, cy: f32, vertices: &Vec<Vec2d>, phi: f32, thickness: f32, color: Color) {
    let shape:Vec<Vec2d> = vertices.iter().map(|v| v.rotate(phi)).collect();
    for i in 0..vertices.len() {
        if i < vertices.len() - 1 {
            draw_line(shape[i].x+cx, shape[i].y+cy, shape[i+1].x+cx, shape[i+1].y+cy, thickness, color);
        } else {
            draw_line(shape[i].x+cx, shape[i].y+cy, shape[0].x+cx, shape[0].y+cy, thickness, color);

        }
    }
}