use rustyline::Editor;
use json_monkey_rs::interpreter::Interpreter;

fn main() {
    let mut rl = Editor::<()>::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("{:?}", Interpreter::eval_str(&line));
            },
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}
