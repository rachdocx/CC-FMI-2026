pub mod ast_interpreter{

    #[derive(Debug)]
    #[derive(PartialEq)]
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

        for byte in code.bytes() {

            match byte {
                b'<' => {
                    stack.last_mut().unwrap().push(Instruction::MoveLeft(1));
                }

                b'>' => {
                    stack.last_mut().unwrap().push(Instruction::MoveRight(1));
                }

                b'+' => {
                    stack.last_mut().unwrap().push(Instruction::Inc(1));
                }

                b'-' => {
                    stack.last_mut().unwrap().push(Instruction::Dec(1));
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