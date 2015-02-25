use std::any::Any;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use chip;

macro_rules! forward {
    // &self
    ($name:ident (&self, $($arg:ident : $typ:ty),*) -> $ret:ty, $(#[$doc:meta])*) => {
        pub fn $name (&self, $($arg : $typ),*) -> $ret {
            $(#![$doc])*
            self.raw.borrow().$name($($arg),*)
        }
    };
    ($name:ident (&self) -> $ret:ty, $(#[$doc:meta])*) => {
        pub fn $name (&self) -> $ret {
            $(#![$doc])*
            self.raw.borrow().$name()
        }
    };

    // &mut self
    ($name:ident (&mut self, $($arg:ident : $typ:ty),*) -> $ret:ty, $(#[$doc:meta])*) => {
        pub fn $name (&mut self, $($arg : $typ),*) -> $ret {
            $(#![$doc])*
            self.raw.borrow_mut().$name($($arg),*)
        }
    };
    ($name:ident (&mut self) -> $ret:ty, $(#[$doc:meta])*) => {
        pub fn $name (&mut self) -> $ret {
            $(#![$doc])*
            self.raw.borrow_mut().$name()
        }
    };
}

struct SpaceRaw {
    cp_space: chip::cpSpace,
    user_data: Option<Box<Any>>
}

#[derive(Clone)]
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
    /// This value is the fraction of velocity a body should have after 1
    /// second.  A value of 0.9 would mean that each second, a body would have
    /// 90% of the velocity it had the previous second.
    );

    forward!(set_collision_slop(&mut self, slop: f64) -> (),
    /// Sets the amount of encouraged penetration between colliding shapes.
    ///
    /// This is used to reduce oscillating contacts and keep the collision cache
    /// warm.  Defaults to 1.0.
    ///
    /// If you have poor simulation quality, increase this number as much as
    /// possible without allowing visible amounts of overlap.
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

impl SpaceRaw {
    pub fn new() -> SpaceRaw {
        unsafe {
            let mut spr = SpaceRaw {
                cp_space: mem::uninitialized(),
                user_data: None
            };
            chip::cpSpaceInit(&mut spr.cp_space);
            spr
        }
    }

    pub fn user_data<T: 'static>(&self) -> Option<&T> {
        self.user_data.as_ref().and_then(|a| a.downcast_ref())
    }

    pub fn user_data_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.user_data.as_mut().and_then(|a| a.downcast_mut())
    }

    pub fn set_user_data<T: 'static>(&mut self, val: T) {
        self.user_data = Some(Box::new(val));
    }

    pub fn set_gravity(&mut self, ax: f64, ay: f64) {
        unsafe {
            chip::cpSpaceSetGravity(&mut self.cp_space, chip::cpv(ax, ay));
        }
    }

    pub fn set_damping(&mut self, damping: f64) {
        unsafe {
            chip::cpSpaceSetDamping(&mut self.cp_space, damping);
        }
    }

    pub fn set_collision_slop(&mut self, slop: f64) {
        unsafe {
            chip::cpSpaceSetCollisionSlop(&mut self.cp_space, slop);
        }
    }

    pub fn set_collision_bias(&mut self, bias: f64) {
        unsafe {
            chip::cpSpaceSetCollisionBias(&mut self.cp_space, bias);
        }
    }

    pub fn set_collision_persistence(&mut self, persistence: u32) {
        unsafe {
            chip::cpSpaceSetCollisionPersistence(&mut self.cp_space, persistence);
        }
    }

    pub fn set_idle_speed_threshold(&mut self, threshold: f64) {
        unsafe {
            chip::cpSpaceSetIdleSpeedThreshold(&mut self.cp_space, threshold);
        }
    }

    pub fn set_iterations(&mut self, iterations: i32) {
        unsafe {
            chip::cpSpaceSetIterations(&mut self.cp_space, iterations);
        }
    }

    pub fn set_sleep_time_threshold(&mut self, threshold: f64) {
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
