

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let sum_of_even_numbers_to_the_power_of_two: i32 = numbers
        .iter()                 // iter trough vec
        .filter(|&&number| number % 2 == 0) // get even numbers
        .map(|&number| number.pow(2))    // apply pow(2) on every even number
        .sum(); // add every item together
    println!("The sum of the even numbers to the power of two is {}", sum_of_even_numbers_to_the_power_of_two);

}