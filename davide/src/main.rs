use notify_rust::Notification;
fn main() {
    Notification::new().summary("Firefox Vaffanculi")
        .body("Ciao ciao ciao")
        .icon("firefox")
        .show().unwrap();
}
