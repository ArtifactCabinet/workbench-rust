// Boos, Kevin, Namitha Liyanage, Ramla Ijaz, and Lin Zhong. "Theseus: an
// Experiment in Operating System Structure and State Management." In 14th
// {USENIX} Symposium on Operating Systems Design and Implementation ({OSDI}
// 20), pp. 1-19. 2020.

fn main(){

    // In Rust, every value has an one owner

    let hel:&str;
    
    {
        let hello = String::from("hello!");

        let borrowed_str: &str = &hello;
//                        ^ transfer ownerships

//        consume(hello);   // Not possible, because ownership of hello already transferred.
        
        consume(borrowed_str.to_string()); // YES, because borrowed_str own the "Hello!"

        hel=substr(borrowed_str);
        println!("{}",hel);
    }

    // println!("{}",hel); => lifetime error

}
//                                   
//                         ▼          ▼ 
fn substr<'a> (input_str: &'a str) -> &'a str{
//       ▲
//return value have same lifetime as input_str

    &input_str[0..3]
}

fn consume(owned_string: String){
    println!("{}",owned_string);
}