use bcrypt::{hash, verify, DEFAULT_COST};
use std::iter::repeat;

fn main() {
    let password = "password";
    let prefix: String = repeat("a").take(72).collect();

    let combined = format!("{}{}", prefix, password);
    let hashed = hash(&combined, DEFAULT_COST).unwrap();

    let wrong_password = "wrong_password";
    let wrong_combined = format!("{}{}", prefix, wrong_password);

    let result = verify(&wrong_combined, &hashed);
    
    match result {
        Ok(valid) => {
            println!("Password: {}", valid);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
