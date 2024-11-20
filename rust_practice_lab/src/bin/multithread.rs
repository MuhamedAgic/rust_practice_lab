use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use itertools::Itertools;
use rayon::prelude::*;
use colored::*;
use rand::Rng;


#[derive(Debug, Copy, Clone)]
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

fn get_knapsack_items(items: &mut Vec<Item>, weight_limit: i32) -> Vec<Item> {
    let mut sum = 0;
    let max_possible_combinations = items
        .iter()
        .sorted_by(|a, b| a.weight.cmp(&b.weight))
        .take_while(|&&item| {
            sum += item.weight;
            sum < weight_limit
        })
        .count();

    println!("\nMax {} possible combinations\n", max_possible_combinations);

    let mut highest_combined_value = 0;
    let mut knapsack_items: Vec<Item> = Vec::new();
    for i in 1..=max_possible_combinations {
        for combination in items.iter().combinations(i) {
            let current_combined_value: i32 = combination.iter().map(|item| item.value).sum();
            let current_combined_weight: i32 = combination.iter().map(|item| item.weight).sum();
            if current_combined_value > highest_combined_value  && current_combined_weight <= weight_limit {
                highest_combined_value = current_combined_value;
                knapsack_items = combination.into_iter().cloned().collect();
            }
        }
    }
    knapsack_items
}


fn get_knapsack_items_par_threads(items: &Vec<Item>, weight_limit: i32) -> Arc<Mutex<Vec<Item>>> {
    let items = Arc::new(items.clone());

    let mut sum = 0;
    let max_possible_combinations = items
        .iter()
        .sorted_by(|a, b| a.weight.cmp(&b.weight))
        .take_while(|&&item| {
            sum += item.weight;
            sum < weight_limit
        })
        .count();

    println!("\nMax {} possible combinations\n", max_possible_combinations);

    let highest_combined_value: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut knapsack_items: Arc<Mutex<Vec<Item>>> = Arc::new(Mutex::new(Vec::<Item>::new()));

    let mut handles = vec![];

    for i in 1..=max_possible_combinations {
        // atomic reference count clone, for every thread
        let items = Arc::clone(&items);
        let highest_combined_value = Arc::clone(&highest_combined_value);
        let knapsack_items = Arc::clone(&knapsack_items);

        // make thread and execute
        let thread = std::thread::spawn(move || {
            for combination in items.iter().combinations(i) {
                let current_combined_value: i32 = combination.iter().map(|item| item.value).sum();
                let current_combined_weight: i32 = combination.iter().map(|item| item.weight).sum();
                let mut highest_combined_value = highest_combined_value.lock().unwrap();
                if current_combined_value > *highest_combined_value  && current_combined_weight <= weight_limit {
                    *highest_combined_value = current_combined_value;
                    let mut knapsack_items = knapsack_items.lock().unwrap();
                    *knapsack_items = combination.into_iter().cloned().collect();
                }
            }
            highest_combined_value
        });

        handles.push(thread);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    knapsack_items
}


fn get_knapsack_items_par_iter(items: &Vec<Item>, weight_limit: i32) -> Arc<Mutex<Vec<Item>>> {
    let items = Arc::new(items.clone());

    let mut sum = 0;
    let max_possible_combinations = items
        .iter()
        .sorted_by(|a, b| a.weight.cmp(&b.weight))
        .take_while(|&&item| {
            sum += item.weight;
            sum < weight_limit
        })
        .count();

    println!("\nMax {} possible combinations\n", max_possible_combinations);

    let highest_combined_value: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));
    let mut knapsack_items: Arc<Mutex<Vec<Item>>> = Arc::new(Mutex::new(Vec::<Item>::new()));

    (1..=max_possible_combinations).into_par_iter().for_each(|i| {
        // atomic reference count clone, for every parallel loop
        let items = Arc::clone(&items);
        let highest_combined_value = Arc::clone(&highest_combined_value);
        let knapsack_items = Arc::clone(&knapsack_items);

        for combination in items.iter().combinations(i) {
            let current_combined_value: i32 = combination.iter().map(|item| item.value).sum();
            let current_combined_weight: i32 = combination.iter().map(|item| item.weight).sum();
            let mut highest_combined_value = highest_combined_value.lock().unwrap();
            if current_combined_value > *highest_combined_value  && current_combined_weight <= weight_limit {
                *highest_combined_value = current_combined_value;
                let mut knapsack_items = knapsack_items.lock().unwrap();
                *knapsack_items = combination.into_iter().cloned().collect();
            }
        }
    });

    knapsack_items
}

