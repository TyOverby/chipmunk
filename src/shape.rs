use std::any::Any;
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::mem;
use std::marker::PhantomData;

use super::user_data::UserData;
use super::body::Body;

use chip;

use void::Void;

pub struct Shape<T> {
    raw: Rc<UnsafeCell<ShapeRaw<T>>>
}

struct ShapeRaw<T> {
    cp_shape: *mut chip::cpShape,
    user_data: Option<Box<Any>>,
    _attached_body: Body<Void>,
    _phantom: PhantomData<T>
}

impl <T> Shape<T> {
    pub fn new_circle<A>(body: &mut Body<A>, radius: f64, offset: (f64, f64)) -> Shape<T> {
        Shape {
            raw: Rc::new(UnsafeCell::new(ShapeRaw::new_circle(body, radius, offset)))
        }
    }

    pub fn new_box<A>(body: &mut Body<A>, width: f64, height: f64, corner_radius: f64) -> Shape<T> {
        Shape {
            raw: Rc::new(UnsafeCell::new(ShapeRaw::new_box(body, width, height, corner_radius)))
        }
    }

    pub fn new_poly<A>(body: &mut Body<A>, vertices:&[(f64, f64)], radius: f64) -> Shape<T> {
        Shape {
            raw: Rc::new(UnsafeCell::new(ShapeRaw::new_poly(body, vertices, radius)))
        }
    }

    // TODO: hide from docs
    pub fn get_cp_shape(&mut self) -> *mut chip::cpShape {
        unsafe {
            (*self.raw.get()).cp_shape
        }
    }

    // TODO: hide from docs
    pub fn duplicate(&mut self) -> Shape<Void> {
        use std::mem::transmute;
        unsafe {
            transmute(Shape {raw: self.raw.clone()})
        }
    }

    forward!(density(&self) -> f64,
    /// Returns the density of this shape.
    );

    forward!(elasticity(&self) -> f64,
    /// Returns the elasticity of this shape.
    );

    forward!(friction(&self) -> f64,
    /// Returns the friction of this shape.
    );

    forward!(mass(&self) -> f64,
    /// Returns the mass of this shape.
    );

    forward!(is_sensor(&self) -> bool,
    /// Returns whether this shape is a sensor or not.
    );

    forward!(surface_velocity(&self) -> (f64, f64),
    /// Returns the surface velocity of this collision shape.
    );

    forward!(set_density(&mut self, density: f64) -> (),
    /// Sets the density of the shape.
    );

    forward!(set_elasticity(&mut self, elasticity: f64) -> (),
    /// Sets the elasticity of collisions performed on this shape.
    );

    forward!(set_friction(&mut self, friction: f64) -> (),
    /// Sets the amout of friction that collisions involving this shape experiences.
    );

    forward!(set_mass(&mut self, mass: f64) -> (),
    /// Sets the mass of this shape.
    );

    forward!(set_sensor(&mut self, sensor: bool) -> (),
    /// Sets this shape as being a sensor or not.
    ///
    /// A sensor is a collision shape that does not influence collision results,
    /// but will trigger collision callbacks when colliding with other shapes.
    );

    forward!(set_surface_velocity(&mut self, surface_velocity: (f64, f64)) -> (),
    /// Sets the velocity of the shape's surface.
    ///
    /// This velocity is used in the collision response when
    /// calculating the friction only.
    );
}

impl <T> ShapeRaw<T> {
    fn new_circle<A>(body: &mut Body<A>, radius: f64, offset: (f64, f64)) -> ShapeRaw<T> {
        unsafe {
            ShapeRaw {
                cp_shape: chip::cpCircleShapeNew(
                                            body.get_cp_body(),
                                            radius,
                                            chip::cpv(offset.0, offset.1)),
                user_data: None,
                _attached_body: body.duplicate(),
                _phantom: PhantomData
            }
        }
    }

    fn new_box<A>(body: &mut Body<A>, width: f64, height: f64, corner_radius: f64) -> ShapeRaw<T> {
        unsafe {
            ShapeRaw {
                cp_shape: chip::cpBoxShapeNew(body.get_cp_body(), width, height, corner_radius),
                user_data: None,
                _attached_body: body.duplicate(),
                _phantom: PhantomData
            }
        }
    }

    fn new_poly<A>(body: &mut Body<A>, vertices: &[(f64, f64)], radius: f64) -> ShapeRaw<T> {
        unsafe {
            ShapeRaw {
                cp_shape: chip::cpPolyShapeNewRaw(
                              body.get_cp_body(),
                              vertices.len() as i32,
                              mem::transmute(vertices.as_ptr()),
                              radius),
                user_data: None,
                _attached_body: body.duplicate(),
                _phantom: PhantomData
            }
        }
    }

    fn density(&self) -> f64 {
        unsafe {
            chip::cpShapeGetDensity(self.cp_shape)
        }
    }

    fn elasticity(&self) -> f64 {
        unsafe {
            chip::cpShapeGetElasticity(self.cp_shape)
        }
    }

    fn friction(&self) -> f64 {
        unsafe {
            chip::cpShapeGetFriction(self.cp_shape)
        }
    }

    fn mass(&self) -> f64 {
        unsafe {
            chip::cpShapeGetMass(self.cp_shape)
        }
    }

    fn is_sensor(&self) -> bool {
        unsafe {
            let r = chip::cpShapeGetSensor(self.cp_shape);
            if r == 0 {false} else {true}
        }
    }

    fn surface_velocity(&self) -> (f64, f64) {
        unsafe {
            let v = chip::cpShapeGetSurfaceVelocity(self.cp_shape);
            (v.x, v.y)
        }
    }

    fn set_density(&mut self, density: f64) {
        unsafe {
            chip::cpShapeSetDensity(self.cp_shape, density);
        }
    }

    fn set_elasticity(&mut self, elasticity: f64) {
        unsafe {
            chip::cpShapeSetElasticity(self.cp_shape, elasticity);
        }
    }

    // set collision type
    // set filter

    fn set_friction(&mut self, friction: f64) {
        unsafe {
            chip::cpShapeSetElasticity(self.cp_shape, friction);
        }
    }

    fn set_mass(&mut self, mass: f64) {
        unsafe {
            chip::cpShapeSetMass(self.cp_shape, mass);
        }
    }

    fn set_sensor(&mut self, is_sensor: bool) {
        unsafe {
            let v = if is_sensor {1} else {0};
            chip::cpShapeSetSensor(self.cp_shape, v);
        }
    }


    fn set_surface_velocity(&mut self, surface_velocity: (f64, f64)) {
        unsafe {
            let cpv = chip::cpv(surface_velocity.0, surface_velocity.1);
            chip::cpShapeSetSurfaceVelocity(self.cp_shape, cpv);
        }
    }
}

#[unsafe_destructor]
impl <T> Drop for ShapeRaw<T> {
    fn drop(&mut self) {
        unsafe {
            chip::cpShapeFree(self.cp_shape);
        }
    }
}


impl <T: 'static> UserData<T> for ShapeRaw<T> {
    fn get_userdata_box(&self) -> &Option<Box<Any>> {
        &self.user_data
    }

    fn get_userdata_mut_box(&mut self) -> &mut Option<Box<Any>> {
        &mut self.user_data
    }
}
