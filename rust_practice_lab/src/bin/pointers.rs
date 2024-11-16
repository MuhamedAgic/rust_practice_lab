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

    println!("\n====================================================================================================\n");


}