fn get_suboptimal_knapsack_items_val_weight_ratio(items: &mut Vec<Item>, weight_limit: i32) -> Vec<Item> {
    let mut sum = 0;
    items
        .iter()
        .map(|item| {
            let value_weight_ratio = item.value as f64 / item.weight as f64;
            (item, value_weight_ratio)
        })
        .sorted_by(|(item_a, val_weight_ratio_a), (item_b, val_weight_ratio_b)| {
            val_weight_ratio_b.partial_cmp(&val_weight_ratio_a).unwrap()
        })
        .take_while(|&(item, value_weight_ratio)| {
            if sum + item.weight <= weight_limit {
                sum += item.weight;
                true
            }
            else {
                false
            }
        })
        .map(|(item, _)| item.to_owned())
        .collect()
}


fn main() -> Result<(), Box<dyn Error>> {

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
    const KNAPSACK_ITEM_COUNT: u8 = 25;
    const KNAPSACK_WEIGHT_LIMIT: i32 = 400;

    let mut knapsack: Vec<Item> = Vec::new();
    for i in 0..KNAPSACK_ITEM_COUNT {
        let mut item = Item::new(0, 0);
        item.randomize();
        knapsack.push(item);
    }

    for (i, item) in knapsack.iter().enumerate() {
        println!("Item {}: {:?}", i, item);
    }

    println!("\n====================================================================================================\n");

    println!("\nUsing {} execution", "non parallel".red());

    let start = Instant::now();
    let best_items = get_knapsack_items(&mut knapsack, KNAPSACK_WEIGHT_LIMIT);
    let best_items_again = get_knapsack_items(&mut knapsack, KNAPSACK_WEIGHT_LIMIT);
    let elapsed = start.elapsed().as_secs();
    println!("\nBest items:");
    for (i, item) in best_items.iter().enumerate() {
        println!("    Item {}: {:?}", i, item);
    }
    println!("Total weight: {}", best_items.iter().fold(0, |acc, item| acc + item.weight));
    println!("Total value: {}", best_items.iter().fold(0, |acc, item| acc + item.value));
    println!("Execution time in seconds, {}: {}", "not parallel".red(), elapsed);

    println!("\n====================================================================================================\n");

    println!("Using {} execution", "parallel".green());

    let knapsack_one = knapsack.clone();
    let knapsack_two = knapsack.clone();

    let knapsack_first = std::thread::spawn(
        move || {
            let mut knapsack_clone = knapsack_one;
            let best_items = get_knapsack_items(&mut knapsack_clone, KNAPSACK_WEIGHT_LIMIT);
            best_items
        }
    );

    let knapsack_second = std::thread::spawn(
        move || {
            let mut knapsack_clone = knapsack_two;
            let best_items = get_knapsack_items(&mut knapsack_clone, KNAPSACK_WEIGHT_LIMIT);
            best_items
        }
    );
    let start = Instant::now();
    let result_one = knapsack_first.join().unwrap();
    let result_two = knapsack_second.join().unwrap();
    let elapsed = start.elapsed().as_secs();
    println!("\nBest items:");
    for (i, item) in result_two.iter().enumerate() {
        println!("    Item {}: {:?}", i, item);
    }
    println!("Total weight: {}", result_two.iter().fold(0, |acc, item| acc + item.weight));
    println!("Total value: {}", result_two.iter().fold(0, |acc, item| acc + item.value));
    println!("Execution time in seconds, {}: {}", "parallel".green(), elapsed);

    println!("\n====================================================================================================\n");

    println!("Using {} execution and {} inside function", "parallel".green(), "parallel processing with spawning threads".green());

    let knapsack_one = knapsack.clone();
    let knapsack_two = knapsack.clone();

    let knapsack_first = std::thread::spawn(
        move || {
            let mut knapsack_clone = knapsack_one;
            let best_items = get_knapsack_items_par_threads(&mut knapsack_clone, KNAPSACK_WEIGHT_LIMIT);
            best_items
        }
    );

    let knapsack_second = std::thread::spawn(
        move || {
            let mut knapsack_clone = knapsack_two;
            let best_items = get_knapsack_items_par_threads(&mut knapsack_clone, KNAPSACK_WEIGHT_LIMIT);
            best_items
        }
    );
    let start = Instant::now();
    let result_one = knapsack_first.join().unwrap();
    let result_two = knapsack_second.join().unwrap();
    let elapsed = start.elapsed().as_secs();
    println!("\nBest items:");
    for (i, item) in result_two.lock().unwrap().iter().enumerate() {
        println!("    Item {}: {:?}", i, item);
    }
    println!("Total weight: {}", result_two.lock().unwrap().iter().fold(0, |acc, item| acc + item.weight));
    println!("Total value: {}", result_two.lock().unwrap().iter().fold(0, |acc, item| acc + item.value));
    println!("Execution time in seconds, {}: {}", "parallel".green(), elapsed);

    println!("\n====================================================================================================\n");

    println!("Using {} execution and {} inside function", "parallel".green(), "parallel processing with rayon par iter".green());

    let knapsack_one = knapsack.clone();
    let knapsack_two = knapsack.clone();

    let knapsack_first = std::thread::spawn(
        move || {
            let mut knapsack_clone = knapsack_one;
            let best_items = get_knapsack_items_par_iter(&mut knapsack_clone, KNAPSACK_WEIGHT_LIMIT);
            best_items
        }
    );

    let knapsack_second = std::thread::spawn(
        move || {
            let mut knapsack_clone = knapsack_two;
            let best_items = get_knapsack_items_par_iter(&mut knapsack_clone, KNAPSACK_WEIGHT_LIMIT);
            best_items
        }
    );
    let start = Instant::now();
    let result_one = knapsack_first.join().unwrap();
    let result_two = knapsack_second.join().unwrap();
    let elapsed = start.elapsed().as_secs();
    println!("\nBest items:");
    for (i, item) in result_two.lock().unwrap().iter().enumerate() {
        println!("    Item {}: {:?}", i, item);
    }
    println!("Total weight: {}", result_two.lock().unwrap().iter().fold(0, |acc, item| acc + item.weight));
    println!("Total value: {}", result_two.lock().unwrap().iter().fold(0, |acc, item| acc + item.value));
    println!("Execution time in seconds, {}: {}", "parallel".green(), elapsed);

    println!("\n====================================================================================================\n");

    println!("Using value weight ratio calculation solution");

    let knapsack_one = knapsack.clone();
    let knapsack_two = knapsack.clone();

    let knapsack_first = std::thread::spawn(
        move || {
            let mut knapsack_clone = knapsack_one;
            let best_items = get_suboptimal_knapsack_items_val_weight_ratio(&mut knapsack_clone, KNAPSACK_WEIGHT_LIMIT);
            best_items
        }
    );

    let knapsack_second = std::thread::spawn(
        move || {
            let mut knapsack_clone = knapsack_two;
            let best_items = get_suboptimal_knapsack_items_val_weight_ratio(&mut knapsack_clone, KNAPSACK_WEIGHT_LIMIT);
            best_items
        }
    );
    let start = Instant::now();
    let result_one = knapsack_first.join().unwrap();
    let result_two = knapsack_second.join().unwrap();
    let elapsed = start.elapsed().as_micros();
    println!("\nBest items according to suboptimal knapsack solution!:");
    for (i, item) in result_two.iter().enumerate() {
        println!("    Item {}: {:?}", i, item);
    }
    println!("Total weight: {}", result_two.iter().fold(0, |acc, item| acc + item.weight));
    println!("Total value: {}", result_two.iter().fold(0, |acc, item| acc + item.value));
    println!("Execution time in microseconds, {}: {}", "parallel".green(), elapsed);

    println!("\n====================================================================================================\n");

    Ok(())
}