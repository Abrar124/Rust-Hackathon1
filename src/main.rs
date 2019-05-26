use std::io;

/////////////////////   Task 1 //////////////////////

// use std::io;

// fn main() {
//     let mut radius = String::new();
//     let pi = 3.14159;
//     println!("Input the radius of the circle");
//     io::stdin()
//         .read_line(&mut radius)
//         .expect("failed to read input.");
//     let radius: f32 = radius.trim().parse().expect("invalid input");
//     println!("The area of the radius is:{}", area(radius, pi));
// }
// fn area(radius: f32, pi: f32) -> f32 {
//     pi * (radius* radius)
// }

///////////////////////// Task 2 //////////////////////

// use std::io;

// fn main() {
//       let mut n = String::new();
//       println!("please input your number");
//     io::stdin()
//         .read_line(&mut n)
//         .expect("failed to read input.");
//     let n: i32 = n.trim().parse().expect("invalid input");
//     println!("{}", n);
//     let number = n;

// if number < 0 {
//         println!("Negative Number Entered");
//     }else if number > 0 {
//         println!("Positive Number Entered");
//     }
//      else {
//         println!("You entered zero");
//     }

// }

////////////////// Task 3 ////////////////////////

// use std::io;

// fn main() {
//     let mut numerator = String::new();
//     let mut denominator = String::new();

//     println!("Input the number for numerator");

//     io::stdin()
//         .read_line(&mut numerator )
//         .expect("failed to read input.");
//            println!("Input the number for Denominator");
//     io::stdin()
//         .read_line(&mut denominator )
//         .expect("failed to read input.");
//     let numerator: f32 = numerator.trim().parse().expect("invalid input");
//     let denominator: f32 = denominator.trim().parse().expect("invalid input");
//      let zero:f32 = 0.0;
//     // println!("The area of the radius is:{}",numerator);
//     // println!("The area of the radius is:{}",denominator);

// if (numerator % denominator == zero)
// {
//     println!("Number {} is completely divisible by {}", numerator, denominator);
// }
// else
// {
//     println!("Number {} is not completely divisible by {}", numerator, denominator);

// }
// }

/////////////////////// Task 4 /////////////////////

// use std::io;
// fn main() {
//     let mut radius = String::new();
//     let pi = 3.14159;
//     let three:f32 = 3.0;
//     let four:f32 = 4.0;

//     println!("Input the Radius of sphere");
//     io::stdin()
//         .read_line(&mut radius)
//         .expect("failed to read input.");
//     let radius: f32 = radius.trim().parse().expect("invalid input");
//     println!("The area of the sphere is:{}", area(radius, pi, three, four));
// }
// fn area(radius: f32, pi: f32, three: f32, four: f32) -> f32 {
//    (four/three) * pi * (radius * radius *radius)
// }

///////////////////   Task 5   //////////////

// use std::io;

// fn main() {
//     let mut text = String::new();
//     let mut index = 0.0;
//     let incr:f32 = 4.0;

//     println!("Input tyour text");

//     io::stdin().read_line(&mut text)
//         .expect("Failed to read line");

//     while index < incr {
//         println!("{}", text);

//         index += 1.0;
//     }
//  }

//////////////  task 6 ///////////////////

//  use std::io;

// fn main() {
//     let mut numb = String::new();
//     let even = 2.0;

//     println!("please input the number");

//     io::stdin()
//         .read_line(&mut numb )
//         .expect("failed to read input.");
//     let numb: f32 = numb.trim().parse().expect("invalid input");
//      let zero:f32 = 0.0;

// if (numb % even == zero)
// {
//     println!("{} is even", numb);
// }
// else
// {
//     println!("{} is odd", numb);
// }
// }

////////////////////  Task 7  /////////////

// fn main() {
//     let mut alphabet = String::new();
//     let a: [i32; 5]= ["a", "e", "i", "o", "u"];

//     let vowel1:i32 = 'a';

//     println!("Enter a character");

//     io::stdin()
//         .read_line(&mut alphabet)
//         .expect("failed to read input.");
//     let alphabet: f32 = alphabet.trim().parse().expect("invalid input");
//     //  let zero:f32 = 0.0;

//     if (alphabet == vowel1) {
//         println!("letter {} is even", alphabet);
//     } else {
//         println!(" is odd",);
//     }
// }

//////////////  Task 8 ///////////////



// fn main() {
//     let mut Base = String::new();
//     let mut Height = String::new();
//     let two:f32 = 2.0;
//     println!("Input the value of base");
//     io::stdin()
//         .read_line(&mut Base)
//         .expect("failed to read input.");
//     let Base: f32 = Base.trim().parse().expect("invalid input");
//     println!("Input the value of Height");
//     io::stdin()
//         .read_line(&mut Height)
//         .expect("failed to read input.");
//     let Height: f32 = Height.trim().parse().expect("invalid input");
//     println!("The area of the radius is:{}", area(Height, Base,two));
// }
// fn area(Height: f32, Base: f32, two:f32) -> f32 {
//     (Height* Base)/two
// }




////////////////   Task 9 ///////////////


// fn main() {
//     let mut amount = String::new();
//     let mut interest = String::new();
//     let mut years = String::new();
//     let one:f32 = 2.0;
//     println!("Please enter principal amount");
//     io::stdin()
//         .read_line(&mut amount)
//         .expect("failed to read input.");
//     let amount: f32 = amount.trim().parse().expect("invalid input");
//     println!("Please Enter Rate of interest in %:");
//     io::stdin()
//         .read_line(&mut interest)
//         .expect("failed to read input.");
//     let interest: f32 = interest.trim().parse().expect("invalid input");
//     println!("Enter number of years for investment:");
//     io::stdin()
//         .read_line(&mut years)
//         .expect("failed to read input.");
//     let years: f32 = years.trim().parse().expect("invalid input");
//     println!("After {} years your principal amount {} over an interest rate of {}% will be {}", years, amount, interest, intcalculation(years,amount,interest, one));
// }
// fn intcalculation(interest: f32, amount: f32, years:f32, one:f32) -> f32 {
//     amount*((one+(0.01*interest))* years)
    
// }




////////////////////    Task 10 //////////////////



fn main() {
    let mut x1 = String::new();
    let mut x2 = String::new();
    let mut y1 = String::new();
    let mut y2 = String::new();
    // let one:f32 = 2.0;
    println!("Enter Co-ordinate for x1:");
    io::stdin()
        .read_line(&mut x1)
        .expect("failed to read input.");
    let x1: f32 = x1.trim().parse().expect("invalid input");
    println!("Enter Co-ordinate for x2:");
    io::stdin()
        .read_line(&mut x2)
        .expect("failed to read input.");
    let x2: f32 = x2.trim().parse().expect("invalid input");
    println!("Enter Co-ordinate for y1:");
    io::stdin()
        .read_line(&mut y1)
        .expect("failed to read input.");
    let y1: f32 = y1.trim().parse().expect("invalid input");
     println!("Enter Co-ordinate for y2:");
    io::stdin()
        .read_line(&mut y2)
        .expect("failed to read input.");
    let y2: f32 = y2.trim().parse().expect("invalid input");
    let  a = x1 - x2;
    let  b = y1 - y2;
    let  a2 = a *a;
    let  b2 = b *b;

    println!("Distance between points ({},{}) and ({}, {}) is:  {} " ,x1,x2, y1, y2, distance_calculation(a2,b2));
}

fn distance_calculation(a2: f32, b2: f32, ) -> f32 {
    0.5*(a2+b2)
}


