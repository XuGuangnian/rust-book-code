use std::slice;

pub fn create_raw_pointers() {
    // 通过引用创建裸指针
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // 创建指向任意内存地址的裸指针
    // let address = 0x012345usize;
    // let r = address as *const i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}

pub fn call_unsafe_fn() {
    unsafe fn dangerous() {}

    unsafe {
        dangerous();
    }
}

pub fn create_safe_abstraction() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3); // core::slice
    let (a, b) = split_at_mut(r, 3); // 自定义

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    // (&mut values[..mid], &mut values[mid..])
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Rust调用C-ABI格式的接口
pub fn extern_fn_calls() {
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

// 其他语言调用rust代码 （rust代码按C-ABI暴露接口）
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
pub fn static_variable() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

pub fn unsafe_trait() {
    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

/// 联合体的关键特性是联合体的所有字段共享同一段存储。因此，对联合体的一个字段的写操作会覆盖其他字段，而联合体的尺寸由其尺寸最大的字段的尺寸所决定。
pub fn unions() {
    #[repr(C)]
    union MyUnion {
        f1: u32,
        f2: f32,
    }

    // 只能指定一个字段
    let u = MyUnion { f1: 1 };
    // 访问union, 所有的联合体字段的读取必须放在非安全(unsafe)块里
    let f = unsafe { u.f1 };
    println!("union: {}", f);
}
