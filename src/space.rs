use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use chip;

use super::user_data::UserData;


struct SpaceRaw {
    cp_space: chip::cpSpace,
    user_data: Option<Box<Any>>
}

pub struct Space {
    raw: Rc<RefCell<SpaceRaw>>
}

impl Space {
    pub fn new() -> Space {
        Space {
            raw: Rc::new(RefCell::new(SpaceRaw::new()))
        }
    }

    /// This function does not release the refcount on the
    /// Rc holding the Raw Space pointer.
    pub unsafe fn into_raw_ptr(self) -> *const () {
        mem::transmute(self)
    }

    pub unsafe fn from_raw_ptr(ptr: *const ()) -> Space {
        mem::transmute(ptr)
    }

    forward!(gravity(&self) -> (f64, f64),
    /// Returns the global gravity for all rigid bodies in this space.
    ///
    /// Default is `(0.0, 0.0)`.
    );

    forward!(damping(&self) -> f64,
    /// Returns the global velocity damping.
    ///
    /// Defaults to 1.0 (no damping).
    ///
    /// This value is the fraction of velocity a body should have after 1
    /// second.  A value of 0.9 would mean that each second, a body would have
    /// 90% of the velocity it had the previous second.
    );

    forward!(collision_slop(&self) -> f64,
    /// Returns the amount of encouraged penetration between colliding shapes.
    ///
    /// Defaults to 1.0
    ///
    /// This is used to reduce oscillating contacts and keep the collision cache
    /// warm.  Defaults to 1.0.
    ///
    /// If you have poor simulation quality, increase this number as much as
    /// possible without allowing visible amounts of overlap.
    );

    forward!(collision_bias(&self) -> f64,
    /// Returns how fast overlapping shapes are pushed apart.
    ///
    /// Defaults to `pow(1 - 0.1, 60)` meaning that chipmunk fixes 10% of
    /// overlap each frame at 60Hz.
    );

    forward!(collision_persistence(&self) -> u32,
    /// Returns the number of frames that contact information should remain.
    ///
    /// Defaults to 3.
    );

    forward!(idle_speed_threshold(&self) -> f64,
    /// Returns the minimum speed to be considered idle.
    /// Defaults to 0.0.
    );

    forward!(iterations(&self) -> i32,
    /// Gets the number of solver passes that the engine uses.
    ///
    /// Defaults to 10.
    ///
    /// Fewer iterations means less CPU usage, but lower quality physics.
    );

    forward!(sleep_time_threshold(&self) -> f64,
    /// Returns the ellapsed time before a group of idle bodies is put to sleep.
    ///
    /// Defaults to infinity (no sleeping).
    );


    forward!(set_gravity(&mut self, ax: f64, ay: f64) -> (),
    /// Sets the global gravity for all rigid bodies in this space.
    ///
    /// Default is `<0, 0>` (no gravity).
    );

    forward!(set_damping(&mut self, damping: f64) -> (),
    /// Sets the global velocity damping.
    ///
    /// Defaults to 1.0 (no damping).
    ///
    /// See `damping()` for a description of this property.
    );

    forward!(set_collision_slop(&mut self, slop: f64) -> (),
    /// Sets the amount of encouraged penetration between colliding shapes.
    ///
    /// See `collision_slop()` for a description of the property.
    );

    forward!(set_collision_bias(&mut self, bias: f64) -> (),
    /// Sets how fast overlapping shapes are pushed apart.
    ///
    /// Defaults to pow(1.0 - 0.1, 60) meaning that chipmunk fixes
    /// 10% of overlap each frame at 60Hz.
    );

    forward!(set_collision_persistence(&mut self, persistence: u32) -> (),
    /// Sets the number of frames that contact information should remain.
    ///
    /// Default is 3.
    );

    forward!(set_idle_speed_threshold(&mut self, threshold: f64) -> (),
    /// Sets the minimum speed to be considered idle.
    ///
    /// Default is 0.0.
    );

    forward!(set_iterations(&mut self, iterations: i32) -> (),
    /// Sets the number of solver passes that the engine uses.
    ///
    /// Default is 10.  Fewer iterations means less CPU usage but
    /// lower quality physics.
    );

