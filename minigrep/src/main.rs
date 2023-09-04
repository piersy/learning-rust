use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];

    let conf = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", conf.query);
    println!("In file {}", conf.file_path);

    let contents = fs::read_to_string(conf.file_path).expect("should have been able to read file");

    println!("With text \n{}", contents);
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        Ok(Config {
            query: args[1].clone(),
            file_path: args[2].clone(),
        })
    }
}
