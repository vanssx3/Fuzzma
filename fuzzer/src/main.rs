use std::process::Command;

fn main() {
    
    let tests = ["🫠"];

    for i in tests {
        Command::new("python3")
            .arg("crackma/crackma.py")
            .arg("-d")
            .arg(i)
            .arg("-p")
            .spawn()
            .expect("");
    }
}
