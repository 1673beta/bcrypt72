use bcrypt::{hash, verify, DEFAULT_COST};
use openssl::sha::Sha512;
use std::iter::repeat;

fn main() {
    let password = "password";
    let prefix: String = repeat("a").take(72).collect();

    let mut hasher = Sha512::new();
    hasher.update(format!("{}{}", prefix, password).as_bytes());
    let sha512ed = hasher.finish();

    // let combined = format!("{}{}", prefix, password);
    let hashed = hash(&sha512ed, DEFAULT_COST).unwrap();

    let wrong_password = "wrong_password";
    let wrong_combined = format!("{}{}", prefix, wrong_password);

    let mut hasher_wrong = Sha512::new();
    hasher_wrong.update(wrong_combined.as_bytes());
    let sha512ed_wrong = hasher_wrong.finish();

    let result = verify(&sha512ed_wrong, &hashed);
    
    match result {
        Ok(valid) => {
            println!("Password: {}", valid);
        },
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}
