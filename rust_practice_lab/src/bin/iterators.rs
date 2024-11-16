mod polymorphism;               // rs file
pub use polymorphism::poly;     // file_name::mod_name
use polymorphism::Vehicles;

fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let sum_of_even_numbers_to_the_power_of_two: i32 = numbers
        .iter()                 // iter trough vec
        .filter(|&&number| number % 2 == 0) // get even numbers
        .map(|&number| number.pow(2))    // apply pow(2) on every even number
        .sum(); // add every item together
    println!("The sum of the even numbers to the power of two is {}", sum_of_even_numbers_to_the_power_of_two);

    println!("\n====================================================================================================\n");

    let boat = poly::Boat::new(
        poly::AbstractVehicle {name: "Boat", price: 67000.0},
        56
    );
    let hybrid_boat = poly::AmphibiousBoat::new(
        poly::Boat::new(
            poly::AbstractVehicle {name: "Boat", price: 67000.0}, 67
        ),
        4
    );
    let airplane = poly::Airplane::new(
        poly::AbstractVehicle {name: "Airplane", price: 6700000.0},
        30
    );
    let car = poly::Car::new(
        poly::AbstractVehicle {name: "SuperFast", price: 64500.0},
        300
    );

    // A vec with objects that implement the vehicle trait
    let vehicle_vec: Vec<polymorphism::Vehicles> = vec![
        Vehicles::Car(car),
        Vehicles::Boat(boat),
        Vehicles::AmphibiousBoat(hybrid_boat),
        Vehicles::Airplane(airplane)
    ];

    println!("Collection of vehicles: ");
    for vehicle in &vehicle_vec {
        println!("    {:?}", vehicle);
    }
    println!("");

    // get all objects which are not cars
    let only_cars: Vec<&poly::Car> = vehicle_vec
        .iter()
        .filter_map(|vehicle|
            // for every vehicle, check if it's a car, if so return it
            if let Vehicles::Car(car) = vehicle {
                Some(car)
            } else {
                None
            }
        )
        .collect();

    println!("Cars collection from vehicles: ");
    for car in &only_cars {
        println!("    {:?}", car);
    }
    println!("");

    println!("\n====================================================================================================\n");

    // analyze addresses
    println!("1. Address of Vehicles enum in vehicle_vec: {:p}", &vehicle_vec[0]);
    println!("2a. Address of car within Vehicles::Car variant: {:p}", if let Vehicles::Car(car) = &vehicle_vec[0] { car } else { panic!("Not a car") });
    println!("2b. Address of car in Vehicles::Car (as raw pointer): {:p}", if let Vehicles::Car(car) = &vehicle_vec[0] { car as *const _ } else { panic!("Not a car") });
    println!("3a. Address that the reference in only_cars points to: {:p}", only_cars[0]);
    println!("3b. Address of reference to Car stored in only_cars: {:p}", &only_cars[0]);

    println!("\n====================================================================================================\n");

    // filter only cheap vehicles
    let cheap_vehicles: Vec<&polymorphism::Vehicles> = vehicle_vec
        .iter()
        .filter(|&vehicle| match vehicle {
            Vehicles::AmphibiousBoat(hybrid_boat) => hybrid_boat.boat.vehicle.price < 65000.0,
            Vehicles::Airplane(airplane) => airplane.vehicle.price < 65000.0,
            Vehicles::Boat(boat) => boat.vehicle.price < 65000.0,
            Vehicles::Car(car) => car.vehicle.price < 65000.0,
            _ => false
        })
        .collect();

    println!("Cheap collection of vehicles: ");
    for cheap_vehicle in &cheap_vehicles {
        println!("    {:?}", cheap_vehicle);
    }

    println!("\n====================================================================================================\n");

    // get sum of the prize of all vehicles and return that sum
    let vehicles_prizes_sum = vehicle_vec
        .iter()
        .fold(0.0, |acc, vehicle| {
            let price = match vehicle {
            Vehicles::AmphibiousBoat(hybrid_boat) => hybrid_boat.boat.vehicle.price,
            Vehicles::Airplane(airplane) => airplane.vehicle.price,
            Vehicles::Boat(boat) => boat.vehicle.price,
            Vehicles::Car(car) => car.vehicle.price,
            _ => 0.0
        };
        acc + price
        });

    println!("The sum of all the vehicle prizes is: {}", vehicles_prizes_sum);

    println!("\n====================================================================================================\n");

    // combine filter on car and then sum all the prices together in one operation
    let car_sum_prices = vehicle_vec
        .iter()
        .filter_map(|vehicle| {
            if let Vehicles::Car(car) = vehicle {
                Some(car)
            }
            else {
                None
            }
        })
        .fold(0.0, |acc, car| acc + car.vehicle.price); // here, only cars are left

    println!("The sum of all the car prizes is: {}", car_sum_prices);

    println!("\n====================================================================================================\n");



}