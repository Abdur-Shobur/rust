// day-1/main.rs 
// run fist Rust program
// rustc main.rs
// ./main 
 // Function definition
fn test (){
    println!("This is my first test")
}


// Variable 

fn variable_example(){
     let x=5;

     // make mutable
     let mut y =10;
     y=15;
    println!("the y value is {}",y)
}

// Data type 

fn data_type_example(){
    let a: i32 = 10; // integer
    let b: f64 = 20.5; // floating point
    let c: char = 'R'; // character
    let d: bool = true; // boolean
 
    println!("Integer: {}, Float: {}, Char: {}, Bool: {}", a, b, c, d);
}

fn write_name(name:&str){
    println!("My name is {}",name)
}

fn main (){
   println!("Hello, Rust!");
}