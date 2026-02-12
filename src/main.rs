use std::io;

#[derive(Debug)]
enum MediaType {
    Video,
    Game,
    Book,
    Music,
}

#[derive(Debug)]
struct Media {
    name: String,
    mediatype: MediaType,
    finished: bool,
}

fn main() {
    let mut media_list = Vec::new();

    loop {
        println!("Hello, user!");
        println!("what do you want to do in your Media libary??? (<help> for options)");

        let mut user_input = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut user_input).unwrap();

        
        media_list.push(match user_input.trim() {
            "help" => {
                println!("<new> => add new Media");
                continue;
            }
            "new" => Media::new(),
            _ => {
                println!("Command doesnt exist");
                continue;
            }
        });

        dbg!(&media_list);
    }
}

impl Media {
    pub fn new() -> Media {
        // name
        println!("name?");

        let mut name = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut name).unwrap();
        name = name.trim().to_string();

        // Media Type
        println!("Media Type? (<Video> || <Game> || <Book> || <Music>)");

        let mut media_string = String::new();
        stdin.read_line(&mut media_string).unwrap();

        let media_type = match media_string.trim() {
            "Video" => MediaType::Video,
            "Game" => MediaType::Game,
            "Book" => MediaType::Book,
            "Music" => MediaType::Music,

            _ => {
                println!("Media Type doesnt exist!!! defaulting to Video");
                MediaType::Video
            }
        };

        //finished
        println!("finished the Media? (<true> <false>)");
        let mut finished_string = String::new();
        stdin.read_line(&mut finished_string).unwrap();
        let finished: bool = finished_string.trim().parse().unwrap_or(false);

        Media {
            name: name,
            mediatype: media_type,
            finished: finished,
        }
    }
}
