use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use chip;

struct BodyRaw {
    cp_body: chip::cpBody,
    user_data: Option<Box<Any>>
}

pub struct Body {
    raw: Rc<RefCell<BodyRaw>>
}

impl BodyRaw {
    fn set_angle_rad(&mut self, angle: f64) {
        unsafe {
            chip::cpBodySetAngle(&mut self.cp_body, angle);
        }
    }

    fn set_angle_deg(&mut self, angle: f64) {
        use std::f64::consts::PI;
        self.set_angle_rad(angle * (180.0 / PI));
    }

    fn set_angular_velocity(&mut self, ang_vel: f64) {
        unsafe {
            chip::cpBodySetAngularVelocity(&mut self.cp_body,
                                           ang_vel)
        }
    }

    fn set_center_of_gravity(&mut self, x: f64, y: f64) {
        unsafe {
            chip::cpBodySetCenterOfGravity(&mut self.cp_body, chip::cpv(x, y));
        }
    }

    fn set_force(&mut self, x: f64, y: f64) {
        unsafe {
            chip::cpBodySetForce(&mut self.cp_body, chip::cpv(x, y));
        }
    }

    fn set_mass(&mut self, mass: f64) {
        unsafe {
            chip::cpBodySetMass(&mut self.cp_body, mass);
        }
    }

    fn set_moment(&mut self, moment: f64) {
        unsafe {
            chip::cpBodySetMoment(&mut self.cp_body, moment);
        }
    }

    fn set_position(&mut self, x: f64, y: f64) {
        unsafe {
            chip::cpBodySetPosition(&mut self.cp_body, chip::cpv(x, y));
        }
    }

    fn set_torque(&mut self, torque: f64) {
        unsafe {
            chip::cpBodySetTorque(&mut self.cp_body, torque);
        }
    }

    fn set_velocity(&mut self, vx: f64, vy: f64) {
        unsafe {
            chip::cpBodySetVelocity(&mut self.cp_body, chip::cpv(vx, vy));
        }
    }

    // fn set_type
    // fn setPositionUpdateFunc
}
