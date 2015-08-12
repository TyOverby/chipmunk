use std::any::Any;
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::mem::{transmute, zeroed};
use std::marker::PhantomData;

use super::user_data::UserData;
use super::body::Body;

use chip;

use void::Void;

pub enum Shape<T = Void> {
    Poly(PolyShape<T>),
    Circle(CircleShape<T>),
    Segment(SegmentShape<T>)
}

pub struct PolyShape<T = Void> {
    raw: Rc<UnsafeCell<PolyShapeRaw<T>>>
}

pub struct CircleShape<T = Void> {
    raw: Rc<UnsafeCell<CircleShapeRaw<T>>>
}

pub struct SegmentShape<T = Void> {
    raw: Rc<UnsafeCell<SegmentShapeRaw<T>>>
}

struct PolyShapeRaw<T = Void> {
    cp_shape: chip::cpPolyShape,
    user_data: Option<Box<Any>>,
    _attached_body: Body<Void>,
    _phantom: PhantomData<T>
}

struct CircleShapeRaw<T = Void> {
    cp_shape: chip::cpCircleShape,
    user_data: Option<Box<Any>>,
    _attached_body: Body<Void>,
    _phantom: PhantomData<T>
}

struct SegmentShapeRaw<T = Void> {
    cp_shape: chip::cpSegmentShape,
    user_data: Option<Box<Any>>,
    _attached_body: Body<Void>,
    _phantom: PhantomData<T>
}

impl <T: 'static + Any> UserData<T> for Shape<T> {
    fn get_userdata_box(&self) -> &Option<Box<Any>> {
        unsafe {
            match *self {
                Shape::Poly(ref p) => &(*p.raw.get()).user_data,
                Shape::Circle(ref p) => &(*p.raw.get()).user_data,
                Shape::Segment(ref p) => &(*p.raw.get()).user_data,
            }
        }
    }

    fn get_userdata_mut_box(&mut self) -> &mut Option<Box<Any>> {
        unsafe {
            match *self {
                Shape::Poly(ref p) => &mut(*p.raw.get()).user_data,
                Shape::Circle(ref p) => &mut(*p.raw.get()).user_data,
                Shape::Segment(ref p) => &mut(*p.raw.get()).user_data,
            }
        }
    }
}

impl Shape<Void> {
    pub fn new_segment(body: &mut Body, start: (f64, f64), end: (f64, f64), radius: f64) -> Shape<Void> {
        let mut shape = SegmentShapeRaw {
            cp_shape: unsafe { zeroed() },
            user_data: None,
            _attached_body: unsafe { body.duplicate() },
            _phantom: PhantomData
        };
        let a = chip::cpv(start.0, start.1);
        let b = chip::cpv(end.0, end.1);
        unsafe {
            chip::cpSegmentShapeInit(&mut shape.cp_shape, body.get_cp_body(), a, b, radius);
        }

        Shape::Segment(SegmentShape{ raw: Rc::new(UnsafeCell::new(shape)) })
    }

    pub fn new_circle(body: &mut Body, radius: f64, offset: (f64, f64)) -> Shape<Void> {
        let mut shape = CircleShapeRaw {
            cp_shape: unsafe { zeroed() },
            user_data: None,
            _attached_body: unsafe { body.duplicate() },
            _phantom: PhantomData
        };

        let offset = chip::cpv(offset.0, offset.1);
        unsafe {
            chip::cpCircleShapeInit(&mut shape.cp_shape, body.get_cp_body(), radius, offset);
        }

        Shape::Circle(CircleShape{ raw: Rc::new(UnsafeCell::new(shape)) })
    }

    pub fn new_poly(body: &mut Body, points: &[(f64)], radius: f64) -> Shape<Void> {
        let mut shape = PolyShapeRaw {
            cp_shape: unsafe { zeroed() },
            user_data: None,
            _attached_body: unsafe { body.duplicate() },
            _phantom: PhantomData
        };

        unsafe {
            chip::cpPolyShapeInitRaw(&mut shape.cp_shape, body.get_cp_body(),
                                    points.len() as i32, transmute(points.as_ptr()),
                                    radius);
        }

        Shape::Poly(PolyShape{ raw: Rc::new(UnsafeCell::new(shape)) })
    }

