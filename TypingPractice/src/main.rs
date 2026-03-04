//Typing Practice

//For the words to get randomly chosen
use rand::Rng;
use std::io;

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
    
    //Variables that are gonna be stored for the sessions
    let mut attempts = 0;
    let mut score = 0;

    println!("Type the words that are shown on the screen. Type 'done' when you want to finish this session.");

    //Inner loop used for the session
    loop {
        //Program picks a random word that the user needs to type
        let random_index = rand::thread_rng().gen_range(0..selected_words.len());
        let word = selected_words[random_index];

        //Prints out the word that got randomly chosen
        println!("Word: {}", word);

        //Grabs user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        let input = input.trim();

        //Exits the session once the user types in done
        if input == "done" {
            break;
        }

        //Checks if what the user typed in matches the word that was chosen
        if input == word {
            println!("Correctamundo!");
            score += 1;
        } else {
            println!("Wrong brochacho!");
        }

        attempts += 1;

    }

    //Prints out how much the user got correct after the session ends
    println!("Session complete! You got {}/{} correct.", score, attempts);
}