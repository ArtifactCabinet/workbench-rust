use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {

    let halt:u8 = 00;
    let load:u8 = 01;
    let  inc:u8 = 02;
    let  add:u8 = 03;

    let mut mdr1:u8; //memory address 1
    let mut mdr2:u8; 
    let mut mdr3:u8; 

    let mut data:u32=00000000;

    {
        let mut file = File::create("foo.hex")?;
        file.write(&[load])?;
        
    }

    {
        let mut file = File::open("foo.hex")?;
        let mut buffer = Vec::<u8>::new();
        file.read_to_end(&mut buffer)?;
        println!("{:?}",buffer);
    }

    Ok(())
}
