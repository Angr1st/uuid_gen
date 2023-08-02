use std::error::Error;
use uuid::Uuid;

fn main() -> Result<(), Box<dyn Error>> {
    let id = Uuid::new_v4();
    println!("{}", id);
    Ok(())
}
