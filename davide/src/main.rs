use notify_rust::Notification;
use rand::seq::SliceRandom;
use rusty_audio::Audio;
use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let jokes = robba("../davide.txt");
    let joke: Vec<_> = jokes.choose_multiple(&mut rand::thread_rng(), 1).collect();
    //    println!("{:?}", joke[0]);

    //    for line in jokes {
    //        println!("ciao {:?}", line);
    //
    //    }

    Notification::new()
        .summary("A me gli occhi, Davide")
        .body(joke[0])
        .icon("firefox")
        .show()
        .unwrap();

    let path = Path::new("./src/audio.mp3");
    let mut audio = Audio::new();
    audio.add("davide", path.to_str().unwrap());
    audio.play("davide");
    audio.wait();

    //let path = PathBuf::from("./src/audio.mp3");
    //println!("ciao {:?}", fs::canonicalize(path.to_str().unwrap()));
    //
}

fn robba(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file or other problem");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse lines"))
        .collect()
}

//    // Read and parse json
//    let json_file_path = Path::new("../davide.json");
//    let file = File::open(json_file_path);
//    println!("ciao {:?}", file);
//
//    #[derive(Debug,Deserialize)]
//    #[serde(transparent)]
//    struct Jokes {
//        davide: Vec<String>
//
//    }
//    let jokes = serde_json::from_reader(file).expect("error");
//
//    Ok(jokes)
