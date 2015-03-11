use std::any::Any;
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::mem;
use std::marker::PhantomData;

use super::user_data::UserData;

use chip;

use void::Void;

struct BodyRaw<T> {
    cp_body: chip::cpBody,
    user_data: Option<Box<Any>>,
    _phantom: PhantomData<T>
}

pub struct Body<T> {
    raw: Rc<UnsafeCell<BodyRaw<T>>>
}

impl Body<Void> {
    pub fn new(mass: f64, moment: f64) -> Body<Void> {
        Body {
            raw: Rc::new(UnsafeCell::new(BodyRaw::new(mass, moment)))
        }
    }

    pub fn new_kinematic() -> Body<Void> {
        Body {
            raw: Rc::new(UnsafeCell::new(BodyRaw::new_kinematic()))
        }
    }

    pub fn new_static() -> Body<Void> {
        Body {
            raw: Rc::new(UnsafeCell::new(BodyRaw::new_static()))
        }
    }
}

impl <T> Body<T> {

    // TODO: hide doc
    pub unsafe fn duplicate(&mut self) -> Body<Void> {
        use std::mem::transmute;
        transmute( Body { raw: self.raw.clone() })
    }

    // TODO: hide doc
    pub unsafe fn get_cp_body(&mut self) -> *mut chip::cpBody {
        &mut (*self.raw.get()).cp_body
    }

    forward!(angle_rad(&self) -> f64,
    /// Returns the rotation angle of the body in radians.
    );

    forward!(angle_deg(&self) -> f64,
    /// Returns the rotation angle fo the body in degrees.
    );

    forward!(angular_velocity_rad(&self) -> f64,
    /// Returns the angular velocity in radians / second.
    );

    forward!(angular_velocity_deg(&self) -> f64,
    /// Returns the angular velocity in degrees / second.
    );

    forward!(center_of_gravity(&self) -> (f64, f64),
    /// Returns the location of the center of gravity in
    /// local coordinates.
    );

    forward!(force(&self) -> (f64, f64),
    /// Returns the force acting on the body.
    );

    forward!(mass(&self) -> f64,
    /// Returns the mass of the body.
    );

    forward!(moment(&self) -> f64,
    /// Returns the moment of inertia of the body.
    );

    forward!(position(&self) -> (f64, f64),
    /// Returns the position of the body in world space.
    );

    forward!(torque(&self) -> f64,
    /// Returns the torque acting on the body.
    );

    forward!(velocity(&self) -> (f64, f64),
    /// Returns the velocity of the body.
    );


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

    forward!(set_center_of_gravity(&mut self, x: f64, y: f64) -> (),
    /// Sets the position of the center of gravity on this body.
    ///
    /// The center of gravity is in local coordinates.
    );

    forward!(set_force(&mut self, x: f64, y: f64) -> (),
    /// Sets the force applied to the body.
    ///
    /// The force is not reset during each physics step.  If you want
    /// to reset the force, you must do that manually.
    );

    forward!(set_mass(&mut self, mass: f64) -> (),
    /// Sets the mass of the body.
    );

    forward!(set_moment(&mut self, moment: f64) -> (),
    /// Sets the moment of inertia of the body.
    ///
    /// The moment of inertia is the rotational mass of the body.
    );

    forward!(set_position(&mut self, x: f64, y: f64) -> (),
    /// Sets the position of the body in world coordinates.
    );

    forward!(set_torque(&mut self, torque: f64) -> (),
    /// Sets the torque applied to the body.
    );

    forward!(set_velocity(&mut self, x: f64, y: f64) -> (),
    /// Directly sets the velocity of the body.
    );
}

impl <T> BodyRaw<T> {
    fn new(mass: f64, moment: f64) -> BodyRaw<T> {
        unsafe {
            let mut ret = BodyRaw {
                cp_body: mem::zeroed(),
                user_data: None,
                _phantom: PhantomData
            };
            chip::cpBodyInit(&mut ret.cp_body, mass, moment);
            ret
        }
    }

    fn new_kinematic() -> BodyRaw<T> {
        let mut res = BodyRaw::new(0.0, 0.0);
        unsafe {
            chip::cpBodySetType(&mut res.cp_body, chip::CP_BODY_TYPE_KINEMATIC);
        }
        res
    }

    fn new_static() -> BodyRaw<T> {
        let mut res = BodyRaw::new(0.0, 0.0);
        unsafe {
            chip::cpBodySetType(&mut res.cp_body, chip::CP_BODY_TYPE_STATIC);
        }
        res
    }

    fn angle_rad(&self) -> f64 {
        unsafe {
            chip::cpBodyGetAngle(&self.cp_body)
        }
    }

    fn angle_deg(&self) -> f64 {
        use std::f64::consts::PI;
        self.angle_rad() * (PI / 180.0)
    }

    fn angular_velocity_rad(&self) -> f64 {
        unsafe {
            chip::cpBodyGetAngularVelocity(&self.cp_body)
        }
    }

    fn angular_velocity_deg(&self) -> f64 {
        use std::f64::consts::PI;
        self.angular_velocity_rad() * (PI / 180.0)
    }

    fn center_of_gravity(&self) -> (f64, f64) {
        unsafe {
            let v = chip::cpBodyGetCenterOfGravity(&self.cp_body);
            (v.x, v.y)
        }
    }

    fn force(&self) -> (f64, f64) {
        unsafe {
            let v = chip::cpBodyGetForce(&self.cp_body);
            (v.x, v.y)
        }
    }

    fn mass(&self) -> f64 {
        unsafe {
            chip::cpBodyGetMass(&self.cp_body)
        }
    }

    fn moment(&self) -> f64 {
        unsafe {
            chip::cpBodyGetMoment(&self.cp_body)
        }
    }

    fn position(&self) -> (f64, f64) {
        unsafe {
            let v = chip::cpBodyGetPosition(&self.cp_body);
            (v.x, v.y)
        }
    }

    fn torque(&self) -> f64 {
        unsafe {
            chip::cpBodyGetTorque(&self.cp_body)
        }
    }

    fn velocity(&self) -> (f64, f64) {
        unsafe {
            let v = chip::cpBodyGetVelocity(&self.cp_body);
            (v.x, v.y)
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

impl <T: 'static> UserData<T> for BodyRaw<T> {
    fn get_userdata_box(&self) -> &Option<Box<Any>> {
        &self.user_data
    }
    fn get_userdata_mut_box(&mut self) -> &mut Option<Box<Any>> {
        &mut self.user_data
    }
}

#[unsafe_destructor]
impl <T> Drop for BodyRaw<T> {
    fn drop(&mut self) {
        unsafe {
            chip::cpBodyDestroy(&mut self.cp_body);
        }
    }
}
