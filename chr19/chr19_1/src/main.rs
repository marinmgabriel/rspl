static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5; 

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // To dereference raw ptrs we need to use unsafe block
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Interact with other Programming lang with 'extern'
    // Calling the function is done in a 'unsafe' block 
    unsafe {
        println!("Absolut value of -3 according to C is {}", abs(-3));
    }

    println!("name is: {}", HELLO_WORLD);

    unsafe {
        println!("COUNTER: {}", COUNTER);
        add_to_count(3);
        println!("COUNTER: {}", COUNTER);
    }
}

extern "C" {
    fn abs(intput: i32) -> i32;
}

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
