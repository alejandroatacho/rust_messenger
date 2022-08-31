use std::io;

fn main() {
    let mut num = 0;
    println!("Hello, Rust Messenger has been initialized!");

    {
        while num != 1 {
            println!("write down your answer/question below and press enter to continue: ");
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            // println!("{}", answer);

            println!("write down your answer/question below and press enter to continue: ");
            let mut answer = String::new();
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");
            num += 1;
        }
    }
}
