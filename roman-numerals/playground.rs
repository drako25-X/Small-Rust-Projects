fn number_to_roman(x: i32) {
    let romans = [
        "I", "IV", "V", "IX", "X", "XL", "L", "XC", "C", "CD", "D", "CM", "M",
    ];
    print!("The Roman numeral for {} is: ", x);
    if x < 4 {
        println!("{}", romans[0].repeat(x as usize));
    } else if x >= 4 {
        match x {
            4 => println!("{}", romans[1]),
            5 => println!("{}", romans[2]),
            6 => println!("{}{}", romans[2], romans[0]),
            7 => println!("{}{}", romans[2], romans[0].repeat(2)),
            8 => println!("{}{}", romans[2], romans[0].repeat(3)),
            9 => println!("{}", romans[3]),
            10 => println!("{}", romans[4]),
            11 => println!("{}{}", romans[4], romans[0]),
            12 => println!("{}{}", romans[4], romans[0].repeat(2)),
            _ => println!("Number too large"),
        }
    }
}

fn main() {
    println!("Hello, world!");
    number_to_roman(8);
}
