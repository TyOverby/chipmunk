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

impl ShapeRaw {
    pub fn new_circle(body: &mut Body, radius: f64, offset: (f64, f64)) -> ShapeRaw {
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

    pub fn new_box(body: &mut Body, width: f64, height: f64, corner_radius: f64) -> ShapeRaw {
        unsafe {
            ShapeRaw {
                cp_shape: chip::cpBoxShapeNew(body.cp_body(), width, height, corner_radius),
                user_data: None,
                attached_body: body.duplicate()
            }
        }
    }

    pub fn new_poly(body: &mut Body, vertices: &[(f64, f64)], radius: f64) -> ShapeRaw {
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

    pub fn set_density(&mut self, density: f64) {
        unsafe {
            chip::cpShapeSetDensity(density);
        }
    }

    pub fn set_elasticity(&mut self, elasticity: f64) {
        unsafe {
            chip::cpShapeSetElasticity(elasticity);
        }
    }

    // set collision type
    // set filter

    pub fn set_friction(&mut self, friction: f64) {
        unsafe {
            chip::cpShapeSetElasticity(friction);
        }
    }

    pub fn set_mass(&mut self, mass: f64) {
        unsafe {
            chip::cpShapeSetMass(mass);
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
