use std::io;

pub fn input_number()->u32 {
    let mut input_number = String::new();

    io::stdin()
        .read_line(&mut input_number)
        .expect("Failed to read line");

    let input_number: u32 = match input_number.trim().parse() {
        Ok(num) => {
            num
        }
        Err(_) => 0,
    };

    input_number
}