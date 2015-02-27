use std::any::Any;
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::mem;

use super::user_data::UserData;

use chip;

struct BodyRaw {
    cp_body: chip::cpBody,
    user_data: Option<Box<Any>>
}

pub struct Body {
    raw: Rc<UnsafeCell<BodyRaw>>
}

impl Body {
    fn new(mass: f64, moment: f64) -> Body {
        Body {
            raw: Rc::new(UnsafeCell::new(BodyRaw::new(mass, moment)))
        }
    }

    forward!(set_angle_rad(&mut self, angle: f64) -> (),
    /// Sets the angle of the object in space (in radians).
    );

    forward!(set_angle_deg(&mut self, angle: f64) -> (),
    /// Sets the angle of the object in space (in degrees).
    );

    forward!(set_angular_velocity_rad(&mut self, ang_vel: f64) -> (),
    /// Sets the angular velocity in radians / second.
    ) ;

    forward!(set_angular_velocity_deg(&mut self, ang_vel: f64) -> (),
    /// Sets the angular velocity in degrees / second.
    ) ;
}

impl BodyRaw {
    fn new(mass: f64, moment: f64) -> BodyRaw {
        unsafe {
            let mut ret = BodyRaw {
                cp_body: mem::uninitialized(),
                user_data: None
            };
            chip::cpBodyInit(&mut ret.cp_body, mass, moment);
            ret
        }
    }

    fn set_angle_rad(&mut self, angle: f64) {
        unsafe {
            chip::cpBodySetAngle(&mut self.cp_body, angle);
        }
    }

    fn set_angle_deg(&mut self, angle: f64) {
        use std::f64::consts::PI;
        self.set_angle_rad(angle * (180.0 / PI));
    }

    fn set_angular_velocity_rad(&mut self, ang_vel: f64) {
        unsafe {
            chip::cpBodySetAngularVelocity(&mut self.cp_body,
                                           ang_vel)
        }
    }

    fn set_angular_velocity_deg(&mut self, ang_vel: f64) {
        use std::f64::consts::PI;
        unsafe {
            chip::cpBodySetAngularVelocity(&mut self.cp_body,
                                           ang_vel * (180.0 / PI))
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

impl UserData for BodyRaw {
    fn get_userdata_box(&self) -> &Option<Box<Any>> {
        &self.user_data
    }
    fn get_userdata_mut_box(&mut self) -> &mut Option<Box<Any>> {
        &mut self.user_data
    }
}
