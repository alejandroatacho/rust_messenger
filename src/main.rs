use std::io;

fn main() {
    let empty_string = " ";
    let mut num = 0;
    println!("Enter your name player 1: ");
    let mut player1: String = String::new();
    io::stdin()
        .read_line(&mut player1)
        .expect("Failed to read line");

    println!("Enter your name player 2: ");
    let mut player2: String = String::new();
    io::stdin()
        .read_line(&mut player2)
        .expect("Failed to read line");
    println!("Hello, Rust Messenger has been initialized!");

    {
        while num != 1 {
            println!("write down your answer/question below and press enter to continue: ");
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            // println!("{}", answer);
            println!("{} wrote: {}", player1, answer);
            println!("write down your answer/question below and press enter to continue: ");
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            println!("{} wrote: {}", player2, answer);

            num += 1;
        }
    }
}
