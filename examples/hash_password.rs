use argon2::{password_hash::SaltString, Argon2, PasswordHash};
use rand_core::OsRng;

fn main() {
    let password = String::from("112358");
    let right_hash = hash_password(password.clone());

    let res = verify_password(password.clone(), right_hash);
    let wrong = verify_password(password, "123455".to_string());

    println!("{:?}", res);
    println!("{:?}", wrong);
}

fn hash_password(password: String) -> String {
    let salt = SaltString::generate(&mut OsRng);

    PasswordHash::generate(Argon2::default(), password, salt.as_str())
        .unwrap()
        .to_string()
}

fn verify_password(password: String, password_hash: String) {
    let hash = PasswordHash::new(&password_hash).unwrap();
    let res = hash.verify_password(&[&Argon2::default()], password);
    println!("{:?}", res);
}
