use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    println!("Multiplication Game");
    println!("-------------------");
    println!("What is the smallest factor you'd like to practice with?");
    let min = get_integer();
    println!("And the largest?");
    let max = get_integer();
    let facts = build_facts_vector(min, max);

    for fact in facts {
        print!("{} x {}: ", fact.0, fact.1);
        io::stdout().flush().unwrap();
        let answer = fact.0 * fact.1;
        let guess = get_integer();

        match guess.cmp(&answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => println!("Perfect!"),
        }
    }
}

fn get_integer() -> i32 {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please type a number!");

    return number;
}

fn build_facts_vector(min: i32, max: i32) -> Vec<(i32, i32)> {
    let mut facts: Vec<(i32, i32)> = Vec::new();

    for i in min..=max {
        for j in i..=max {
            facts.push((i, j));
        }
    }

    return facts;
}
