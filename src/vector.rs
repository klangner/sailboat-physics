//! 2d Vector class
//! 


#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Vec2d {
    x: f32,
    y: f32,
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

// Base.:(+)(v::Vector2d, u::Vector2d) = Vector2d(v.x + u.x, v.y + u.y)
// Base.:(-)(v::Vector2d, u::Vector2d) = Vector2d(v.x - u.x, v.y - u.y)

// norm(v::Vector2d) = √(v.x*v.x + v.y*v.y)
// dot(v::Vector2d, u::Vector2d) = v.x*u.x + v.y*u.y 
// angle(v::Vector2d) = acos(dot(v, Vector2d(0, 1)) / norm(v))
// rad2deg(angle) = angle / (2*π) * 360

}


// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use crate::vector::Vec2d;

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
}
