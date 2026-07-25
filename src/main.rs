mod shell;
mod prompt;
mod parser;
mod executor;
mod builtins;
mod history;
mod config;
mod theme;
mod git;
mod completion;
mod aliases;

fn main() {

    let args: Vec<String> =
        std::env::args().collect();


    if args.contains(&"--version".to_string()) {
        println!(
            "astra {}",
            env!("CARGO_PKG_VERSION")
        );

        return;
    }


    shell::start();
}
