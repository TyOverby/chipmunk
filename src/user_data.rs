use std::any::Any;

pub trait UserData {
    fn get_userdata_box(&self) -> &Option<Box<Any>>;
    fn get_userdata_mut_box(&mut self) -> &mut Option<Box<Any>>;

    fn user_data<T: 'static>(&self) -> Option<&T> {
        self.get_userdata_box().as_ref().and_then(|a| a.downcast_ref())
    }

    fn user_data_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.get_userdata_mut_box().as_mut().and_then(|a| a.downcast_mut())
    }

    fn set_user_data<T: 'static>(&mut self, val: T) {
        *self.get_userdata_mut_box() = Some(Box::new(val));
    }
}

