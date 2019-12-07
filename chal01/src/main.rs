use std::env;
use std::fs;

// Turns out integer truncation is just 'rounding down'
fn get_fuel(mass: i32) -> i32 {
    return (mass/3) - 2;
}

fn get_fuel_part2(mass: i32) -> i32 {
    let fuel = (mass/3) - 2; // Do calculation once to diff mass from fuel
    if fuel < 0 {            // b/c we dont want to add mass to fuel
        return 0;
    } else {
        return fuel + get_fuel_part2(fuel);
    }
}

// Find the fuel required for the total mass
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Opening {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Error when reading file.");

// ...mass, divide by three, round down, and subtract 2.
// fuel = round_down(mass/3) - 2
    // Process each line of mass in contents into fuel
    let mut total_fuel = 0;
    let mut total_fuel_part2 = 0;
    let lines = contents.lines(); 
    for l in lines {
        let mass: i32 = l.parse().expect("Cant parse line");
        total_fuel = total_fuel + get_fuel(mass);
        total_fuel_part2 = total_fuel_part2 + get_fuel_part2(mass);
    }
    println!("Total fuel: {}", total_fuel);

    // Part Two
    // Recursion that converges to a fuel number
    // ex. mass 14 -> 2 fuel (part 1 sol)
    //     additional_fuel(2): (2/3) -> 0
    //     total_fuel = get_fuel + additional_fuel(get_fuel)
    // ex2. mass 1969 -> 654 fuel -> 216 additional fuel ->
    //      70 additional fuel -> 21 -> 5 -> 0 -> total 966 fuel
    println!("Total fuel (part2): {}", total_fuel_part2);
}
