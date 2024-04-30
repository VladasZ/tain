use std::{
    fmt::Debug,
    sync::{Arc, Mutex, Weak},
};

use crate::Static;

impl Static {
    pub fn arc<T: Default + Debug + 'static>(setup: impl FnOnce() -> T) -> Arc<T> {
        static LOCK: Mutex<()> = Mutex::new(());

        let _lock = LOCK.lock().unwrap();

        if Static::exists::<Weak<T>>() {
            let weak = Static::get::<Weak<T>>();
            return weak.upgrade().unwrap();
        }

        let arc = Arc::new(setup());
        let weak = Arc::downgrade(&arc);

        Static::set(weak);

        arc
    }
}

#[cfg(test)]
mod test {
    use std::{ops::Deref, sync::Arc, thread::sleep, time::Duration};

    use crate::Static;

    fn get_val() -> Arc<u32> {
        Static::arc(|| 5)
    }

    fn check() {
        let arc = get_val();
        sleep(Duration::from_millis(10));
        assert_eq!(arc.deref(), &5);
    }

    #[test]
    fn test1() {
        check();
    }

    #[test]
    fn test2() {
        check();
    }

    #[test]
    fn test3() {
        check();
    }

    #[test]
    fn test4() {
        check();
    }

    #[test]
    fn test6() {
        check();
    }

    #[test]
    fn test7() {
        check();
    }

    #[test]
    fn test8() {
        check();
    }

    #[test]
    fn test9() {
        check();
    }
}
