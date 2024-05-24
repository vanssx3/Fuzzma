use std::process::Command;

fn main() {
    let list = vec!["crack", "-p", "1234", "😈"];
    let mut i = 0;
    i = 0;
    while i < list.len() {
        print!("{}", list[i].to_owned() + " 1");
        Command::new("python3")
            .arg("crackma/crackma.py")
            .arg(list[i])
            .arg("1234")
            .arg("-p")
            .spawn()
            .expect("");
        i = i + 1;
    }

    i = 0;
    while i < list.len() {
        print!("{}", list[i].to_owned() + " 2");
        Command::new("python3")
            .arg("crackma/crackma.py")
            .arg("-b")
            .arg(list[i])
            .arg("-p")
            .spawn()
            .expect("");
        i = i + 1;
    }

    i = 0;
    while i < list.len() {
        print!("{}", list[i].to_owned() + " 3");
        Command::new("python3")
            .arg("crackma/crackma.py")
            .arg("-b")
            .arg("1234")
            .arg(list[i])
            .spawn()
            .expect("");
        i = i + 1;
    }
}
