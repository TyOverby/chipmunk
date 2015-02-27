use std::any::Any;
use std::rc::Rc;
use std::cell::UnsafeCell;
use std::mem;

use super::user_data::UserData;

use chip;

struct ShapeRaw {
    cp_body: chip::cpShape,
    user_data: Option<Box<Any>>
}

pub struct Shape {
    raw: Rc<UnsafeCell<ShapeRaw>>
}

impl Shape {
    fn
}
