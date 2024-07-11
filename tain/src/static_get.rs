use std::{
    any::type_name,
    collections::BTreeMap,
    mem::size_of,
    ptr::{addr_of, from_ref},
    sync::Mutex,
};

static LOCK: Mutex<()> = Mutex::new(());
static mut STORAGE: BTreeMap<&'static str, Vec<u8>> = BTreeMap::new();

fn val_to_vec<T>(val: T) -> Vec<u8> {
    let mut data = addr_of!(val).cast::<u8>();
    let mut buff = vec![0u8; size_of::<T>()];

    for byte in &mut buff {
        *byte = unsafe { *data };
        data = unsafe { data.add(1) };
    }

    buff
}

fn vec_to_val<T>(v: &[u8]) -> &T {
    let buff = &v[0];
    unsafe { &*from_ref::<u8>(buff).cast::<T>() }
}

pub struct Static;

impl Static {
    pub fn get<T: Default>() -> &'static T {
        let _lock = LOCK.lock().unwrap();
        let type_name = type_name::<T>();
        unsafe {
            let data = STORAGE.entry(type_name).or_insert_with(|| val_to_vec(T::default()));
            vec_to_val(data)
        }
    }

    pub fn set<T>(val: T) -> &'static T {
        let _lock = LOCK.lock().unwrap();
        let type_name = type_name::<T>();
        unsafe {
            assert!(
                STORAGE.insert(type_name, val_to_vec(val)).is_none(),
                "Static initialization can pe performen only once. Type: {type_name}"
            );
            let data = STORAGE.get(type_name).unwrap();
            vec_to_val(data)
        }
    }

    pub fn exists<T>() -> bool {
        let _lock = LOCK.lock().unwrap();
        unsafe { STORAGE.contains_key(type_name::<T>()) }
    }
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicU8, Ordering};

    use fake::Fake;

    use crate::static_get::{val_to_vec, vec_to_val, Static};

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Data {
        a: bool,
        b: bool,
        c: bool,
        d: bool,
    }

    impl Default for Data {
        fn default() -> Self {
            Self {
                a: true,
                b: false,
                c: true,
                d: false,
            }
        }
    }

    #[test]
    fn test_static_get() {
        assert_eq!(Static::exists::<u32>(), false);
        assert_eq!(Static::get::<u32>(), &0);
        assert_eq!(Static::exists::<u32>(), true);

        assert_eq!(Static::get::<bool>(), &false);
        assert_eq!(Static::get::<String>(), &String::default());
        assert_eq!(Static::get::<Data>(), &Data::default());

        let atomic = Static::get::<AtomicU8>();
        assert_eq!(atomic.load(Ordering::Relaxed), 0);

        for _ in 0..1_000_000 {
            let val = (0..255).fake();

            let atomic = Static::get::<AtomicU8>();
            atomic.store(val, Ordering::Relaxed);
            assert_eq!(Static::get::<AtomicU8>().load(Ordering::Relaxed), val);
        }

        let atomic = Static::get::<AtomicU8>();
        atomic.store(55, Ordering::Relaxed);
        assert_eq!(Static::get::<AtomicU8>().load(Ordering::Relaxed), 55);
    }

    #[test]
    fn test_static_set() {
        assert_eq!(Static::set::<f64>(5.0), &5.0);
        assert_eq!(Static::get::<f64>(), &5.0);
    }

    #[test]
    fn test_val_to_vec() {
        let data = Data::default();

        let buff = val_to_vec(data);
        assert_eq!(buff, vec![1, 0, 1, 0]);

        let restored_data: &Data = vec_to_val(&buff);
        assert_eq!(&data, restored_data);
    }
}
