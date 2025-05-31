// Compound Data Types
// array, tuples, slices, strings (slice string)

// array
// fixed size of same data type 

// tuples
// fixed size of different data types

// string
// "Alice" is NOT a string, it is a string datatype
// a string slice can be converted with .to_string

// slices
// contigous means uninterupted, which means in memory it is stored next to each other
// dawg ts just a pointer

// string vs string slices
// strings are growable (can change size) and mutable (can change contents) and they are owned (ownership stuff later on)
// string slices are used when u want to work with string data without taking ownership

fn main() {
    // ARRAY
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Int Array: {:?}", numbers);

    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array: {}", fruits[0]);
    println!("Fruits Array: {}", fruits[1]);
    println!("Fruits Array: {}", fruits[2]);

    // tuples
    let human = ("Alice", 30, false);
    println!("Human Tuple: {:?}", human);
    let human2: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human 2 Tuple: {:?}", human2);
    let mix_tuple = ("Kratos", 23, true, [1,2,3]);
    println!("mix tuple: {:?}", mix_tuple);

    //slices
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number Slice: {:?}", number_slices);
    let string_slices:&[&str] = &["hello", "world","wowie"];
    println!("Number Slice: {:?}", string_slices);
    let words_slices:&[&String] = &[&"test".to_string(), &"testing".to_string(),&"tested".to_string()];
    println!("Number Slice: {:?}", words_slices);

    // strinmgs vs string slice
    let mut stone_cold: String = String::from("Hell, ");    // stored on heap
    stone_cold.push_str("Yeah!");
    println!("Says: {}", stone_cold);

    // B- &str (string slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice val: {}", slice);
}

// rust cleans up its own memory in its own contexts
// here the call to slice is outside of the scope of that context
// so we get an error
// fn print() {
//     println!("SLICE: {}", slice);
// }