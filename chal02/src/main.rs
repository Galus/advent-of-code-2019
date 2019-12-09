use std::env;
use std::fs;

fn is_opcode_index(index: usize) -> bool {
    return index % 4 == 0;
}

fn str_to_i32(s: &str) -> i32 {
    let yee2 : i32 = s.trim().parse().unwrap();
    return yee2;
}

fn run_operation(opcode: i32, op1: i32, op2: i32) -> i32 {
    if opcode == 1 {
        return op1+op2;
    }
    if opcode == 2 {
        return op1*op2;
    }
    println!("Your opcode is invalid opcode: {}", opcode);
    panic!("Programming at its finest!"); 
}
fn new_program(noun: i32, verb: i32, contents: String) -> Vec<i32> {
    let mut tmp: Vec<i32> = contents.split(',')
        .map(str_to_i32)
        .collect();
    tmp[1] = noun;
    tmp[2] = verb;
    return tmp;
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let answer_str = &args[2];
    let answer: i32 = answer_str.parse().unwrap();
    println!("Opening {}", filename);
    let contents: String = fs::read_to_string(filename)
        .expect("Error when reading file.");

    println!("Looking for answer {}", answer);
    let mut program_output: i32 = 0;

    let mut noun : i32 = 0;
    loop { // loop nouns
        if noun > 99 { break; }
        let mut verb : i32 = 0;
        loop { // loop verbs
            
            if verb > 99 { break; }
            let mut index : usize = 0;
            // create new fresh program, unmolested
            let mut program: Vec<i32> = new_program(noun, verb, contents.clone());
            loop { // loop index
                // println!("index {} opcode {}", index, program[index]);
                if 99 == program[index] {
                    // println!("Finished program.");
                    // let ret = format!("{:?}", program);
                    // println!("Program\n{}",ret);
                    program_output = program[0];
                    break;
                }
                if index >= program.len() { break; }
                if is_opcode_index(index) {
                    // index == opcode_pos 
                    let result_pos : usize = program[index+3] as usize;
                    let op1_pos : usize = program[index+1] as usize;
                    let op2_pos : usize = program[index+2] as usize;
                    let opcode = program[index];
                    let op1: i32 = program[op1_pos];
                    let op2: i32 = program[op2_pos];
                    let result: i32= run_operation(opcode, op1, op2);
                    program[result_pos] = result;
                }
                index += 4;
            }
            println!("{}\t\t{}\t\t{}\t\tnoun verb output",noun, verb, program_output);
            if  program_output == answer {
                println!("Answer is noun: {} verb: {}", noun, verb);
                panic!("Done working.")
            }
            verb += 1;
        }
        noun += 1;
    }
}
