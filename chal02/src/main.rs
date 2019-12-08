use std::env;
use std::fs;

fn is_opcode_index(index: usize) -> bool {
    return index % 4 == 0;
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
    //println!("{}", format!("{:?}", program));

    let mut index : usize = 0;
    loop {
        if 99 == program[index] {
            println!("Finished program.");
            let ret = format!("{:?}", program);
            println!("Program\n{}",ret);
            break;
        }
        if index >= program.len() { break }
        if is_opcode_index(index) {
            let result_pos : usize = program[index+3] as usize;
            let op1 : usize = program[index+1] as usize;
            let op2 : usize = program[index+2] as usize;
            //println!("{} is an opcode", index);
            if 1 == program[index] {
                println!("{}:Op p[{}]+p[{}] -> {}+{} replacing p[{}]-> {} with {}",
                    index,
                    op1,
                    op2,
                    program[op1],
                    program[op2],
                    result_pos,
                    program[result_pos],
                    program[op1]+program[op2]);
                println!("OLD: {}", format!("{:?}", program));
                program[result_pos] = program[op1] + program[op2];
                println!("NEW: {}", format!("{:?}", program));
            }
            if 2 == program[index] {
                println!("{}:Op p[{}]*p[{}] -> {}*{} replacing p[{}]-> {} with {}",
                    index,
                    op1,
                    op2,
                    program[op1],
                    program[op2],
                    result_pos,
                    program[result_pos],
                    program[op1]*program[op2]);
                println!("OLD: {}", format!("{:?}", program));
                program[result_pos] = program[op1] * program[op2];
                println!("NEW: {}", format!("{:?}", program));
            }
        }
            println!("Incrementing index: {}", index);
            index += 4;
            println!(" index: {}", index);
    }
}
