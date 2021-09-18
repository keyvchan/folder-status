use std::fs;

fn main() {
    let paths = fs::read_dir("./").unwrap();

    let mut count_dir = 0;
    let mut count_file = 0;
    for path in paths {
        count_dir = count_dir + 1;
        let metadata = path.as_ref().unwrap().metadata().unwrap();
        if metadata.is_dir() {
            count_dir = count_dir + 1
        }
        if metadata.is_file() {
            count_file = count_file + 1;
        }
    }
    println!(
        "This foldrer has:
    dirs: {}
    files: {}
    ",
        count_dir, count_file
    )
}
