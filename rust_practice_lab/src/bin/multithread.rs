use std::error::Error;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use itertools::Itertools;
use rayon::prelude::*;
use colored::*;
use rand::Rng;
use rust_practice_lab::knapsack::*;


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
    const KNAPSACK_ITEM_COUNT: u8 = 24;
    const KNAPSACK_WEIGHT_LIMIT: i32 = 400;

    let mut knapsack: Vec<Item> = Vec::new();
    for i in 0..KNAPSACK_ITEM_COUNT {
        let mut item = Item::new(0, 0);
        item.randomize();
        knapsack.push(item.deref().to_owned());
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
    for (i, item) in result_two.iter().enumerate() {
        println!("    Item {}: {:?}", i, item);
    }
    println!("Total weight: {}", result_two.iter().fold(0, |acc, item| acc + item.weight));
    println!("Total value: {}", result_two.iter().fold(0, |acc, item| acc + item.value));
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
    for (i, item) in result_two.iter().enumerate() {
        println!("    Item {}: {:?}", i, item);
    }
    println!("Total weight: {}", result_two.iter().fold(0, |acc, item| acc + item.weight));
    println!("Total value: {}", result_two.iter().fold(0, |acc, item| acc + item.value));
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