/**
 * prompt user for password to check
 * read password.txt into memory
 * loop through passwords in password.txt
 * if user_password exists, print warning
 */

mod user_input;

fn main() {
    let user_password: String = user_input::get_user_input("Enter password:");

    println!("Password entered: {}", user_password);
}
