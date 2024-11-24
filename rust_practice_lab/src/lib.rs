
use cxx::*;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::thread;
use itertools::Itertools;
use rayon::prelude::*;
use colored::*;
use rand::Rng;


#[cxx::bridge]
mod to_cpp {
    extern "Rust" {
        fn hello();
        fn counting_until(limit: i32);
        fn fibonacci_recursive(n: i64) -> i64;
        fn fibonacci_iterative(n: i64) -> i64;
    }
}

pub fn hello() {
    println!("Hello from Rust!");
}

pub fn counting_until(limit: i32) {
    if limit > 50000 {
        println!("Limit too high! Choose something below 50k!");
    }
    for i in 1..=limit {
        println!("{}", i);
    }
}

pub fn fibonacci_recursive(n: i64) -> i64 {
    if n < 2 {
        return n;
    }
    return fibonacci_recursive(n - 1) + fibonacci_recursive( n - 2);
}

pub fn fibonacci_iterative(n: i64) -> i64 {
    let mut first_number: i64 = 0;
    let mut second_number: i64 = 0;
    let mut current_number: i64 = 1;

    let mut i: i64 = 1;

    while i < n {
        first_number = second_number;
        second_number = current_number;
        current_number = first_number + second_number;
        i = i + 1;
    }
    return current_number;
}



pub mod knapsack {
    use std::cmp::Ordering;
    use super::*;

    #[derive(Debug, Copy, Clone)]
    pub struct Item {
        pub weight: i32,
        pub value: i32
    }

    impl Item {
        pub fn new(weight: i32, value: i32) -> Box<Item> {
            Box::new(Item { weight, value })
        }

        pub fn randomize(&mut self) {
            self.weight = rand::thread_rng().gen_range(1..=100);
            self.value = rand::thread_rng().gen_range(1..=100);
        }
    }

    pub fn get_knapsack_items(items: &mut Vec<Item>, weight_limit: i32) -> Vec<Item> {
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

    pub fn get_knapsack_items_par_threads(items: &Vec<Item>, weight_limit: i32) -> Vec<Item> {
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

        Arc::try_unwrap(knapsack_items).unwrap().into_inner().unwrap()
    }


    pub fn get_knapsack_items_par_iter(items: &Vec<Item>, weight_limit: i32) -> Vec<Item> {
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

        Arc::try_unwrap(knapsack_items).unwrap().into_inner().unwrap()
    }

    pub fn get_suboptimal_knapsack_items_val_weight_ratio(items: &mut Vec<Item>, weight_limit: i32) -> Vec<Item> {
        let mut sum = 0;
        items
            .iter()
            .map(|item| {
                let value_weight_ratio = item.value as f64 / item.weight as f64;
                (item, value_weight_ratio)
            })
            .sorted_by(|(item_a, val_weight_ratio_a), (item_b, val_weight_ratio_b)| {
                val_weight_ratio_b.partial_cmp(&val_weight_ratio_a).unwrap_or(Ordering::Equal)
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

    pub fn test_parallel_knapsack_threads(items: &Vec<Item>, weight_limit: i32) -> Vec<Item> {
        println!("Using {} execution and {} inside function", "parallel".green(), "parallel processing with spawning threads".green());

        const KNAPSACK_ITEM_COUNT: u8 = 20;

        let knapsack_one = items.clone();
        let knapsack_two = items.clone();

        let knapsack_first = std::thread::spawn(
            move || {
                let mut knapsack_clone = knapsack_one;
                let best_items = get_knapsack_items_par_threads(&mut knapsack_clone, weight_limit);
                best_items
            }
        );

        let knapsack_second = std::thread::spawn(
            move || {
                let mut knapsack_clone = knapsack_two;
                let best_items = get_knapsack_items_par_threads(&mut knapsack_clone, weight_limit);
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

        result_two
    }


    // extern "Rust" {
    //     fn new(weight: i32, value: i32) -> Box<Item>;
    //     fn randomize(item: &mut Item);
    //     fn get_knapsack_items(items: &mut Vec<Item>, weight_limit: i32) -> Vec<Item>;
    //     fn get_knapsack_items_par_threads(items: &Vec<Item>, weight_limit: i32) -> Vec<Item>;
    //     fn get_knapsack_items_par_iter(items: &Vec<Item>, weight_limit: i32) -> Vec<Item>;
    //     fn get_suboptimal_knapsack_items_val_weight_ratio(items: &mut Vec<Item>, weight_limit: i32) -> Vec<Item>;
    //     fn test_parallel_knapsack_threads(items: &Vec<Item>, weight_limit: i32) -> Vec<Item>;
    // }
}

