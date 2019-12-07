use std::env;
use std::fs;
use std::f32;

// Turns out integer truncation is just 'rounding down'
fn get_fuel(mass: i32) -> i32 {
    return (mass/3) - 2;
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
    let mut lines = contents.lines(); 
    for l in lines {
        let mass: i32 = l.parse().expect("Cant parse line");
        total_fuel = total_fuel + get_fuel(mass);
    }
    println!("Total fuel: {}", total_fuel);

}
