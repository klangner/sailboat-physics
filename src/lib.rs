//! All angles in radians with 0 pointing North
//! 

pub mod vector;

// use vector::Vec2d;


pub struct Sail {
}

pub struct Sailboat {
}

pub struct SailboatState {
}


impl Sailboat {
    pub fn new() -> Sailboat {
        Sailboat { }
    }
}


// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
