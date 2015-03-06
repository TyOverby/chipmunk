use std::any::Any;
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::mem;

use super::user_data::UserData;
use super::body::Body;

use chip;



pub struct Shape {
    raw: Rc<UnsafeCell<ShapeRaw>>
}

struct ShapeRaw {
    cp_shape: *mut chip::cpShape,
    user_data: Option<Box<Any>>,
    attached_body: Body
}

impl Shape {
    pub fn new_circle(body: &mut Body, radius: f64, offset: (f64, f64)) -> Shape {
        Shape {
            raw: Rc::new(UnsafeCell::new(ShapeRaw::new_circle(body, radius, offset)))
        }
    }

    pub fn new_box(body: &mut Body, width: f64, height: f64, corner_radius: f64) -> Shape {
        Shape {
            raw: Rc::new(UnsafeCell::new(ShapeRaw::new_box(body, width, height, corner_radius)))
        }
    }

    pub fn new_poly(body: &mut Body, vertices:&[(f64, f64)], radius: f64) -> Shape {
        Shape {
            raw: Rc::new(UnsafeCell::new(ShapeRaw::new_poly(body, vertices, radius)))
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

impl ShapeRaw {
    fn new_circle(body: &mut Body, radius: f64, offset: (f64, f64)) -> ShapeRaw {
        unsafe {
            ShapeRaw {
                cp_shape: chip::cpCircleShapeNew(
                                            body.cp_body(),
                                            radius,
                                            chip::cpv(offset.0, offset.1)),
                user_data: None,
                attached_body: body.duplicate()
            }
        }
    }

    fn new_box(body: &mut Body, width: f64, height: f64, corner_radius: f64) -> ShapeRaw {
        unsafe {
            ShapeRaw {
                cp_shape: chip::cpBoxShapeNew(body.cp_body(), width, height, corner_radius),
                user_data: None,
                attached_body: body.duplicate()
            }
        }
    }

    fn new_poly(body: &mut Body, vertices: &[(f64, f64)], radius: f64) -> ShapeRaw {
        unsafe {
            ShapeRaw {
                cp_shape: chip::cpPolyShapeNewRaw(
                              body.cp_body(),
                              vertices.len() as i32,
                              mem::transmute(vertices.as_ptr()),
                              radius),
                user_data: None,
                              attached_body: body.duplicate()
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

impl Drop for ShapeRaw {
    fn drop(&mut self) {
        unsafe {
            chip::cpShapeFree(self.cp_shape);
        }
    }
}

impl UserData for ShapeRaw {
    fn get_userdata_box(&self) -> &Option<Box<Any>> {
        &self.user_data
    }

    fn get_userdata_mut_box(&mut self) -> &mut Option<Box<Any>> {
        &mut self.user_data
    }
}
