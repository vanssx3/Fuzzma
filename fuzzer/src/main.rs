use std::process::Command;

fn main() {
    
    Command::new("python3")
        .arg("crackma/crackma.py")
        .arg("-b")
        .arg("1234")
        .arg("-p")
        .spawn()
        .expect("");
    
}
