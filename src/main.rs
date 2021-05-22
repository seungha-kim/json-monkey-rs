use rustyline::Editor;

use json_monkey_rs::interpreter::Interpreter;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut i = Interpreter::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("{:?}", i.eval_str(&line));
            }
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}
