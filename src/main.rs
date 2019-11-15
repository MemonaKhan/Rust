use std::io;
fn main() {
    println!("Choose any one option in the following:");
    println!("1. Check the the number is Prime number or not.");
    println!("2. Print Fibonacci Series.");
    println!("3. Calculate area of a circle.");
    println!("4. Print Table.");
    println!("5. Check whether character is vowel or consonent.");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice:u8 = choice.trim().parse().expect("Please type a number");

    if choice == 1 {
        println!("Enter any number:");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read line");
        let num:u8 = num.trim().parse().expect("Please type a number");
        prime_number(num);
    }
    if choice == 2{
        println!("Enter any number:");
        let mut num = String::new();
        io::stdin().read_line(&mut num).expect("Failed to read line");
        let num:u8 = num.trim().parse().expect("Please type a number");
        fibonacci(num);
    }
    else if choice == 3{
        println!("Enter radius of a circle:");
        let mut rad = String::new();
        io::stdin().read_line(&mut rad).expect("Failed to read line");
        let rad:u8 = rad.trim().parse().expect("Please type a number");
        println!("Area of a Circle is {}",area_of_circle(rad));
    }
    else if choice == 4 {
        table();
    }
    else if choice == 5{
        println!("Enter a character:");
        let mut character = String::new();
        io::stdin().read_line(&mut character).expect("Failed to read line");
        let character:char = character.trim().parse().expect("Please type a character");
        check_character(character);
    }
}
fn prime_number(number:u8){
    let mut i=2;
    while i<number {
        if number%i == 0 {
            break;
        }
        i=i+1;
    }
    if number==i {
        println!("{} is a Prime Number",number);
    }
    else {
        println!("{} is not a Prime Number",number);
    }
}
fn fibonacci(number:u8){
    let mut a = -1;
    let mut b = 1;
    let mut c = 0;
    println!("First {} terms of Fibonacci Series:",number);
    for _i in 1..number+1{
        c=a+b;
        print!("{} ",c);
        a=b;
        b=c;
    }
}
fn area_of_circle(radius:u8)->f32{
    let area:f32 = 3.14*radius as f32*radius as f32;
    area
}
fn table(){
    println!("Enter any number:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let num:u8 = num.trim().parse().expect("Please type a number");
    let mut i=1;
    println!("Table of {}",num);
    loop{
        if i>10{
            break;
        }
        println!("{} x {} = {}",num,i,num*i);
        i=i+1;
    }
}
fn check_character(character:char){
    if character=='A'||character=='E'||character=='I'||character=='O'||character=='U'||
        character=='a'||character=='e'||character=='i'||character=='o'||character=='u'{
            println!{"{} is Vowel",character};
        }
    else{
        println!{"{} is Consonent",character};
    }
}