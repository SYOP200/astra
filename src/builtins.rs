use std::{
    env,
    process
};

pub fn run(args: &[String]) -> bool {

    match args[0].as_str() {

        "exit" => {
            process::exit(0);
        }

        "cd" => {

            if args.len() < 2 {
                return true;
            }

            let _ = env::set_current_dir(&args[1]);

            true
        }

        "clear" => {
            print!("\x1B[2J\x1B[1;1H");
            true
        }

        "pwd" => {

            if let Ok(path) = env::current_dir() {
                println!("{}", path.display());
            }

            true
        }

        _ => false
    }
}
