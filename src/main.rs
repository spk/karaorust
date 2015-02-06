use std::os;
use std::io::BufferedReader;
use std::io::File;
use std::io::timer;
use std::time::duration::Duration;

fn main() {
    let args = os::args();
    if args.len() < 2 {
        println!("USAGE: {} [file]", args[0]);
        panic!();
    }
    let path = Path::new(args[1].as_slice());
    let mut file = BufferedReader::new(File::open(&path));
    let lines: Vec<String> = file.lines().map(|x| x.unwrap()).collect();

    for line in lines.iter() {
        if line.char_at(0) == '#' {
            let metadata: Vec<&str> = line.slice(1, line.len()).split_str(":").collect();
            println!("{} ~> {}", metadata[0], metadata[1]);
        } else if line.char_at(0) == ':' {
            let row: Vec<&str> = line.slice(2, line.len()).words().collect();
            let duration = row[1].parse::<i64>().unwrap() * 100;
            let interval = Duration::milliseconds(duration);
            let text = row.slice(3, row.len());
            println!("{} ", text);
            timer::sleep(interval);
        } else if line.char_at(0) == '-' {
            println!("{}", "");
        }
    }
}
