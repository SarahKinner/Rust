//Typing Practice

//For the words to get randomly chosen
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::Instant;
use std::io;

fn run_typing_session(words: &Vec<&str>) {
    //Variables that are gonna be stored for the sessions
    let mut attempts = 0;
    let mut score = 0;
    let mut wrong_words: Vec<&str> = Vec::new();

    //shuffles the word list and goes through it once so no repeated words
    let mut lesson_words = words.clone();
    lesson_words.shuffle(&mut thread_rng());

    println!("Type the words that are shown on the screen:");

    //Starts the timer for the lesson
    let start_time = Instant::now();

    for word in &lesson_words {
        //Prints out the word that got randomly chosen
        println!("Word: {}", word);

        //Grabs user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        //Checks if what the user typed in matches the word that was chosen if wrong it'll store the wrong word
        if input == *word {
            println!("Correctamundo!");
            score += 1;
        } else {
            println!("Wrong brochacho!");
            wrong_words.push(word);
        }

        attempts += 1;

    }

    //Stops the timer
    let duration = start_time.elapsed();

    //Prints out how much the user got correct after the session ends
    println!("Session complete! You got {}/{} correct.", score, attempts);

    //Prints how long it took to complete the session
    println!("Time taken: {:.2} seconds", duration.as_secs_f64());

    //Calculate the accuracy and WPM
    let accuracy = (score as f64 / attempts as f64) * 100.0;
    let minutes = duration.as_secs_f64() / 60.0;
    let wpm = score as f64 / minutes;

    //Prints out the accuracy and WPM
    println!("Accuracy: {:.2}%", accuracy);
    println!("Words per minute: {:.2}", wpm);

    //Prints out the words you got wrong
    if !wrong_words.is_empty() {
        println!("Words you got wrong:");
        for w in wrong_words {
            println!("- {}", w);
        }
    } else {
        println!("Amazing! You got all words correct!");
    }
}

fn main() {
    
    //Words that are used for easy difficulty
    let easy_words = vec![
        "cat", "dog", "sun", "tree", "book",
        "pen", "cup", "ball", "hat", "fish",
        "car", "bird", "shoe", "milk", "apple",
        "star", "leaf", "map", "egg", "door",
        "toy", "ring", "cup", "bag", "rain"
    ];

    //Words that are used for medium difficulty
    let medium_words = vec![
        "keyboard", "window", "coffee", "pencil", "flower",
        "orange", "garden", "butter", "laptop", "banana",
        "bottle", "castle", "guitar", "friend", "letter",
        "market", "pillow", "rocket", "school", "summer",
        "travel", "violet", "wallet", "yellow", "zigzag"
    ];

    //Words that are used for hard difficulty
    let hard_words = vec![
        "ownership", "borrowing", "match", "exquisite", "programming",
        "architecture", "development", "sophisticated", "assignment", "synchronize",
        "functionality", "multiplication", "implementation", "interaction", "optimization",
        "inheritance", "encapsulation", "polymorphism", "abstraction", "asynchronous",
        "cryptography", "algorithm", "integration", "concurrency", "documentation"
    ];

    loop {
        //What the menu is gonna look like
        println!("Welcome to Typing Practice!");
        println!("Select difficulty:");
        println!("1. Easy");
        println!("2. Medium");
        println!("3. Hard");

        //Get user input
        let mut choice = String::new();
        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read input");

        //Print out the choice that the user picked
        let choice = choice.trim();
        println!("You chose: {}", choice);

        // Matches the user input to pick the right word list difficulty
        let selected_words = match choice {
            "1" => &easy_words,
            "2" => &medium_words,
            "3" => &hard_words,
            _ => {
                println!("Bro cannot type something valid. If you can't follow some simple instructions Imma just give you easy difficulty.");
                &easy_words
            }
        };
        run_typing_session(selected_words);
    }
}