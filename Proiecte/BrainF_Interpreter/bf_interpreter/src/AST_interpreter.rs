/*
    This implementation features an Abstract Syntax Tree. Our parser() function builds this Tree and tries to optimize the number 
    of operation and idioms. The interpret() function then takes this AST with the optimized operations and modifies the TAPE at runtime.

    We covered the next topics:

        At Parse Time:
            - Syntax Errors like unmatched loop paranthesis.
            - Warnings for loops that might never end like [+].
            - Optimization: Run Length Encoding: Instead of parsing into the the multiple instructions of the same type, we compress 
              them into a single instruction with the amount number of repetead consecutive instruction. This applies for +, -, <, and >.
    
        At Runtime: 
            - Optimization: Trying to find loops that respects a mathematical formula and instead of repeating instruction we change the tape
              accordingly to that formula (i.e idioms)
              Currently implemented: [-] is actually set the tape block to 0.
*/
pub mod ast_interpreter{
    use colored::Colorize;

    #[derive(Debug, PartialEq)]
    pub enum Instruction {
        Inc(u32),
        Dec(u32),
        MoveLeft(u32),
        MoveRight(u32),
        PrintToAscii,
        Loop(Vec<Instruction>)
    }

    const TAPE_SIZE: usize = 30_000;

    pub fn parser(code: &str) -> Result<Vec<Instruction>, String>{

        let mut stack: Vec<Vec<Instruction>>  = vec![Vec::new()];
        let mut byte_counter = 0;

        for byte in code.bytes() {
            byte_counter += 1;
            
            match byte {
                b'<' => {
                    let last_instruction = stack.last_mut().unwrap().last_mut();

                    match last_instruction{
                        Some(Instruction::MoveLeft(amount)) => {
                            *amount += 1;
                        }

                        _ => {
                            stack.last_mut().unwrap().push(Instruction::MoveLeft(1))
                        }
                    }

                }

                b'>' => {
                    let last_instruction = stack.last_mut().unwrap().last_mut();

                    match last_instruction{

                        Some(Instruction::MoveRight(amount)) => {
                            *amount += 1;
                        }

                        _ => {
                            stack.last_mut().unwrap().push(Instruction::MoveRight(1))
                        }
                    }
                }

                b'+' => {
                    let last_instruction = stack.last_mut().unwrap().last_mut();

                    match last_instruction {
                        Some(Instruction::Inc(amount)) => {
                            *amount += 1;
                        }

                        _ => {
                            stack.last_mut().unwrap().push(Instruction::Inc(1));
                        }
                    }
                }

                b'-' => {
                    let last_instruction = stack.last_mut().unwrap().last_mut();

                    match last_instruction {
                        Some(Instruction::Dec(amount)) => {
                            *amount += 1;
                        }

                        _ => {
                            stack.last_mut().unwrap().push(Instruction::Dec(1));
                        }
                    }
                }

                b'.' => {
                    stack.last_mut().unwrap().push(Instruction::PrintToAscii)
                }
                
                b'[' => {
                    stack.push(Vec::new());
                }

                b']' => {

                    if stack.len() < 2 {
                        return Err("Syntax Error: Unmatched '[' ".to_string());
                    }

                    let inner_loop = stack.pop().unwrap();

                    if inner_loop.len() == 1 && inner_loop[0] == Instruction::Inc(1){
                        
                        let warning_msg = format!("Warning: The [+] ending at index {byte_counter} does not have an loop ending condition");
                        eprintln!("{}", warning_msg.yellow());
                    }

                    stack.last_mut().unwrap().push(Instruction::Loop(inner_loop));
                }

                _ => {
                    
                }
            }
        }

        // dbg!(&stack);
        if stack.len() > 1{
            return Err("Syntax Error: Unmatched '['".to_string());
        }

        let ast = stack.pop().unwrap();
        // dbg!(&ast);

        Ok(ast)
    }

    pub fn interpret(code: &str) -> Result<(), String>{

        let mut tape: [u32; TAPE_SIZE] = [0; TAPE_SIZE];
        let mut pointer: usize = 0;
        
        let ast = parser(&code)?;

        execute(&ast,&mut tape, &mut pointer);

        Ok(())
        // dbg!(ast);
    }

    fn execute(instructions: &[Instruction], tape: &mut [u32], pointer: &mut usize){
        for instruction in instructions{
            match instruction{
                Instruction::Inc(amount) => {
                    tape[*pointer] = tape[*pointer] + amount;
                }

                Instruction::Dec(amount) =>{
                    tape[*pointer] = tape[*pointer] - amount;
                }

                Instruction::MoveLeft(amount) => {
                    *pointer -= *amount as usize;
                }

                Instruction::MoveRight(amount) => {
                    *pointer += *amount as usize;
                }

                Instruction::PrintToAscii => {
                    print!("{}", tape[*pointer] as u8 as char);
                }

                Instruction::Loop(inner_instructions) => {

                    // [-] idiom => tape[i] = 0
                    if inner_instructions.len() == 1 && inner_instructions[0] == Instruction::Dec(1){
                        tape[*pointer] = 0;
                        dbg!("Handled [-] with no loops!");
                    }

                    while tape [*pointer] != 0 {
                        execute(inner_instructions, tape, pointer);
                    }
                }
            }
        }
    }

}