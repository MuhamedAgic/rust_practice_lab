use std::any::type_name;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let a: i32 = 5;
    let b: &i32 = &a;
    let a_ptr = a as *const i32;
    println!("Variable a:\n    Type: {}\n    value: {}\n    address: {:p}", type_of(&a), a, &a);
    println!("Variable b:\n    Type: {}\n    value: {}\n    address: {:p}\n    address of value b points to: {:p}", type_of(&b), b, &b, b);
    println!("Raw pointer to a: {:p}", a_ptr);

    println!("\n\n====================================================================================================\n\n");

    // rust allows unsafe programming
    unsafe {
        let c: i32 = 5;
        let d: i32 = 10;
        let mut e = &c as *const i32; // raw pointer to c
        println!("Value of e: {}", *e); // dereferencing raw pointer only allowed in unsafe block

        e = &d as *const i32;
        println!("Value of e: {}", *e); // pointer now points to d
    }

}