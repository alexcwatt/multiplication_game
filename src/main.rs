use rand::seq::SliceRandom;
use rand::thread_rng;
use std::io;
use std::io::Write;
use std::time::Instant;

fn main() {
    println!("Multiplication Game");
    println!("-------------------");
    println!("What is the smallest factor you'd like to practice with?");
    let min = get_integer();
    println!("And the largest?");
    let max = get_integer();
    let facts = build_facts_vector(min, max);
    let count = facts.len();

    let mut streak = 0;
    let start = Instant::now();

    for fact in facts {
        let mut failed_attempts = 0;
        while !request_fact(fact) {
            failed_attempts += 1;
            println!("Try again");
        }
        if failed_attempts == 0 {
            streak += 1;
            println!("First try! Your streak is {}.", streak);
        } else {
            if streak > 0 {
                println!("You got it! Unfortunately your streak of {} was broken.", streak);
                streak = 0;
            }
        }
    }

    let total_time = start.elapsed();
    println!("You solved {} math facts in {} seconds", count, total_time.as_secs());
}

fn request_fact(fact: (i32, i32)) -> bool {
    print!("{} x {}: ", fact.0, fact.1);
    io::stdout().flush().unwrap();
    let answer = fact.0 * fact.1;
    let guess = get_integer();

    return guess == answer;
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

    facts.shuffle(&mut thread_rng());

    return facts;
}
