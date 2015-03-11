use chip;

pub fn moment_of_circle(mass: f64, r1: f64, r2: f64) -> f64 {
    unsafe {
        chip::cpMomentForCircle(mass, r1, r2, chip::cpv(0.0, 0.0))
    }
}

/*
use std::mem;
use std::rc::Rc;


pub unsafe fn incr_rc<T>(rc: &Rc<T>) {
    let cln = rc.clone();
    mem::forget(cln);
}

pub unsafe fn decr_rc<T>(rc: &Rc<T>) {
    let cln: Rc<T> = mem::transmute_copy(rc);
    mem::drop(cln);
}

#[cfg(test)]
mod test {
    use std::rc::Rc;

    struct DontDrop {
        dropped: *mut bool
    }

    impl Drop for DontDrop {
        fn drop(&mut self) {
            unsafe { *self.dropped = true; }
        }
    }

    impl DontDrop {
        fn new(cd: *mut bool) -> DontDrop {
            DontDrop {
                dropped: cd
            }
        }
    }

    #[test]
    fn test_assumption() {
        let mut dropped = false;
        {
            let foo = Rc::new(DontDrop::new(&mut dropped));
        }
        assert!(dropped);
    }

    #[test]
    fn test_incr() {
        let mut dropped = false;
        {
            let foo = Rc::new(DontDrop::new(&mut dropped));
            unsafe {super::incr_rc(&foo)};
        }
        assert!(!dropped);
    }

    #[test]
    fn test_incr_decr() {
        let mut dropped = false;
        {
            let foo = Rc::new(DontDrop::new(&mut dropped));
            unsafe {super::incr_rc(&foo)};
            unsafe {super::decr_rc(&foo)};
        }
        assert!(dropped);
    }

    #[test]
    fn test_incr_decr2() {
        let mut dropped = false;
        {
            let foo = Rc::new(DontDrop::new(&mut dropped));
            unsafe {super::incr_rc(&foo)};
            unsafe {super::incr_rc(&foo)};
            unsafe {super::decr_rc(&foo)};
        }
        assert!(!dropped);
    }
}*/
