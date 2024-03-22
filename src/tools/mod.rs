use md5::Digest;
use md5::Md5;

pub mod jwt;

pub fn md5_crypto(password: String) -> String {
    let hashed_password = format!("{:x}", Md5::digest(password.as_bytes()));
    hashed_password
}
pub fn are_strings_equal(input_password: &str, password: &str) -> bool {
    password == format!("{:x}", Md5::digest(input_password.as_bytes()))
}
