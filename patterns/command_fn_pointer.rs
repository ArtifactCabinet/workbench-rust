fn plus(x:i32,y:i32) -> i32 {
    x+y
}

fn minus(x:i32,y:i32) -> i32{
    x-y
}

fn apply(f:fn(i32,i32)->i32,arg1:i32,arg2:i32) -> i32 {
    f(arg1,arg2)
}

fn main(){
    println!("1+2={}\n1-2={}",apply(plus, 1, 2),apply(minus, 1, 2));
}