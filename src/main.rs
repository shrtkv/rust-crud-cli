use clap::{App, Arg};
use std::io::{self, Write};

fn main() {
    let matches = App::new("My CLI")
        .arg(Arg::with_name("input")
            .required(true)
            .takes_value(true)
            .help("Input file"))
        .arg(Arg::with_name("output")
            .takes_value(true)
            .help("Output file"))
        .get_matches();

    loop {
        println!("What do you want to do?");
        println!("1. Create");
        println!("2. Read");
        println!("3. Update");
        println!("4. Delete");
        println!("0. Exit");
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();

        if choice == "1" {
            confirm_and_execute("Create", create, &matches);
        } else if choice == "2" {
            confirm_and_execute("Read", read, &matches);
        } else if choice == "3" {
            confirm_and_execute("Update", update, &matches);
        } else if choice == "4" {
            confirm_and_execute("Delete", delete, &matches);
        } else if choice == "0" {
            break;
        } else {
            println!("Invalid choice. Please try again.");
        }
    }
}

fn confirm_and_execute(option_name: &str, option_function: fn(&clap::ArgMatches), matches: &clap::ArgMatches) {
    println!("You chose {}. Confirm? (y/n)", option_name);
    let mut confirm_choice = String::new();
    io::stdin().read_line(&mut confirm_choice).unwrap();
    let confirm_choice = confirm_choice.trim();

    if confirm_choice == "y" {
        option_function(matches);
        println!("Confirmed.");
    } else {
        println!("Cancelled.");
    }

    println!("Do you want to do anything else? (y/n)");
    let mut continue_choice = String::new();
    io::stdin().read_line(&mut continue_choice).unwrap();
    let continue_choice = continue_choice.trim();

    if continue_choice == "n" {
        return;
    }
}

fn create(matches: &clap::ArgMatches) {
    let input_file = matches.value_of("input").unwrap();
    println!("Performing Create operation with input file: {}", input_file);
    // create logic here
}

fn read(matches: &clap::ArgMatches) {
    let input_file = matches.value_of("input").unwrap();
    println!("Performing Read operation with input file: {}", input_file);
    // read logic here
}

fn update(matches: &clap::ArgMatches) {
    let input_file = matches.value_of("input").unwrap();
    println!("Performing Update operation with input file: {}", input_file);
    // update logic here
}

fn delete(matches: &clap::ArgMatches) {
    let input_file = matches.value_of("input").unwrap();
    println!("Performing Delete operation with input file: {}", input_file);
    // delete logic here
}

