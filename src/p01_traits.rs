// basic fn def
fn more_than_two(x:i32) -> bool {
    return match x {
        i if i > 2 => true, // @match guard
        _ => false
    }
}

struct Number {
    value: i32,
}

// trait is common s
trait Signed {
    fn is_strictly_negative(self)->bool;
}

impl Signed for Number {
    fn is_strictly_negative(self) -> bool {
        self.value < 0
    }
}

pub trait Summary {
    fn print(&self);
}
// add a trait for built-in types
impl Summary for i32{
    fn print(&self)  {
        println!("{}", self)
    }
}



pub fn main() {
    let n = Number { value: -44 };
    println!("{}", n.is_strictly_negative()); // prints "true"
    println!("{}",more_than_two(2));

    let n:i32 = 13;
    n.print()

}