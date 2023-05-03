use rust_pass_practice::run;
fn main() {
    if let Err(e) = run() {
        eprintln!("Could not read saved password: {e}")
    };
}
