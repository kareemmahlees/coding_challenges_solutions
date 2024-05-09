use json_parser::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
        std::process::exit(1)
    };

    std::process::exit(0);
}
