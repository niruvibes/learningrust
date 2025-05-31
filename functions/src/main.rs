// functions
// any functions and variables need to be written in
// snake_case or, 
// kebab-case

// functions are hoisted, so can be placed anywhere and called, doesnt need a prototype


// this is the entry point to any rust program
fn main() {
    // let height: i32 = 24;
    // tell_height(height);

    // let val1: i32 = -34;
    // let val2: i32 = 68;
    // let name: &str = "values";
    // diff_between(val1, val2, name);

    let x = {
        let price = 5;
        let qty = 5;
        
        // return price * qty;  This line does the same thing as below
        price * qty
    };

    println!("The stock price on hand is {}", x);

}

// put vasraibles to be passed inside the brackets, similar to C (name: format)
// fn tell_height(height: i32) {
//     println!("Hello, the height is {}", height);
// }

// fn diff_between(val1: i32, val2: i32, name: &str) {
//     let result: i32 = val1 - val2;
//     println!("Yo the diff between {} is {}", name, result)
// }

// Expressions and Statements
// expressions: return value
//  true/false
//  if else
//  numbers
//
// statement: does not return value
//  
