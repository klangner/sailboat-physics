//! 2d Vector class
//! 


#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec2d {
    pub x: f32,
    pub y: f32,
}


impl Vec2d {

    /// Construct a new Vector
    pub fn new (x: f32, y: f32) -> Vec2d {
        Vec2d {x, y}
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
        f32::acos(self.dot(other) / (self.norm()*other.norm()))
    }
}


// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::vector::Vec2d;
    use std::f32::consts::PI;

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
    fn angle() {
        let v = Vec2d::new(3., 3.);
        let u = Vec2d::new(-4., 4.);
        assert_eq!(v.angle(&u), PI/2.);
    }
}
