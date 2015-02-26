use std::any::Any;

pub trait UserData {
    fn get_box(&self) -> Option<&Box<Any>>;
    fn get_mut_box(&mut self) -> Option<&mut Box<Any>>;
    fn set_box(&mut self, Box<Any>);

    fn user_data<T: 'static>(&self) -> Option<&T> {
        self.get_box().and_then(|a| a.downcast_ref())
    }

    fn user_data_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.get_mut_box().and_then(|a| a.downcast_mut())
    }

    fn set_user_data<T: 'static>(&mut self, val: T) {
        self.set_box(Box::new(val));
    }
}

