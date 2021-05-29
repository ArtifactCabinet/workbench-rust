use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main(){
    let path = Path::new("test.csv");

    let mut file = match File::open(&path){
        Err(_why) => panic!("No"),
        Ok(file) => file,
    };

    let mut s = String::new();
    
    match file.read_to_string(&mut s){
        Err(_why) => panic!("No str"),
        Ok(_) => (),
    }

    let split = s.split("\n");
    
    for a in split{
        println!("Splitting line");

        let temp = a.split(",");
        for b in temp{
            println!(" => {}",b);
        }
    }
}