//! 2d Vector class
//! 

use std::f32::consts::PI;


#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}


impl Vec2d {

    /// Construct a new Vector
    pub fn new(x: f32, y: f32) -> Vec2d {
        Vec2d {x, y}
    }

    /// Construct a new Vector from polar coordinates
    pub fn from_polar(r: f32, phi: f32) -> Vec2d {
        let x = r * f32::cos(phi);
        let y = r * f32::sin(phi);
        Vec2d::new(x, y)
    }

    /// Vector inverse
    pub fn neg(&self) -> Vec2d {
        Vec2d::new(-self.x, -self.y)
    }

    /// Add 2 vectors
    pub fn add(&self, u: &Vec2d) -> Vec2d {
        Vec2d::new(self.x+u.x, self.y+u.y)
    }

    /// Dot product between 2 vectors
    pub fn dot(&self, other: &Self) -> f32 {
        self.x*other.x + self.y*other.y
    }

    /// Vector norm (Magnitude)
    pub fn norm(&self) -> f32 {
        f32::sqrt(self.dot(self))
    }

    /// Angle in radians between 2 vectors
    pub fn angle(&self, other: &Self) -> f32 {
        let alpha = self.dir() - other.dir();
        if alpha > PI {
            alpha - 2.0*PI
        } else if alpha < -PI {
            alpha + 2.0*PI
        } else {
            alpha
        }
    }

    /// Vector direction. 0,1 == 0
    pub fn dir(&self) -> f32 {
        self.y.atan2(self.x)
    }
}


// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::goemetry::Vec2d;
    use std::f32::consts::PI;
    use approx::{assert_abs_diff_eq, AbsDiffEq};

    impl AbsDiffEq for Vec2d {
        type Epsilon = f32;

        fn default_epsilon() -> Self::Epsilon {
            f32::default_epsilon()
        }

        fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
            self.x.abs_diff_eq(&other.x, epsilon) &&
            self.y.abs_diff_eq(&other.y, epsilon)
        }
    }

    #[test]
    fn from_polar() {
        assert_abs_diff_eq!(Vec2d::from_polar(1., 0.), Vec2d::new(1., 0.));
        assert_abs_diff_eq!(Vec2d::from_polar(1., PI/2.), Vec2d::new(0., 1.));
        assert_abs_diff_eq!(Vec2d::from_polar(1., PI), Vec2d::new(-1., 0.));
        assert_abs_diff_eq!(Vec2d::from_polar(1., 3.*PI/2.), Vec2d::new(0., -1.));
    }
    
    #[test]
    fn inverse() {
        let v = Vec2d::new(5., 4.);
        assert_eq!(v.neg(), Vec2d::new(-5., -4.));
    }
    
    #[test]
    fn add() {
        let v = Vec2d::new(5., 4.);
        let u = Vec2d::new(2., -8.);
        assert_eq!(v.add(&u), Vec2d::new(7., -4.));
    }
    
    #[test]
    fn dot() {
        let v = Vec2d::new(5., 4.);
        let u = Vec2d::new(2., -8.);
        assert_eq!(v.dot(&u), -22.);
    }

    #[test]
    fn norm() {
        let v = Vec2d::new(3., 4.);
        assert_eq!(v.norm(), 5.);
    }

    #[test]
    fn test_positive_angle() {
        let v = Vec2d::new(3., 3.);
        let u = Vec2d::new(1., 0.);
        assert_eq!(v.angle(&u), PI/4.);
    }

    #[test]
    fn test_negative_angle() {
        let v = Vec2d::from_polar(1., PI/2.0);
        let u = Vec2d::from_polar(1., 3.0*PI/4.0);
        assert_abs_diff_eq!(v.angle(&u), -PI/4.);
    }

    #[test]
    fn test_angle_overflow() {
        let v = Vec2d::from_polar(1., 7.0*PI/8.0);
        let u = Vec2d::from_polar(1., -7.0*PI/8.0);
        assert_abs_diff_eq!(v.angle(&u), -PI/4., epsilon=0.01);
        assert_abs_diff_eq!(u.angle(&v), PI/4.0, epsilon=0.01);
    }

    #[test]
    fn test_dir() {
        let v = Vec2d::from_polar(1., 7.0*PI/8.0);
        let u = Vec2d::from_polar(1., -7.0*PI/8.0);
        assert_abs_diff_eq!(v.dir(), 7.0*PI/8.0);
        assert_abs_diff_eq!(u.dir(), -7.0*PI/8.0);
    }
}
