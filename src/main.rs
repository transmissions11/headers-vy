use std::env;

use clipboard::{ClipboardContext, ClipboardProvider};

fn main() {
    let input = env::args().collect::<Vec<String>>()[1..].join(" ");

    let space1_length = (62 - input.len()) / 2;
    let space2_length = 62 - input.len() - space1_length;

    let output = format!(
        "{}\n{}{}{}{}{}\n{}",
        "################################################################",
        "#",
        (0..space1_length).map(|_| " ").collect::<String>(),
        input.to_uppercase(),
        (0..space2_length).map(|_| " ").collect::<String>(),
        "#",
        "################################################################"
    );

    println!("{}", output); // Print the header to console.

    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    ctx.set_contents(output).unwrap(); // Copy the header to clipboard.
}
