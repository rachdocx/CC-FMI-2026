pub mod interpreter {
    const TAPE_SIZE: usize = 30_000;

    pub fn interpret(code: &str){
        let mut tape: [i32; TAPE_SIZE] = [0; TAPE_SIZE];
        let mut pointer = 0;
        let mut i = 0;

        let bytes = code.as_bytes(); //why??
        while  i < bytes.len() {

            match bytes[i] {
                
                b'<' => {
                    pointer -= 1;
                }

                b'>' => {
                    pointer += 1;
                }

                b'+' => {
                    tape[pointer] += 1;
                }

                b'-' => {
                    tape[pointer] -= 1;
                }

                b'.' => {
                    print!("{}", tape[pointer] as u8 as char);
                }

                b'[' => {
                    if tape[pointer] == 0{
                        let mut loop_rank = 1;
                        while loop_rank > 0{
                            i += 1;

                            if bytes[i] == b'[' {
                                loop_rank += 1;
                            }

                            if bytes[i] == b']' {
                                loop_rank -= 1;
                            }
                            
                        }
                    }
                }

                b']' => {
                    if tape[pointer] != 0{
                        let mut loop_rank = 1;
                        while loop_rank > 0{
                            i -= 1;

                            if bytes[i] == b']' {
                                loop_rank += 1;
                            }

                            if bytes[i] == b'[' {
                                loop_rank -= 1;
                            }
                        }
                    }
                }
                _ => {}
                
            }
            i +=1;   
        }
    }
}