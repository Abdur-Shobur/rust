// Move 

fn check_move(){
    let s1= String::from("hello");
    let s2=s1; // s1 is moved to s2 

    // println!("{}",s1);  // Error : s1 move to s2

    println!("{}",s2); // valid 
}

fn main(){
    check_move();
}