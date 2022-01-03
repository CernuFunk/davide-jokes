use notify_rust::Notification;
use rusty_audio::Audio;

fn main() {
    Notification::new().summary("Firefox Vaffanculi")
        .body("Ciao ciao ciao")
        .icon("firefox")
        .show().unwrap();
    
    let mut audio = Audio::new();
    audio.add("davide", "/home/cerfu/Projects/davide-jokes/davide/src/audio.mp3");
    audio.play("davide");
    audio.wait();

}


