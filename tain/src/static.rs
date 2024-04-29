use std::mem::size_of;

pub trait AsAny {}

fn val_to_vec<T>(val: T) -> Vec<u8> {
    let mut data = &val as *const T as *const u8;
    let mut buff = vec![0u8; size_of::<T>()];

    for i in 0..size_of::<T>() {
        buff[i] = unsafe { *data };
        data = unsafe { data.add(1) };
    }

    buff
}

pub fn static_getter<T>() -> &'static T {
    const SIZE: usize = 10;
    static BUFFER: [u8; SIZE] = [0; SIZE];

    //static STORAGE: BTreeMap<&'static str, Box<dyn Any>> = BTreeMap::new();

    let p = &BUFFER[4];

    unsafe { &*(p as *const u8 as *const T) }
}

#[cfg(test)]
mod test {
    use std::mem::transmute;

    use crate::r#static::val_to_vec;

    #[test]
    fn test_val_to_vec() {
        #[derive(Debug, Copy, Clone, PartialEq)]
        struct Data {
            a: bool,
            b: bool,
            c: bool,
            d: bool,
        }

        let data = Data {
            a: true,
            b: false,
            c: true,
            d: false,
        };

        let buff = val_to_vec(data);

        assert_eq!(buff, vec![1, 0, 1, 0]);

        let restored_data: &Data = unsafe { transmute(&buff[0]) };

        assert_eq!(&data, restored_data);
    }
}
