use json_parser::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}");
    };
}
