//! All angles in radians with 0 pointing North
//! 

pub mod goemetry;
pub mod aerodynamics;

use goemetry::Vec2d;
use std::f32::consts::PI;


/// Wind vectors have x coordinate pointing North
pub struct Wind {
    pub velocity: Vec2d,
}

pub struct Sailboat {
    pub velocity: Vec2d,
    pub sail_angle: f32,
}


impl Wind {
    pub fn new(velocity: Vec2d) -> Wind {
        Wind {velocity}
    }

    /// Wind direction in degres 0ᵒ = North, 90ᵒ = East
    pub fn direction(&self) -> f32 {
        let phi =  self.velocity.phi();
        let alpha = phi / PI * 180.0;
        alpha
    }

    pub fn speed(&self) -> f32 {
        self.velocity.r()
    }
}

impl Sailboat {
    pub fn new(velocity: Vec2d, sail_angle: f32) -> Sailboat {
        Sailboat {velocity, sail_angle}
    }

    // Apparent wind on the boat
    pub fn apparent_wind(&self, wind: &Vec2d) -> Vec2d {
        self.velocity.neg().add(wind)
    }

    pub fn turn(&self, phi: f32) -> Sailboat {
        let v = self.velocity.rotate(phi);
        Sailboat { velocity: v, sail_angle: self.sail_angle }
    }

    pub fn push(&self, amount: f32) -> Sailboat {
        let v = self.velocity.increase(amount);
        Sailboat { velocity: v, sail_angle: self.sail_angle }
    }
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn wind_dir() {
        assert_abs_diff_eq!(Wind::new(Vec2d::new(1., 0.)).direction(), 0.0);
        assert_abs_diff_eq!(Wind::new(Vec2d::new(3., 3.)).direction(), 45.0);
        assert_abs_diff_eq!(Wind::new(Vec2d::new(-3., -3.)).direction(), -135.0);
    }
}
