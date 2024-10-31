use std::io;

fn main() {
    println!("Guess the number!");//Guess user

    println!("Please input your guess.");

    //A mutable variable called 'guess' with an initial empty string
    //In Rust, variables are immutable by default, meaning once assigned, they can’t be changed.
    //mut stands for "mutable," allowing this variable to be modified after its initial creation.
    //We need mut here because guess will be updated with the user’s input, which changes its value.
    let mut guess = String::new();
    //importing std::io with use std::io;, we gain access to input/output tools, including the stdin() function.
    io::stdin()
    //.read_line(...) is a method that is called on the standard input stream object, which we get from io::stdin().
    //read_line takes a mutable reference (&mut) to a String as an argument, here &mut guess.
    //Without &mut, Rust would not allow guess to be modified, and the program would produce an error. 
        .read_line(&mut guess)
    //If the Result is Ok, meaning read_line worked as expected, expect unwraps the Ok value, allowing the program to continue.
    //If the Result is Err, meaning an error occurred, expect will terminate the program and print the custom error message provided: "Failed to read line".    
        .expect("Failed to read line");
    //The println! macro automatically replaces {} with the value passed as an argument after the format string—in this case, guess.
    println!("You guessed: {}", guess);
}

