use std::{
    fmt::write,
    fs::{File, OpenOptions},
    io::{self, Seek, Write, stdin},
    ptr::read,
    usize::{self},
};

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
    println!("What File do you want to change? (if it doesnt exist it creates a new one)");

    let mut file_name = String::new();
    io::stdin().read_line(&mut file_name).unwrap();

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_name.trim())
        .expect("failed to open file");
    let mut media_list = Vec::new();

    loop {
        println!("what do you want to do in your Media libary??? (<help> for options)");

        let mut user_input = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        match user_input.trim() {
            "help" => {
                println!(
                    "<new> => add new Media, <list> => list all your Media <remove> => remove a Media"
                );
                continue;
            }
            "new" => media_list.push(Media::make_new_media()),
            "list" => list(&media_list),
            "remove" => {
                let index = delete(&media_list);
                if index < media_list.len() {
                    media_list.remove(index);
                } else {
                    continue;
                }
            }

            _ => {
                println!("Command doesnt exist");
                continue;
            }
        }
        // clear file
        file.set_len(0);
        // set writer at begining
        file.rewind().unwrap();

        for i in &media_list {
            write!(file, "{:?}", i).unwrap();
        }
    }
}

impl Media {
    pub fn make_new_media() -> Media {
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

fn delete(media_list: &Vec<Media>) -> usize {
    println!("what do you want to delete?");
    list(&media_list);
    let mut media = String::new();
    io::stdin().read_line(&mut media).unwrap();

    for i in 0..media_list.len() {
        if media_list[i].name == media.trim() {
            return i;
        }
    }

    println!("name wasnt found");
    usize::MAX
}
