use rand::Error;

fn example(n: i32) -> Option<i32> {
    // not a real use case, but illustrates the 'if let' constructions
    if n < 10 {
        return Some(n);
    }
    return None;
}


fn main() {

    // normally, one would do something like this
    let result = example(3);
    if result == Some(3) {
        println!("Success: {}", result.unwrap());
    }
    else {
        println!("Failed: {:?}", result);
    }

    // here you can combine an if with a let statement, shortening the code
    if let Some(n) = example(9) {
        println!("Success, if let construction: {}", n);
    }
    else {
        println!("Failed: {:?}", result);
    }

}