/**
 * --- Day 8: Handheld Halting ---
 * 
 * Your flight to the major airline hub reaches cruising altitude without incident. While you consider checking the in-flight menu for one of those drinks that come with a little umbrella, you are interrupted by the kid sitting next to you.
 * Their handheld game console won't turn on! They ask if you can take a look.
 * You narrow the problem down to a strange infinite loop in the boot code (your puzzle input) of the device. You should be able to fix it, but first you need to be able to run the code in isolation.
 * The boot code is represented as a text file with one instruction per line of text. Each instruction consists of an operation (acc, jmp, or nop) and an argument (a signed number like +4 or -20).
 * 
 *     acc increases or decreases a single global value called the accumulator by the value given in the argument. For example, acc +7 would increase the accumulator by 7. The accumulator starts at 0. After an acc instruction, the instruction immediately below it is executed next.
 *     jmp jumps to a new instruction relative to itself. The next instruction to execute is found using the argument as an offset from the jmp instruction; for example, jmp +2 would skip the next instruction, jmp +1 would continue to the instruction immediately below it, and jmp -20 would cause the instruction 20 lines above to be executed next.
 *     nop stands for No OPeration - it does nothing. The instruction immediately below it is executed next.
 * 
 * For example, consider the following program:
 * 
 * nop +0
 * acc +1
 * jmp +4
 * acc +3
 * jmp -3
 * acc -99
 * acc +1
 * jmp -4
 * acc +6
 * 
 * These instructions are visited in this order:
 * 
 * nop +0  | 1
 * acc +1  | 2, 8(!)
 * jmp +4  | 3
 * acc +3  | 6
 * jmp -3  | 7
 * acc -99 |
 * acc +1  | 4
 * jmp -4  | 5
 * acc +6  |
 * 
 * First, the nop +0 does nothing. Then, the accumulator is increased from 0 to 1 (acc +1) and jmp +4 sets the next instruction to the other acc +1 near the bottom. After it increases the accumulator from 1 to 2, jmp -4 executes, setting the next instruction to the only acc +3. It sets the accumulator to 5, and jmp -3 causes the program to continue back at the first acc +1.
 * This is an infinite loop: with this sequence of jumps, the program will run forever. The moment the program tries to run any instruction a second time, you know it will never terminate.
 * Immediately before the program would run an instruction a second time, the value in the accumulator is 5.
 * Run your copy of the boot code. Immediately before any instruction is executed a second time, what value is in the accumulator?
 * 
 * --- Part Two ---
 * 
 * After some careful analysis, you believe that exactly one instruction is corrupted.
 * Somewhere in the program, either a jmp is supposed to be a nop, or a nop is supposed to be a jmp. (No acc instructions were harmed in the corruption of this boot code.)
 * The program is supposed to terminate by attempting to execute an instruction immediately after the last instruction in the file. By changing exactly one jmp or nop, you can repair the boot code and make it terminate correctly.
 * For example, consider the same program from above:
 * 
 * nop +0
 * acc +1
 * jmp +4
 * acc +3
 * jmp -3
 * acc -99
 * acc +1
 * jmp -4
 * acc +6
 * 
 * If you change the first instruction from nop +0 to jmp +0, it would create a single-instruction infinite loop, never leaving that instruction. If you change almost any of the jmp instructions, the program will still eventually find another jmp instruction and loop forever.
 * However, if you change the second-to-last instruction (from jmp -4 to nop -4), the program terminates! The instructions are visited in this order:
 * 
 * nop +0  | 1
 * acc +1  | 2
 * jmp +4  | 3
 * acc +3  |
 * jmp -3  |
 * acc -99 |
 * acc +1  | 4
 * nop -4  | 5
 * acc +6  | 6
 * 
 * After the last instruction (acc +6), the program terminates by attempting to run the instruction below the last instruction in the file. With this change, after the program terminates, the accumulator contains the value 8 (acc +1, acc +1, acc +6).
 * 
 * Fix the program so that it terminates normally by changing exactly one jmp (to nop) or nop (to jmp). What is the value of the accumulator after the program terminates?
 */

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let code = parse_code(read_in_file("src/code.txt"));

    println!("---- PART 1 ----");
    debug_code(code.clone());

    println!("\n\n---- PART 2 ----");
    correct_code(code.clone());
}

/// Read in lines of a file to a vector
/// 
/// # Arguments
/// 
/// * `filename` - String containing the filename
fn read_in_file(filename: &str) -> Vec<String> {
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader.lines().flatten().collect()
}

