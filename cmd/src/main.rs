use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, filename) = parse_args(&args);
    // debug formatter: :?
    println!("query = {:?}, filename = {:?}", query, filename);
    let config = Config::new(&args);

    // --snip--
}

fn parse_args(args: &[String]) -> (&str, &str) {
    let query = &args[1];
    let filename = &args[2];
    (query, filename)
}

struct Config {
    query: String,
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone();

    Config { query, filename }
}

// --snip--

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Config { query, filename }
    }
}