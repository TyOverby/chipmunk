use std::any::Any;
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::marker::PhantomData;

use super::user_data::UserData;

use chip;
use void::Void;

pub struct Arbiter<T=Void> {
    raw: Rc<UnsafeCell<ArbiterRaw<T>>>
}

pub struct ContactPointSet {
    pub count: u32,
    pub normal: (f64, f64),
    pub points: [ContactPoint; 2]
}

pub struct ContactPoint {
    pub a: (f64, f64),
    pub b: (f64, f64),
    pub dist: f64
}

struct ArbiterRaw<T=Void> {
    cp_arbiter: chip::cpArbiter,
    user_data: Option<Box<Any>>,
    _phantom: PhantomData<T>
}


impl <T> ArbiterRaw<T> {
    fn contact_point_set(&self) -> ContactPointSet {
        let cps = unsafe { chip::cpArbiterGetContactPointSet(&self.cp_arbiter) };

        ContactPointSet {
            count: cps.count as u32,
            normal: (cps.normal.x, cps.normal.y),
            points: [
                ContactPoint {
                    a: cps.points[0].pointA.to_tuple(),
                    b: cps.points[0].pointB.to_tuple(),
                    dist: cps.points[0].distance
                },
                ContactPoint {
                    a: cps.points[1].pointA.to_tuple(),
                    b: cps.points[1].pointB.to_tuple(),
                    dist: cps.points[1].distance
                }
            ]

        }
    }

    fn count(&self) -> u32 {
        unsafe { chip::cpArbiterGetCount(&self.cp_arbiter) as u32 }
    }

    fn depth(&self, i: u32) -> f64 {
        unsafe { chip::cpArbiterGetDepth(&self.cp_arbiter, i as i32) }
    }

    fn friction(&self) -> f64 {
        unsafe { chip::cpArbiterGetFriction(&self.cp_arbiter) }
    }

    fn normal(&self) -> (f64, f64) {
        unsafe { chip::cpArbiterGetNormal(&self.cp_arbiter).to_tuple() }
    }

    fn point_a(&self, i: u32) -> (f64, f64) {
        unsafe { chip::cpArbiterGetPointA(&self.cp_arbiter, i as i32).to_tuple() }
    }

    fn point_b(&self, i: u32) -> (f64, f64) {
        unsafe { chip::cpArbiterGetPointB(&self.cp_arbiter, i as i32).to_tuple() }
    }

    fn restitution(&self) -> f64 {
        unsafe { chip::cpArbiterGetRestitution(&self.cp_arbiter) }
    }

    fn surface_velocity(&self) -> (f64, f64) {
        unsafe { chip::cpArbiterGetSurfaceVelocity(&self.cp_arbiter).to_tuple() }
    }

    fn set_friction(&mut self, friction: f64) {
        unsafe { chip::cpArbiterSetFriction(&mut self.cp_arbiter, friction) };
    }

    fn set_restitution(&mut self, restitution: f64) {
        unsafe { chip::cpArbiterSetRestitution(&mut self.cp_arbiter, restitution) };
    }

    fn set_surface_velocity(&mut self, vx: f64, vy: f64) {
        unsafe { chip::cpArbiterSetSurfaceVelocity(&mut self.cp_arbiter, chip::cpv(vx, vy)) };
    }
}

impl <T> Arbiter<T> {
    forward!(contact_point_set(&self) -> ContactPointSet,
    /// Returns the set of contact points.
    );

    forward!(count(&self) -> u32,
    /// Returns the number of points of contact.
    );

    forward!(depth(&self, i: u32) -> f64,
    /// Returns the depth of the penetration for a point of contact.
    );

    forward!(friction(&self) -> f64,
    /// Returns the friction of the contact.
    );

    forward!(normal(&self) -> (f64, f64),
    /// Returns the normal vector of the collision.
    );

    forward!(point_a(&self, i: u32) -> (f64, f64),
    /// Returns a point on object `a` in the colision for a point of intersection.
    );

    forward!(point_b(&self, i: u32) -> (f64, f64),
    /// Returns a point on object `b` in the colision for a point of intersection.
    );

    forward!(restitution(&self) -> f64,
    /// Returns the restitution for this collision.
    );

    forward!(surface_velocity(&self) -> (f64, f64),
    /// Returns the surface velocity of this collision.
    );

    forward!(set_surface_velocity(&mut self, vx: f64, vy: f64) -> (),
    /// Sets the surface velocity for this collision.
    );

    forward!(set_friction(&mut self, friction: f64) -> (),
    /// Sets the friction for this collision.
    );

    forward!(set_restitution(&mut self, restitution: f64) -> (),
    /// Sets the restitutionfor this collision.
    );
}

impl <T: Any> UserData<T> for Arbiter<T> {
    fn get_userdata_box(&self) -> &Option<Box<Any>> {
        unsafe { &(*self.raw.get()).user_data }
    }

    fn get_userdata_mut_box(&mut self) -> &mut Option<Box<Any>> {
        unsafe { &mut (*self.raw.get()).user_data }

    }
}
