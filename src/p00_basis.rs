use std::ops::Div;
use std::convert::From;

fn sum(x:u32, y:u32)->(){
    println!("{}",x+y);
}

//generic function:

fn divide<T: Div>(a: T, b: T) -> T
    where T: From<<T as Div>::Output>
{
    T::from(a / b)
}

// imp over a type

struct SlbType {
    number: i32,
}

impl SlbType{
    fn inc(&mut self){
        self.number += 1;
    }

    fn get_number(&self) -> i32{
        self.number
    }

    fn to_string(&self){
        println!("{}",self.number);
    }

}

pub fn main() {
    sum(10000000,333);

    //tuple
    let tup: (i32,i32,f32) = (123,321,1.2);
    println!("{}",tup.1);

    let arr:[i32;2] = [1,2];

    for i in arr.iter(){
        println!("{}",i);
    }

    println!("\nCollections ====");

    let mut myv =vec![4, 3];
    myv.push(5);
    myv.sort();

    println!("{:?}", myv);

    let myv: Vec<_> = myv.iter().map(|x| x * 2).collect();

    println!("{:?}", myv);

    for i in (0..myv.len()).rev(){
        println!("{}",myv[i]);
    }

    println!("\nGeneric function ====");
    // Note that
    let y = divide(1.1,2.2);
    assert_eq!(y,0.5);

    println!("\nimp  ====");

    let mut mytype = SlbType{
        number:10,
    };

    mytype.inc();
    mytype.to_string();


}


