use std::env;
use std::process;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let conf = minigrep::Config::build(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {err}");
    //     process::exit(1);
    // });

    let conf = minigrep::Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(conf) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
