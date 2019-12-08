use std::env;
use std::fs;

fn is_opcode(index: usize) -> bool {
    (index+1) & 4 == 0
}

fn str_to_i32(s: &str) -> i32 {
    let yee : i32 = s.trim().parse().unwrap();
    return yee;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("Opening {}", filename);
    let contents = fs::read_to_string(filename)
        .expect("Error when reading file.");

    let mut program: Vec<i32> = contents.split(',')
        .map(str_to_i32)
        .collect();
    println!("{}", format!("{:?}", program));

    let mut index : usize = 0;
    loop {
        if index + 1 > program.len() { break }
        if is_opcode(index) {
            if 1 == program[index] {
                program[index+3] = program[index+1] + program[index+2];
            }
            if 2 == program[index] {
                program[index+3] = program[index+1] * program[index+2]
            }
            if 99 == program[index] {
                let ret = format!("{:?}", program);
                println!("Program\n{}",ret);
            }
        }
        index += 1;
    }
    
    for (index, opcode) in program.iter_mut().enumerate() {
        if is_opcode(index) {
            if 1 == *opcode {
                program[index+3] = program[index+1] + program[index+2]
            }
            if *opcode == 2 {
                // program[index+3] = program[index+1] * program[index+2]
            }
            if *opcode == 99 {
                // let ret = format!("{:?}", program);
                // println!("Program\n{}",ret);
            }
        }
    }
}