    forward!(set_sleep_time_threshold(&mut self, threshold: f64) -> (),
    /// Sets the ellapsed time before a group of idle bodies is put to sleep.
    ///
    /// Unless this method is called, this property will default to infinity
    /// which disables sleeping.
    );

}

impl UserData for Space {
    fn get_box(&self) -> Option<&Box<Any>> {
        self.raw.borrow().get_box()
    }
    fn get_mut_box(&mut self) -> Option<&mut Box<Any>> {
        self.raw.borrow_mut().get_mut_box()
    }
    fn set_box(&mut self, data: Box<Any>) {
        self.raw.borrow_mut().set_box(data)
    }
}

impl SpaceRaw {
    fn new() -> SpaceRaw {
        unsafe {
            let mut spr = SpaceRaw {
                cp_space: mem::uninitialized(),
                user_data: None
            };
            chip::cpSpaceInit(&mut spr.cp_space);
            spr
        }
    }

    fn gravity(&self) -> (f64, f64) {
        unsafe {
            let vec = chip::cpSpaceGetGravity(&self.cp_space);
            (vec.x, vec.y)
        }
    }

    fn damping(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetDamping(&self.cp_space)
        }
    }

    fn collision_slop(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetCollisionSlop(&self.cp_space)
        }
    }

    fn collision_bias(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetCollisionBias(&self.cp_space)
        }
    }

    fn collision_persistence(&self) -> u32 {
        unsafe {
            chip::cpSpaceGetCollisionPersistence(&self.cp_space)
        }
    }

    fn idle_speed_threshold(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetIdleSpeedThreshold(&self.cp_space)
        }
    }

    fn iterations(&self) -> i32 {
        unsafe {
            chip::cpSpaceGetIterations(&self.cp_space)
        }
    }

    fn sleep_time_threshold(&self) -> f64 {
        unsafe {
            chip::cpSpaceGetSleepTimeThreshold(&self.cp_space)
        }
    }

    fn set_gravity(&mut self, ax: f64, ay: f64) {
        unsafe {
            chip::cpSpaceSetGravity(&mut self.cp_space, chip::cpv(ax, ay));
        }
    }

    fn set_damping(&mut self, damping: f64) {
        unsafe {
            chip::cpSpaceSetDamping(&mut self.cp_space, damping);
        }
    }

    fn set_collision_slop(&mut self, slop: f64) {
        unsafe {
            chip::cpSpaceSetCollisionSlop(&mut self.cp_space, slop);
        }
    }

    fn set_collision_bias(&mut self, bias: f64) {
        unsafe {
            chip::cpSpaceSetCollisionBias(&mut self.cp_space, bias);
        }
    }

    fn set_collision_persistence(&mut self, persistence: u32) {
        unsafe {
            chip::cpSpaceSetCollisionPersistence(&mut self.cp_space, persistence);
        }
    }

    fn set_idle_speed_threshold(&mut self, threshold: f64) {
        unsafe {
            chip::cpSpaceSetIdleSpeedThreshold(&mut self.cp_space, threshold);
        }
    }

    fn set_iterations(&mut self, iterations: i32) {
        unsafe {
            chip::cpSpaceSetIterations(&mut self.cp_space, iterations);
        }
    }

    fn set_sleep_time_threshold(&mut self, threshold: f64) {
        unsafe {
            chip::cpSpaceSetSleepTimeThreshold(&mut self.cp_space, threshold);
        }
    }
}

impl Drop for SpaceRaw {
    fn drop(&mut self) {
        // TODO: destroy all bodies and constraints that are attached to this.
        unsafe {
            chip::cpSpaceDestroy(&mut self.cp_space);
        }
    }
}

impl UserData for SpaceRaw {
    fn get_box(&self) -> Option<&Box<Any>> {
        self.user_data.as_ref()
    }
    fn get_mut_box(&mut self) -> Option<&mut Box<Any>> {
        self.user_data.as_mut()
    }
    fn set_box(&mut self, data: Box<Any>) {
        self.user_data = Some(data);
    }
}
