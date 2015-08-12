use chip;

pub fn moment_of_circle(mass: f64, r1: f64, r2: f64) -> f64 {
    unsafe {
        chip::cpMomentForCircle(mass, r1, r2, chip::cpv(0.0, 0.0))
    }
}
