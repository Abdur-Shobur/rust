// Move

fn check_move() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    // println!("{}",s1);  // Error : s1 move to s2

    println!("{}", s2); // valid
}

// Copy type [ this are copy not move]
// i32, u64, f64, bool, char

fn check_copy() {
    let x = 10;
    let y = x;

    println!("x {} y {}", x, y) // x 10 y 10
}

// Borrowing ধার করা,
fn check_borrow() {
    // immutable borrow
    // let s = String::from("Hello");
    // let len = calc_len(&s);

    // println!("Length = {}", len);

    // mutable borrow
    let mut x = String::from("Test String");
    {
        let y = &mut x;
        println!("y= {}", y);
    }
    println!("x = {}", x);
}

fn calc_len(s: &String) -> usize {
    s.len()
}

// Slice String-এর নির্দিষ্ট অংশ borrow করা — এটাকে slice বলে।

fn check_slice() {
    let s = String::from("This is string");

    let s1 = &s[0..3];
    let s2 = &s[6..10];

    println!("s= {} s1= {} s2= {}", s, s1, s2)
    // s= This is string s1= Thi s2= s st
}

// Slice Array

fn check_array_slice() {
    let arr = [10, 23, 423, 2, 0, 323];
    let sli = &arr[1..3];

    println!("{:?}", sli)
}
fn main() {
    check_array_slice();
}
