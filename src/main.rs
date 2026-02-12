use std::io;

enum MediaType {
    Video,
    Game,
    Book,
    Music,
}

struct Media {
    name: String,
    mediatype: MediaType,
    finished: bool,
}

fn main() {
    loop {
        let media_list: Vec<Media> = vec![];

        println!("Hello, user!");
        println!("what do you want to do in your Media libary??? (<help> for options)");

        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut user_input).unwrap();

        let user_input = user_input.trim();
        match user_input {
            "help" => println!("hello World"),

            _ => println!("Command doesnt exist"),
        }
    }
}
