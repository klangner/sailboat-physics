/// Aerodynamic implementation for the sail

use std::f32::consts::PI;
use crate::goemetry::Vec2d;


const AIR_DENSITY: f32 = 1.;
const LIFT_COEFF:[f32; 11] = [0.0, 0.1, 1.3, 1.5, 1.4, 1.3, 1., 0.8, 0.4, 0.3, 0.15];
// const DRAG_COEFF:[f32; 11] = [0.01, 0.15, 0.18, 0.22, 0.3, 0.4, 0.6, 0.75, 0.9, 1.2, 1.6];


/// Use empirical table to calculate force coefficient. Angle of attack is in radians
fn airfoil_coefficient(angle_of_attack: f32, table: &[f32; 11]) -> f32 {
    let angle = angle_of_attack / PI * 180.0;
    if angle > 99.0 || angle < 0.0 { return 0.}

    let idx = (angle / 10.).floor() as usize;
    let a = table[idx];
    let b = table[idx+1];
    a + (b-a) * (angle / 10.).fract()
}


/// Lift
/// Lift = 1/2 * ro * Va^2 * A * Cl
/// where:
///     ro - air density 
///     Va - Apparent wind speed
///     A  - Sail area 
///     Cl - Lift coefficient for specific angle of attack
pub fn lift(wind: Vec2d, sail_area: f32, sail_angle: f32) -> Vec2d {
    let sail_vec = Vec2d::from_polar(sail_angle, 1.);
    let wind_speed = wind.r();
    let aoa = wind.neg().angle(&sail_vec);
    let cl = airfoil_coefficient(aoa.abs(), &LIFT_COEFF);
    print!("{:?}, {:?}", aoa.abs(), cl);

    let lm = 0.5 * AIR_DENSITY * wind_speed*wind_speed * sail_area * cl;
    let phi = if aoa > 0.0 { wind.phi() + PI/2.0 } else { wind.phi() - PI/2.0 };
    Vec2d::from_polar(lm, phi)
}

// ----------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use approx::assert_abs_diff_eq;
    use super::*;


    #[test]
    fn lift_coefficient() {
        assert_abs_diff_eq!(airfoil_coefficient(0., &LIFT_COEFF), 0.);
        assert_abs_diff_eq!(airfoil_coefficient(PI, &LIFT_COEFF), 0.);
        assert_abs_diff_eq!(airfoil_coefficient(PI/4., &LIFT_COEFF), 1.35);
    }

    #[test]
    fn lift_1() {
        let l = lift(Vec2d::from_polar(1., -PI), 1., PI/4.); 
        assert_abs_diff_eq!(l.phi(), PI/2.0, epsilon=0.001);
    }

    #[test]
    fn lift_2() {
        let l = lift(Vec2d::from_polar(1., -3.0*PI/4.0), 1., 0.0); 
        assert_abs_diff_eq!(l.phi(), -PI/4.0, epsilon=0.001);
    }

}