use std::io;

fn read_number() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

// Roman numerals conversion function, for matrice; one for each position
fn digit_to_roman(digit: usize, place: usize) -> &'static str {
    let ones = ["", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"];
    let tens = ["", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"];
    let hundreds = ["", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"];
    let thousands = ["", "M", "MM", "MMM"];

    match place {
        0 => ones[digit],
        1 => tens[digit],
        2 => hundreds[digit],
        3 => thousands[digit],
        _ => "",
    }
}
fn main() {
    println!("Enter a number between 1 and 3999: ");
    let input = read_number();

    let number: u32 = match input.parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };
    if number < 1 || number > 3999 {
        println!("Number out of range. Please enter a number between 1 and 3999.");
        return;
    }

    let digits = number.to_string();
    let digit_count = digits.len();
    println!("{} has {} digit(s)", number, digit_count);

    let mut result = String::new();
    for (index, ch) in digits.chars().enumerate() {
        let digit = ch.to_digit(10).unwrap() as usize;
        let place = digit_count - 1 - index;
        println!(" digit {} is in place {}", digit, place);
        result.push_str(digit_to_roman(digit, place));
    }
    println!("Roman numeral: {}", result);
}
