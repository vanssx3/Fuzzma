use std::process::Command;

fn main() {
    println!("Attempting to open Firefox");
    Command::new("firefox")
            .arg("attack.html")
            .spawn()
            .expect("Page unable to open");
    println!("Opened Succesfully");
    
}