/// Parse the code from a vector of strings and return a vector of Instructions
/// Each line is of the form [instr] [num]
fn parse_code(lines: Vec<String>) -> Vec<Instruction> {
    let mut instructions = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let temp: Vec<&str> = line.split_whitespace().collect();
        let instr = String::from(temp[0]);
        let value = temp[1].parse::<i64>().unwrap();
        let line_num = i as u64;
        let instruction = Instruction{
            line_num,
            instr,
            value,
        };
        instructions.push(instruction);
    }
    instructions
}


/// A structure to store the current memory state of the program
/// 
/// # Parameters
/// 
/// * `current_line` the current line of execution
/// * `accumulator` the current value of the accumulator (before execution of current_line)
#[derive(Debug, Clone)]
struct Memory {
    current_line: u64,
    accumulator: i64,
}

/// A structure containing a code instruction
/// 
/// # Parameters
/// 
/// * `line_num` the line number of the instruction used for debug purposes
/// * `instr` a string containing the instruction
/// * `value` the number to apply to the instruction
#[derive(Debug, Clone)]
struct Instruction {
    line_num: u64,
    instr: String,
    value: i64,
}

impl PartialEq for Instruction {
    /// We consider two instructions equal if they have the same line number
    fn eq(&self, other: &Instruction) -> bool {
        self.line_num == other.line_num
    }
}

/// Run through the code and print the memory state when we reach an exception or the program end
fn debug_code(code: Vec<Instruction>) -> Memory {
    let mut memory = Memory{
        current_line: 0,
        accumulator: 0
    };

    let mut history: Vec<u64> = Vec::new();
    
    loop {
        history.push(memory.current_line);
        if let Some(instr) = code.get(memory.current_line as usize) {
            // RUN CODE
            memory = run_instruction(instr, memory);
        } else {
            // PROGRAM ENDED
            println!("END: Memory state is {:?}", memory);
            return memory;
        }

        if history.contains(&memory.current_line) {
            println!("ERROR: Infinite Loop Reached memory state is {:?}", memory);
            return memory
        }
    }  
}

/// Attempt to correct one nop or jmp instruction so the code runs to completion
/// 
/// # Parameters
/// 
/// 
fn correct_code(mut code: Vec<Instruction>) -> Memory{
    
    let mut memory = Memory{
        current_line: 0,
        accumulator: 0
    };
    
    let mut history: Vec<Instruction> = Vec::new();
    let mut memory_snapshot = None;
    let mut history_snapshot = None;
    let code_snapshot = code.clone();
    
    loop {
        let code_copy = code.clone();
        if let Some(line) = code_copy.get(memory.current_line as usize) {
            if history.contains(line) {
                // Roll back to previous snapshot if exists
                if let Some(memory_old) = memory_snapshot {
                    memory = memory_old;
                    history = history_snapshot.unwrap();
                    code = code_snapshot.clone();
                }
                // Find the next instruction we can try to modify
                loop {
                    if let Some(instr) = history.pop() {
                        // We modify the memory to roll back the state as we pop off instructions
                        match instr.instr.as_str() {
                            "jmp" => {
                                code.get_mut(instr.line_num as usize).unwrap().instr = String::from("nop");
                                memory.current_line = instr.line_num;
                                break
                            }
                            "nop" => {
                                code.get_mut(instr.line_num as usize).unwrap().instr = String::from("jmp");
                                memory.current_line = instr.line_num;
                                break;
                            }
                            "acc" => {
                                memory.accumulator -= instr.value;
                            }
                            _ => {
                                println!("ERROR: Found unsupported instruction");
                                return memory
                            }
                        }
                    } else {
                        println!("ERROR: No more lines to correct");
                        return memory
                    }
                }
                // Save snapshot
                memory_snapshot = Some(memory.clone());
                history_snapshot = Some(history.clone());
                continue;
            }
            history.push(line.clone());
            memory = run_instruction(line, memory);
        } else {
            // PROGRAM ENDED
            println!("PROGRAM END: {:?}", memory);
            return memory
        }
    }
}

/// Run an instruction. Currently accepts the following instructions:
/// * acc [value] - add value to the accumulator, goto the next line
/// * jmp [value] - jump value lines in the program
/// * nop [value] - perform no operation, goto the next line
/// 
/// # Parameters
/// 
/// * `instr` the instruction to execute
/// * `mem` the memory to manupulate
/// 
/// # Returns
/// 
/// * the new memory state
fn run_instruction(instr: &Instruction, mem: Memory) -> Memory {
    match instr.instr.as_str() {
        "acc" => Memory{ current_line: mem.current_line + 1, accumulator: mem.accumulator + instr.value},
        "jmp" => Memory{ current_line: (mem.current_line as i64 + instr.value) as u64, accumulator: mem.accumulator},
        "nop" => Memory{ current_line: mem.current_line + 1, accumulator: mem.accumulator},
        _ => Memory{ current_line: mem.current_line, accumulator: mem.accumulator},        
    }
}