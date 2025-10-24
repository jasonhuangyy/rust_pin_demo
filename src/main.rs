use std::{marker::PhantomPinned, pin::Pin, ptr::NonNull};

pub struct SelfRef {
    value: String,
    pointer_to_value: NonNull<String>,
    _pin: PhantomPinned,
}

impl SelfRef {
    fn new(data: String) -> Pin<Box<Self>> {
        let res = SelfRef {
            value: data,
            pointer_to_value: NonNull::dangling(),
            _pin: PhantomPinned,
        };

        let mut boxed = Box::pin(res);
        let slice = NonNull::from(&boxed.value);

        unsafe {
            let mut_ref: Pin<&mut Self> = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).pointer_to_value = slice;
        }

        boxed
    }
}
fn main() {
    let data = "aaa".to_string();
    let v = SelfRef::new(data);
    println!("{:?}", v.pointer_to_value);
    println!("{:?}", NonNull::from(&v.value));

    // 对象复制移动，实际数据并没有发送移动
    let still_unmoved = v;
    println!("{:?}", still_unmoved.pointer_to_value);
    println!("{:?}", NonNull::from(&still_unmoved.value));

    // let mut new_unmoved = SelfRef::new("world".to_string());
    // std::mem::swap(&mut *still_unmoved, &mut *new_unmoved);
}
