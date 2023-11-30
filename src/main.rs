
#![allow(dead_code)]

// Uses
use std::io;

fn array_count() {

}

fn divide_number() {

}

fn multiply_number() {

}

fn subtract_number() {

}

fn add_number() {

    println!("Enter the first number:");

    let mut input_1 = String::new();
    io::stdin()
        .read_line(&mut input_1)
        .expect("Failed to read input");
    
    let num_1: f64 = input_1.trim().parse().expect("Please enter a number");

    println!("Enter the second number:");

    let mut input_2 = String::new();
    io::stdin()
        .read_line(&mut input_2)
        .expect("Failed to read input");
    
    let num_2: f64 = input_2.trim().parse().expect("Please enter a number");

    let result = num_1 + num_2;
    println!("The result of {} + {} is: {}", num_1, num_2, result);


}

fn count_to_ten() {

    let mut counter = 0;
    
    loop {
        while counter < 10 {
            counter +=1;
            println!("{:?}", counter);
            
        }
        break;
    }
}

fn choices() {

    loop {
        
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    
        let choice: u32 = input.trim().parse().expect("Please pick a number");
    
        match choice {
            1 => add_number(),
            2 => subtract_number(),
            _ => println!("Invalid choice"),
        }

    }

    
}

fn main() {

    println!("Please pick a choice below");
    println!("1. Add");

    choices();
}
