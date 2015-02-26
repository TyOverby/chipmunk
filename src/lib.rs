extern crate "chipmunk-sys" as chip;

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

mod util;
pub mod user_data;

pub mod space;
pub mod body;

