// primitive data types
// int float bool and char

// int
// signed and unsinged
// i8, i16, i32, i64, i128
// u8, u16, u32, u64, u128
// the number is the number of bits

// float
// f32, f64

// bool
// true false
// not capitalized

// character
// unicode value

fn main() {
    let x: i32 = -42;
    let y: u64 = 100;
    println!("signed: {}", x);
    println!("unsigned: {}", y);

    let z: f32 = 3.14;
    println!("float: {}", z);

    let is_snowing: bool = true;
    println!("bool: {}", is_snowing);

    let letter: char = 'a';
    println!("character: {}", letter);
}