    pub fn new_box(body: &mut Body, width: f64, height: f64, radius: f64) -> Shape<Void> {
        let mut shape = PolyShapeRaw {
            cp_shape: unsafe { zeroed() },
            user_data: None,
            _attached_body: unsafe { body.duplicate() },
            _phantom: PhantomData
        };

        unsafe {
            chip::cpBoxShapeInit(&mut shape.cp_shape, body.get_cp_body(),
                                    width, height, radius);
        }

        Shape::Poly(PolyShape{ raw: Rc::new(UnsafeCell::new(shape)) })
    }
}

impl <T> Shape<T> {
    pub unsafe fn get_cp_shape(&self) -> *const chip::cpShape {
        match *self {
            Shape::Poly(ref p) => transmute(&(*p.raw.get()).cp_shape),
            Shape::Circle(ref p) => transmute(&(*p.raw.get()).cp_shape),
            Shape::Segment(ref p) => transmute(&(*p.raw.get()).cp_shape),
        }
    }

    pub unsafe fn get_cp_shape_mut(&mut self) -> *mut chip::cpShape {
        transmute(self.get_cp_shape())
    }

    pub unsafe fn duplicate(&self) -> Shape<Void> {
        match *self {
            Shape::Poly(ref p) => Shape::Poly(transmute(PolyShape{raw: p.raw.clone()})),
            Shape::Circle(ref p) => Shape::Circle(transmute(CircleShape{raw: p.raw.clone()})),
            Shape::Segment(ref p) => Shape::Segment(transmute(SegmentShape{raw: p.raw.clone()})),
        }
    }

    pub fn density(&self) -> f64 {
        unsafe {
            chip::cpShapeGetDensity(self.get_cp_shape())
        }
    }

    pub fn elasticity(&self) -> f64 {
        unsafe {
            chip::cpShapeGetElasticity(self.get_cp_shape())
        }
    }

    pub fn friction(&self) -> f64 {
        unsafe {
            chip::cpShapeGetFriction(self.get_cp_shape())
        }
    }

    pub fn mass(&self) -> f64 {
        unsafe {
            chip::cpShapeGetMass(self.get_cp_shape())
        }
    }

    /// Returns true if this shape is a sensor.
    ///
    /// A sensor is a shape that dosn't participate in collisions, but
    /// still calls callbacks.
    pub fn is_sensor(&self) -> bool {
        unsafe {
            let r = chip::cpShapeGetSensor(self.get_cp_shape());
            if r == 0 {false} else {true}
        }
    }

    /// Returns the velocity of the shape at the surface.
    pub fn surface_velocity(&self) -> (f64, f64) {
        unsafe {
            let v = chip::cpShapeGetSurfaceVelocity(self.get_cp_shape());
            (v.x, v.y)
        }
    }

    pub fn set_density(&mut self, density: f64) {
        unsafe {
            chip::cpShapeSetDensity(self.get_cp_shape_mut(), density);
        }
    }

    pub fn set_elasticity(&mut self, elasticity: f64) {
        unsafe {
            chip::cpShapeSetElasticity(self.get_cp_shape_mut(), elasticity);
        }
    }

    // set collision type
    // set filter

    pub fn set_friction(&mut self, friction: f64) {
        unsafe {
            chip::cpShapeSetElasticity(self.get_cp_shape_mut(), friction);
        }
    }

    pub fn set_mass(&mut self, mass: f64) {
        unsafe {
            chip::cpShapeSetMass(self.get_cp_shape_mut(), mass);
        }
    }

    pub fn set_sensor(&mut self, is_sensor: bool) {
        unsafe {
            let v = if is_sensor {1} else {0};
            chip::cpShapeSetSensor(self.get_cp_shape_mut(), v);
        }
    }


