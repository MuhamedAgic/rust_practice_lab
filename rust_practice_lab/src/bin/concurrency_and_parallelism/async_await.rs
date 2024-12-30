use std::future::Future;

async fn fib(n: u64) -> Option<u64> {
    if n > 60 {
        println!("Please use something smaller than 60, this is a small example");
        return None;
    }
    let mut next_val = 1;
    let mut curr_val = 0;
    for i in 0..n {
        let temp= curr_val + next_val;
        curr_val = next_val;
        next_val = temp;
        println!("iteration {}, value: {}", i, next_val);
        std::thread::sleep(std::time::Duration::from_millis(100)); // Add a small delay
    }
    return Some(next_val);
}

fn using_fib(n: u64) -> impl Future<Output = Option<u64>> {
    // When using an async function, it returns a future
    println!("Using the async fib function");
    let result= async move {
        // to force the async block to take ownership of `n` (and any other referenced variables), use the `move` keyword
        if let Some(mut res) = fib(n).await {
            res += 999;
            return Some(res);
        } else {
            return None
        }
    };
    return result;
}


#[tokio::main]
async fn main() {
    let fib_result = using_fib(10);
    let fib_result2 = using_fib(40);
    let fib_result3 = using_fib(59);

    let (result1, result2, result3) = tokio::join!(fib_result, fib_result2, fib_result3);

    println!("fib result: {:?}", result1);
    println!("fib result 2: {:?}", result2);
    println!("fib result 3: {:?}", result3);
}