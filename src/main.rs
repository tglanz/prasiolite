use prasiolite;

fn main() {
    if let Err(error) = prasiolite::run() {
        eprintln!("an error has ocurred: {}", error);
    }
}
