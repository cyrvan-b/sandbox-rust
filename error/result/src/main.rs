use std::fs::File;

fn main() {
    let file = File::open("do-not-exist.txt");

    let _ = match file {
        Ok(file) => file,
        Err(error) => return,
    };
    return;
}
