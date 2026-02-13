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
        println!("what do you want to do in your Media libary??? (<help> for options)");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim() {
            "help" => {
                println!("<new> => add new Media, <list> => list all your Media");
                continue;
            }
            "new" => media_list.push(Media::new()),
            "list" => list(&media_list),

            _ => {
                println!("Command doesnt exist");
                continue;
            }
        }
    }
}

impl Media {
    pub fn new() -> Media {
        // name
        println!("name?");

        let mut name = String::new();
        io::stdin().read_line(&mut name).unwrap();
        name = name.trim().to_string();

        // Media Type
        println!("Media Type? (<Video> || <Game> || <Book> || <Music>)");

        let mut media_string = String::new();
        io::stdin().read_line(&mut media_string).unwrap();

        let mediatype = match media_string.trim() {
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
        io::stdin().read_line(&mut finished_string).unwrap();
        let finished: bool = finished_string.trim().parse().unwrap_or(false);

        Media {
            name,
            mediatype,
            finished,
        }
    }
}

fn list(media_list: &Vec<Media>) {
    for i in 0..media_list.len() {
        println!("{:?}", media_list[i]);
    }
}
