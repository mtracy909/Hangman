use std::io::{self, Write};
fn main() {
    let mut again = true;
    let mut answer = String::new();
    while again == true {
        play_game();
        print!("Would you like to play again (Y/N)? ");
        io::stdout().flush().unwrap();
        answer.clear();
        io::stdin().read_line(&mut answer).expect("failed to readline");
        match answer.trim(){
            "y" | "Y" => again = true,
            "n" | "N" => again = false,
            _ => {
                println!("Invalid answer. Ending program.");
                again = false;
            },
        }
    }
}
fn play_game(){
    let secret = "rust".to_string();
    let mut correct_answers : Vec<char> = vec![];
    let mut wrong_answers : Vec<char> = vec![];
    let mut lives_left = secret.len() - 1;
    let mut clue = "".to_string();
    for _ in secret.chars(){
        clue.push_str("_");
    }

    println!("Let's play hangman!");
    while lives_left > 0{
        // println!({}, get_clue(secret, correct_answers));
        println!("Word: {}", clue);
        println!("Wrong guesses: {:?}", wrong_answers);
        println!("Lives left: {}", lives_left);
        let mut guess_str = String::new();
        let guess_char: char;
        print!("Enter your guess (a single letter): ");
        io::stdin().read_line(&mut guess_str).expect("failed to readline");
        println!("You entered {}", guess_str);
        guess_char = guess_str.chars().next().unwrap();
        if correct_answers.contains(&guess_char) || wrong_answers.contains(&guess_char){
            println!("You have already guessed that letter. Try again.");
        }
        else{
            if secret.contains(guess_char){
                println!("Correct! The letter {} is in the word.", guess_char);
                correct_answers.push(guess_char);
                clue = "".to_string();
                for letter in secret.chars(){
                    if correct_answers.contains(&letter){
                        clue.push(letter);
                    }
                    else{
                        clue.push('_')
                    }
                }
                if clue == secret{
                    println!("You win! You solved the hidden word, {}!", secret);
                    return;
                }
            }
            else{
                println!("Incorrect.The letter {} is not in the word.", guess_char);
                wrong_answers.push(guess_char);
                lives_left -= 1;
            }
        }
    }
    println!("Game over. You ran out of lives. The word was: {}", secret);
    println!("You solved it, the word was {}", secret);
}