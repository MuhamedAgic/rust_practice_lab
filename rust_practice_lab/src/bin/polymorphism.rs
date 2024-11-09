

// Traits are used for shared behaviour
// They are some kind of interface
trait Vehicle {
    fn describe(&self);
    fn drive(&self);
}

// Different types of vehicles have generic vehicle behaviour (describe, drive)
// But also a bit more specific behaviour
trait LandVehicle: Vehicle {
    fn drift(&self);
}

trait SeaVehicle: Vehicle {
    fn drop_sail(&self);
}

trait AirVehicle: Vehicle {
    fn fly(&self); // air vehicles can drive and fly
}

//=================================================================
#[derive(Debug, Copy, Clone)]
struct AbstractVehicle{
    name: &'static str,
    price: f64
}

//=================================================================

struct Car {
    vehicle: AbstractVehicle, // composition
    horse_power: u16
}

impl Car {
    // constructor
    fn new(vehicle: AbstractVehicle, horse_power: u16) -> Self {
        Self { vehicle, horse_power }
    }
}

// implement trait for car type, car gets the implemented trait functions
// which means car objects can call these functions
impl Vehicle for Car {
    fn describe(&self) {
        println!("I'm a Car! I am {}, have {} horse power and cost {} dollar", self.vehicle.name, self.horse_power, self.vehicle.price);
    }

    fn drive(&self) {
        println!("{} (type Car) is now driving!", self.vehicle.name);
    }
}

// A car is not just a vehicle, but also a land vehicle
impl LandVehicle for Car {
    fn drift(&self) {
        println!("{} is now spinning and making donuts, get some new tires!", self.vehicle.name);
    }
}
//=================================================================
#[derive(Debug, Copy, Clone)]
struct Boat {
    vehicle: AbstractVehicle,
    anchor_weight: u16
}

impl Boat {
    fn new(vehicle: AbstractVehicle, anchor_weight: u16) -> Self {
        Self { vehicle, anchor_weight }
    }
}

impl Vehicle for Boat {
    fn describe(&self) {
        println!("I'm a Boat! I am anchored!");
    }
    fn drive(&self) {
        println!("{} (type Boat) is now driving at sea!", self.vehicle.name);
    }
}

impl SeaVehicle for Boat {
    fn drop_sail(&self) {
        println!("Dropping the sail for more wind and speed!")
    }
}

//=================================================================
struct AmphibiousBoat {
    boat: Boat, // and boat contains vehicle
    nr_wheels: u8
}

impl AmphibiousBoat {
    fn new(boat: Boat, nr_wheels: u8) -> Self {
        Self { boat, nr_wheels }
    }
}

impl Vehicle for AmphibiousBoat {
    fn describe(&self) {
        println!("I'm a Amphibious Boat! I am anchored!");
    }
    fn drive(&self) {
        println!("{} (type AmphibiousBoat) is now driving (or sailing)!", self.boat.vehicle.name);
    }
}

// This boat can also drive on land so it's a land vehicle and implements land vehicle functions
impl LandVehicle for AmphibiousBoat {
    fn drift(&self) {
        println!("I may be a boat, but i have {} wheels and enough power to drift!", self.nr_wheels);
    }
}

// it is also a boat and of course implements the sea vehicle trait
impl SeaVehicle for AmphibiousBoat {
    fn drop_sail(&self) {
        println!("Dropping the sail for more wind and speed!")
    }
}

//=================================================================

struct Airplane {
    vehicle: AbstractVehicle,
    wing_length: u16
}

impl Airplane {
    fn new(vehicle: AbstractVehicle, wing_length: u16) -> Self {
        Self { vehicle, wing_length }
    }
}

impl Vehicle for Airplane {
    fn describe(&self) {
        println!("I'm an airplane! My name is {}, i cost {} and my wings are {} meters tall!", self.vehicle.name, self.vehicle.price, self.wing_length);
    }

    fn drive(&self) {
        println!("{} (type Airplane) is now driving", self.vehicle.name);
    }
}

// Airplane can be driven on land
impl LandVehicle for Airplane {
    fn drift(&self) {
        println!("I'm not so good at drifting, sorry!");
    }
}

impl AirVehicle for Airplane {
    fn fly(&self) {
        println!("I'm very good at flying, see ya later!");
    }
}

