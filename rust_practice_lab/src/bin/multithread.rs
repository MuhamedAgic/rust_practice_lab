use colored::*;
use rand::Rng;
use rayon::prelude::*;
use std::time::{Duration, Instant};

#[derive(Debug)]
struct Item {
    weight: i32,
    value: i32
}

impl Item {
    fn new(weight: i32, value: i32) -> Self {
        Self { weight, value }
    }

    fn randomize(&mut self) {
        self.weight = rand::thread_rng().gen_range(1..=100);
        self.value = rand::thread_rng().gen_range(1..=100);
    }
}

fn get_knapsack_items(items: &Vec<Item>, weight_limit: i32) -> Vec<Item> {
    for item in items {

    }
    todo!()
}


fn main() {

    println!("This is a {} message.", "red".red());
    println!("This is a {} message.", "green".green());
    println!("This is a {} message.", "blue".blue());

    // Combining colors and styles
    println!("This is a {} and {} message.", "bold".bold(), "italic".italic());
    println!("This is a {} message on a {} background.", "yellow".yellow(), "blue".on_blue());

    println!("\n====================================================================================================\n");

    println!("Example using rayon crate for parallelism and par_iter()\n");

    let numbers: Vec<u128> = (1..=1000000).collect(); // Create a large vector of numbers

    let time = Instant::now();
    let sum: u128 = numbers.iter().sum();
    let elapsed = time.elapsed().as_micros();
    println!("Execution time in microseconds, {}: {}", "not parallel".red(), elapsed);
    println!("The sum of numbers from 1 to 1,000,000 is: {}", sum);

    println!(" ");

    let time = Instant::now();
    let sum: u128 = numbers.par_iter().sum(); // Calculate the sum in parallel
    let elapsed = time.elapsed().as_micros();
    println!("Execution time in microseconds, {}: {}", "parallel".green(), elapsed);
    println!("The sum of numbers from 1 to 1,000,000 is: {}", sum);

    println!("\n====================================================================================================\n");

    // knapsack problem, give the items where the combined weight can't exceed limit
    // but where the value is as high as possible

    let mut knapsack: Vec<Item> = Vec::new();
    for i in 1..10 {
        let mut item = Item::new(0, 0);
        item.randomize();
        knapsack.push(item);
    }

    for (i, item) in knapsack.iter().enumerate() {
        println!("Item {}: {:?}", i, item);
    }

    // let best_items = get_knapsack_items(&knapsack);
    // println!("Best items:");
    // for (i, item) in knapsack.iter().enumerate() {
    //     println!("    Item {}: {:?}", i, item);
    // }
    // println!("Total value: {}", best_items.iter().fold(0, |acc, item| acc + item.value));


}