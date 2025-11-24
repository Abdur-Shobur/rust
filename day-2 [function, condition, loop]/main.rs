// Function in rust

/*
fn fn_example(param:Type)->ReturnType{
    Function Body
}
*/


// example
fn greet(name:&str){
    println!("Hello,{}",name);
}

// with return 
fn add(a:i32,b:i32)->i32
{
    a+b
}

//  if else condition
fn check_age(){
    let age=12;

    if age>=18{
        println!("Adult");
    }else{
        println!("Minor")
    }
}

// in in expression
const x:i32=10;
const is_even:bool= if x%2==0{true}else{false};

// switch case 
fn check_match(){
    let number=2;

    match number {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
         _ => println!("Other number"),
    }
}

// loop

fn check_loop(){
    let mut count=0;

    loop{
        count += 1;
        println!("Count={}",count);

        if count==5{
            break;
        }
    }
}

// while loop

fn check_while_loop(){
    let mut n=1;

    while n<=5{
        println!("{}",n);
        n+=1;
    }
}

// for loop

fn check_for_loop(){

    for i in 1..=5{  // 1 to 5, 1..5 means 1 to 4
        println!("{}",i);
    }
}




// square function

fn square(n:i32)->i32{
    n*n
}

// print event or odd

fn checkEvOrOdd(n:i32){
    if n%2==0{
        println!("Even");
    }else{
        println!("Odd");
    }

}

fn main(){
 checkEvOrOdd(5);
 
}