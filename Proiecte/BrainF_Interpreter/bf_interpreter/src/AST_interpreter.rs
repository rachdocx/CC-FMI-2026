pub mod AST_interpreter{

    #[derive(Debug)]
    pub enum instruction {
        inc(u32),
        dec(u32),
        move_left(u32),
        move_right(u32),
        print_to_ascii,
        Loop(Vec<instruction>)
    }
    const TAPE_SIZE: usize = 30_000;

    pub fn parser(code: &str) -> Result<Vec<instruction>, String>{

        let mut stack: Vec<Vec<instruction>>  = vec![Vec::new()];

        for byte in code.bytes() {

            match byte {
                b'<' => {
                    stack.last_mut().unwrap().push(instruction::move_left(1));
                }

                b'>' => {
                    stack.last_mut().unwrap().push(instruction::move_right(1));
                }

                b'+' => {
                    stack.last_mut().unwrap().push(instruction::inc(1));
                }

                b'-' => {
                    stack.last_mut().unwrap().push(instruction::dec(1));
                }

                b'.' => {
                    stack.last_mut().unwrap().push(instruction::print_to_ascii)
                }
                
                b'[' => {
                    stack.push(Vec::new());
                }

                b']' => {
                    let inner_loop = stack.pop().unwrap();

                    stack.last_mut().unwrap().push(instruction::Loop(inner_loop));
                }
                _ => {
                    
                }
            }
        }

        // dbg!(&stack);

        let ast = stack.pop().unwrap();
        
        // dbg!(&ast);

        Ok(ast)
    }

    pub fn interpret(code: &str){
        let mut tape: [u32; TAPE_SIZE] = [0; TAPE_SIZE];
        let mut pointer: usize = 0;
        
        let ast = parser(&code).unwrap();

        execute(&ast,&mut tape, &mut pointer);
        // dbg!(ast);
    }

    fn execute(instructions: &[instruction], tape: &mut [u32], pointer: &mut usize){
        for instruction in instructions{
            match instruction{
                instruction::inc(amount) => {
                    tape[*pointer] = tape[*pointer] + amount;
                }

                instruction::dec(amount) =>{
                    tape[*pointer] = tape[*pointer] - amount;
                }

                instruction::move_left(amount) => {
                    *pointer -= 1;
                }

                instruction::move_right(amount) => {
                    *pointer += 1;
                }

                instruction::print_to_ascii => {
                    print!("{}", tape[*pointer] as u8 as char);
                }

                instruction::Loop(inner_instructions) => {
                    while tape [*pointer] != 0 {
                        execute(inner_instructions, tape, pointer);
                    }
                }

                _ => {

                }
            }
        }
    }

}