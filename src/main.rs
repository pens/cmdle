// https://en.wikipedia.org/wiki/ANSI_escape_code
// \x1b = ESC
// Foreground Color: ESC[38;2;<r>;<g>;<b>m
// Background Color: ESC[48;2;<r>;<g>;<b>m

use std::io::Write;

#[derive(Copy, Clone)]
enum Status {
    Correct,
    WrongPosition,
    Incorrect,
}

fn check_guess(guess: &[u8; 5], answer: &[u8; 5]) -> (bool, [Status; 5]) {
    let mut result = [Status::Incorrect; 5];
    let mut correct = true;
    for i in 0..5 {
        result[i] = if guess[i] == answer[i] {
            Status::Correct
        } else if answer.contains(&guess[i]) {
            correct = false;
            Status::WrongPosition
        } else {
            correct = false;
            Status::Incorrect
        }
    }
    (correct, result)
}

fn print_result(guess: &[u8; 5], result: &[Status; 5]) {
    // Set foreground to white
    print!("\x1b[38;2;255;255;255m");
    for i in 0..5 {
        match result[i] {
            // Set background colors to green, yellow, or gray
            Status::Correct => print!("\x1b[48;2;108;169;101m"),
            Status::WrongPosition => print!("\x1b[48;2;200;182;83m"),
            Status::Incorrect => print!("\x1b[48;2;120;124;127m"),
        }
        // Reset colors for empty spaces between letters
        print!(" {} \x1b[0m", guess[i] as char);
        if i < 4 {
            print!(" ");
        }
    }
    // Reset colors
    println!("\x1b[0m");
}

fn main() {
    let answer: [u8; 5] = "SUSHI".as_bytes().try_into().unwrap();

    let mut buf = String::new();
    let mut input: &str;
    let mut previous_input_invalid = false;

    // Move cursor to top left; Clear screen
    println!("\x1b[H\x1b[J");

    // Print hint for next step
    let hint: [u8; 5] = "SHELF".as_bytes().try_into().unwrap();
    let (_, result) = check_guess(&hint, &answer);
    print_result(&hint, &result);

    loop {
        // Get 5 character input
        loop {
            // Read input
            buf.clear();
            std::io::stdin().read_line(&mut buf).unwrap();
            input = buf.trim_end();

            // Erase printed input
            // Move up; Erase line
            print!("\x1b[A\x1b[2K");

            if previous_input_invalid {
                // If previous input was invalid, we printed a line indicating so.
                // We do not need to keep this warning after a new input comes in.
                // Move up; Erase line
                print!("\x1b[A\x1b[2K");
                previous_input_invalid = false;
            }

            if input.len() == 5 && input.chars().all(|c| c.is_ascii_alphabetic()) {
                break;
            } else {
                previous_input_invalid = true;
                println!("Invalid input: {}", input);
            }
        }

        // Format input
        let guess: [u8; 5] = input.to_uppercase().as_bytes().try_into().unwrap();

        // Check guess
        let (correct, result) = check_guess(&guess, &answer);
        if correct {
            break;
        }
        print_result(&guess, &result);
    }

    // Print answer with special formatting as clue to next puzzle
    let black_on_yellow = "\x1b[38;2;0;0;0m\x1b[48;2;255;255;0m";
    let pink_on_green = "\x1b[38;2;255;0;128m\x1b[48;2;0;255;0m";
    let green_on_white = "\x1b[38;2;0;255;0m\x1b[48;2;255;255;255m";
    let white_on_orange = "\x1b[38;2;255;255;255m\x1b[48;2;255;128;0m";
    let reset = "\x1b[0m";
    println!(
        "{} S {} {} U {} {} S {} {} H {} {} I {}",
        black_on_yellow,
        reset,
        pink_on_green,
        reset,
        black_on_yellow,
        reset,
        green_on_white,
        reset,
        white_on_orange,
        reset
    );

    // Don't allow any more input
    loop {
        buf.clear();
        std::io::stdin().read_line(&mut buf).unwrap();
        // Move up; Erase line
        print!("\x1b[A\x1b[2K");
        std::io::stdout().flush().unwrap();
    }
}