    pub fn set_surface_velocity(&mut self, surface_velocity: (f64, f64)) {
        unsafe {
            let cpv = chip::cpv(surface_velocity.0, surface_velocity.1);
            chip::cpShapeSetSurfaceVelocity(self.get_cp_shape_mut(), cpv);
        }
    }
}

impl <T> PolyShape<T> {
    forward!(count(&self) -> usize,
    /// Returns the number of vertices in this shape.
    );

    forward!(radius(&self) -> f64,
    /// Returns the radius that encompases all the vertices.
    );

    forward!(vert(&self, i: usize) -> (f64, f64),
    /// Returns the i-th vertex in this shape.
    );
}

impl <T> CircleShape<T> {
    forward!(offset(&self) -> (f64, f64),
    /// Returns the local offset at which the shape is
    /// placed relative to the body that it is attached to.
    );

    forward!(radius(&self) -> f64,
    /// Returns the radius of the circle.
    );
}

impl <T> SegmentShape<T> {
    forward!(start(&self) -> (f64, f64),
    /// Returns the first point in the segment.
    );

    forward!(end(&self) -> (f64, f64),
    /// Returns the second poin in the segment.
    );

    forward!(normal(&self) -> (f64, f64),
    /// Returns the normal vector given by this segment.
    );

    forward!(radius(&self) -> f64,
    /// Returns the radius that encompases both points.
    );
}

impl <T> PolyShapeRaw<T> {
    fn count(&self) -> usize {
        unsafe {
            chip::cpPolyShapeGetCount(transmute(&self.cp_shape)) as usize
        }
    }

    fn radius(&self) -> f64 {
        unsafe {
            chip::cpPolyShapeGetRadius(transmute(&self.cp_shape))
        }
    }

    fn vert(&self, index: usize)  -> (f64, f64) {
        unsafe {
            let index = index as i32;
            let cpv = chip::cpPolyShapeGetVert(transmute(&self.cp_shape), index);
            (cpv.x, cpv.y)
        }
    }
}

impl <T> CircleShapeRaw<T> {
    fn offset(&self) -> (f64, f64) {
        unsafe {
            let cpv = chip::cpCircleShapeGetOffset(transmute(&self.cp_shape));
            (cpv.x, cpv.y)
        }
    }

    fn radius(&self) -> f64 {
        unsafe {
            chip::cpCircleShapeGetRadius(transmute(&self.cp_shape))
        }
    }
}

impl <T> SegmentShapeRaw<T> {
    fn start(&self) -> (f64, f64) {
        unsafe {
            let cpv = chip::cpSegmentShapeGetA(transmute(&self.cp_shape));
            (cpv.x, cpv.y)
        }
    }

    fn end(&self) -> (f64, f64) {
        unsafe {
            let cpv = chip::cpSegmentShapeGetB(transmute(&self.cp_shape));
            (cpv.x, cpv.y)
        }
    }

    fn normal(&self) -> (f64, f64) {
        unsafe {
            let cpv = chip::cpSegmentShapeGetNormal(transmute(&self.cp_shape));
            (cpv.x, cpv.y)
        }
    }

    fn radius(&self) -> f64 {
        unsafe {
            chip::cpSegmentShapeGetRadius(transmute(&self.cp_shape))
        }
    }
}

impl <T> Drop for SegmentShapeRaw<T> {
    fn drop(&mut self) {
        unsafe {
            chip::cpShapeDestroy(transmute(&mut self.cp_shape));
        }
    }
}

impl <T> Drop for CircleShapeRaw<T> {
    fn drop(&mut self) {
        unsafe {
            chip::cpShapeDestroy(transmute(&mut self.cp_shape));
        }
    }
}

impl <T> Drop for PolyShapeRaw<T> {
    fn drop(&mut self) {
        unsafe {
            chip::cpShapeDestroy(transmute(&mut self.cp_shape));
        }
    }
}
