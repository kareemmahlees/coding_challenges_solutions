use comprust::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("{e}")
    }
}
