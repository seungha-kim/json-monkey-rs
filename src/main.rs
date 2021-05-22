use rustyline::Editor;

use json_monkey_rs::repl::Repl;

fn main() {
    let mut rl = Editor::<()>::new();
    let mut repl = Repl::new();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("{:?}", repl.eval_str(&line));
            }
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}
