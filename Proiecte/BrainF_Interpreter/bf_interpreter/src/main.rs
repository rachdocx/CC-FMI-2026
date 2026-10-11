use std::env;
use std::io::Error;
use std::fs;

//naive interpreter
mod naive_interpreter;
// use naive_interpreter::naive_interpreter::interpret;

//AST interpreter
mod AST_interpreter;
use AST_interpreter::AST_interpreter::interpret;


fn main() -> Result<(), Error>{
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();

    let code = match fs::read_to_string(file_path){
        Ok(value) => value,
        Err(e) => return Err(e),
    };

    // interpret(&code);
    interpret(&code);
    Ok(())
}
