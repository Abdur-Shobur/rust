// Day 05 [Option, Result, Error]

// Option

fn checkEven(n: i32) -> Option<i32> {
    if n % 2 == 0 { Some(n) } else { None }
}

/*
   let number = checkEven(10);
    match number {
        Some(n) => println!("Even number: {}", n),
        None => println!("Not an even number"),
    }

    // unwrap
    let number = checkEven(9).unwrap_or(0);
    println!("Number is: {}", number);
*/

// Result
// Ok(value)
// Err(error)

fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 { Err(String::from("Can not divide by zero")) } else { Ok(a / b) }
}
/*
 match safe_div(10, 2) {
        Ok(v) => println!("Result: {}", v),
        Err(e) => println!("Error: {}", e),
    }
*/

// Operator
use std::fs;

fn readFile() -> Result<String, std::io::Error> {
    let content = fs::read_to_string("test.txt")?;
    Ok(content)

    /*
    match readFile() {
        Ok(v) => println!("Ok{}", v),
        Err(e) => println!("Error {}", e),
    }
    */
}

// custom error message

fn checkAge(age: i32) -> Result<&'static str, &'static str> {
    if age >= 18 {
        Ok("Adult")
    } else {
        Err("Underage")
    }
    /*
      match checkAge(10) {
        Ok(msg) => println!("msg {}", msg),
        Err(err) => println!("Err {}", err),
    }
    */
}

// mix example

fn fileLen(path: &str) -> Result<Option<usize>, std::io::Error> {
    let content = fs::read_to_string(path)?;

    if content.is_empty() {
        Ok(None)
    } else {
        Ok(Some(content.len()))
    }

    /*
     match fileLen("test.txt") {
        Ok(Some(len)) => println!("File length: {}", len),
        Ok(None) => println!("File is empty"),
        Err(err) => println!("Error: {}", err),
    }
    */
}
fn first_char(s: &str) -> Option<char> {
    s.chars().nth(0)
}

fn checkString(s: &str) -> Result<Option<char>, ()> {
    // if s.len() > 0 { s[0] } else { None }

    Ok(first_char(s))

    /*
      match checkString("test") {
        Ok(v) => println!("Ok: {:?}", v),
        Err(e) => println!("Err: {:?}", e),
    }
    */
}

// fn divideFn(a: i32, b: i32) -> Result<i32, String> {
//     if b == 0 { Err(String::from("Can not divide")) } else { Ok(a / b) }
// }

fn readFileContent(path: &str) -> Result<String, std::io::Error> {
    let read = fs::read_to_string(path)?;

    Ok(read)
}
fn main() {
    match readFileContent("test.txt") {
        Ok(v) => println!("Ok: {:?}", v),
        Err(e) => println!("Err: {:?}", e),
    }
}