//=================================================================
// Let's create a function which accepts any type which implements the Vehicle trait
// argument is a reference to anything which implements vehicle
fn test_drive_vehicle_v1(vehicle: &impl Vehicle) {
    println!("Test drive v1");
    vehicle.drive();
}

// other syntax
fn test_drive_vehicle_v2<T: Vehicle>(vehicle: &T) {
    println!("Test drive v2");
    vehicle.drive();
}

// something which implements land vehicle
fn test_drive_land_vehicle_v1(land_vehicle: &impl LandVehicle) {
    println!("Test land vehicle drive v1");
    land_vehicle.describe();
    land_vehicle.drive();
    land_vehicle.drift();
}

// something which implements sea vehicle
fn test_drive_sea_vehicle_v1(sea_vehicle: &impl SeaVehicle) {
    println!("Test sea vehicle drive v1");
    sea_vehicle.describe();
    sea_vehicle.drive();
    sea_vehicle.drop_sail();
}

// something which implements more than one trait!
fn test_drive_hybrid_vehicle_v1<T: SeaVehicle + LandVehicle>(hybrid_vehicle: &T) {
    println!("Test drive hybrid vehicle (sea and land vehicle)");
    hybrid_vehicle.describe();
    hybrid_vehicle.drive();
    hybrid_vehicle.drift();         // it can do land vehicle stuff
    hybrid_vehicle.drop_sail();     // and sea vehicle stuff
}

fn test_drive_air_vehicle_v1<T: LandVehicle + AirVehicle>(air_vehicle: &T) {
    println!("Test drive air vehicle");
    air_vehicle.describe();
    air_vehicle.drive();
    air_vehicle.drift();
    air_vehicle.fly();
}

//=================================================================
fn main() {
    let vc = AbstractVehicle {
        name: "Carry the Car",
        price: 4500.0
    };

    let vb = AbstractVehicle {
        name: "Bobby the Boat",
        price: 63850.0
    };

    let vab = AbstractVehicle {
        name: "Amber the Amphibious Boat",
        price: 132000.0
    };

    let va = AbstractVehicle {
        name: "Aimy the Airplane",
        price: 4500000.0
    };

    let car = Car::new(vc, 110);
    let boat = Boat::new(vb, 230);
    let amphibious_boat = AmphibiousBoat::new( Boat {
        vehicle: vab, anchor_weight: 3
    }, 4);
    let airplane = Airplane::new(va, 30);

    car.describe();
    car.drive();

    println!("================================================================================\n");
    println!("Test driving Vehicles");
    test_drive_vehicle_v1(&car);
    test_drive_vehicle_v1(&boat);
    test_drive_vehicle_v1(&airplane);
    test_drive_vehicle_v1(&amphibious_boat);
    println!("================================================================================\n");
    println!("Test driving Land Vehicles");
    // test_drive_land_vehicle_v1(&boat);   // Not a land vehicle!
    test_drive_land_vehicle_v1(&car);
    test_drive_land_vehicle_v1(&airplane);
    test_drive_land_vehicle_v1(&amphibious_boat);
    println!("================================================================================\n");
    println!("Test driving Sea Vehicles");
    // test_drive_sea_vehicle_v1(&car);         // Not a sea vehicle!
    // test_drive_sea_vehicle_v1(&airplane);    // Not a sea vehicle!
    test_drive_sea_vehicle_v1(&amphibious_boat);
    test_drive_sea_vehicle_v1(&boat);
    println!("================================================================================\n");
    println!("Test driving Vehicles which can be driven on land and sea");
    // test_drive_hybrid_vehicle_v1(&car);          // Not a land AND sea vehicle!
    // test_drive_hybrid_vehicle_v1(&airplane);     // Not a land AND sea vehicle!
    // test_drive_hybrid_vehicle_v1(&boat);         // Not a land AND sea vehicle!
    test_drive_hybrid_vehicle_v1(&amphibious_boat);
    println!("================================================================================\n");
    println!("Test driving Vehicles which can fly");
    // test_drive_air_vehicle_v1(&car);             // Not a land AND air vehicle!
    // test_drive_air_vehicle_v1(&boat);            // Not a land AND air vehicle!
    // test_drive_air_vehicle_v1(&amphibious_boat); // Not a land AND air vehicle!
    test_drive_air_vehicle_v1(&airplane);

}