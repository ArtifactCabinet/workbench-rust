// SHIT

// get os name

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::String;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// TODO: delete this trash string methods.
fn get_os_name()-> String {
    if let Ok(lines) = read_lines("/etc/os-release"){
        for line in lines{
            if let Ok(l) = line {
                if l.starts_with("NAME"){
                    let name_l:Vec<String> =l.split("=").map(|s| s.to_string()).collect();
                    let nice = name_l[1].strip_prefix("\"").unwrap().strip_suffix("\"").unwrap().to_string();
                    return nice
                }
            }
        }
    }
    return "".to_string();
}

fn main() {
    println!("{}",get_os_name())
}