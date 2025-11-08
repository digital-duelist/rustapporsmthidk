use std::io;
use std::io::Write;
use names::Generator;
use owo_colors::OwoColorize;

fn main() {
    
    fn calculator() {

        loop {

            let mut input1 = String::new();

            println!("^C To Exit At Any Time");

        print!("Enter The First Number: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input1)
            .expect("Failed To Read Line");

        let num1: i32 = input1
            .trim()
            .parse()
            .expect("Not A Valid Number");

        let mut input2 = String::new();

        print!("Enter The Second Number: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input2)
            .expect("Failed To Read Line");

        let num2: i32 = input2
            .trim()
            .parse()
            .expect("Not A Valid Number");

        println!("Choose One");
        println!("1. Addition");
        println!("2. Subtraction");
        println!("3. Multiplication");
        println!("4. Division");
        println!("5. Remainder");

        let mut choice = String::new();

        print!("Enter Your Choice: ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed To Read Line");

        let numchoice: i8 = choice
            .trim()
            .parse()
            .expect("Not A Valid Choice");

        let sum = num1 + num2;
        let difference = num1 - num2;
        let product = num1 * num2;
        let quotient = num1 / num2;
        let remainder = num1 % num2;

        match numchoice {
            1 => println!("The Sum Of {} And {} Is {}", num1, num2, sum),
            2 => println!("The Difference Between {} And {} Is {}", num1, num2, difference),
            3 => println!("The Product Of {} And {} Is {}", num1, num2, product),
            4 => println!("The Quotient Of {} And {} Is {}", num1, num2, quotient),
            5 => println!("The Remainder Of {} And {} Is {}", num1, num2, remainder),
            _ => println!("Not A Valid Option"),
        }

        }

    }

    fn namegen() {

        let mut input = String::new();

        print!("Generate A Random Name (yes/no): ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed To Read Line");

        let input = input.trim().to_string();

        let mut name = Generator::default();

        if input == "yes" {
            println!("Your Random Name Is: {}", name.next().unwrap().purple());
        } else if input == "no" {
            println!("Bye");
        } else {
            println!("Please Enter Valid Input");
        }

    }

    println!("Choose One");
    println!("1. Calculator");
    println!("2. Name Generator");

    let mut input = String::new();

    print!("Enter Your Choice: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed To Read Line");

    let numinput = input
        .trim()
        .parse()
        .expect("Not A Valid Number");

    match numinput {
        1 => calculator(),
        2 => namegen(),
        _ => println!("Not A Valid Choice"),
    }


}