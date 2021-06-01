macro_rules! say_yes {
    () => {
        println!("{:X}Yes!", 123);
    };
}

macro_rules! meta_hello {
    ($a_func_name_any:ident) => {
        fn $a_func_name_any() {
            println!("Hello, from {:?}()", stringify!($a_func_name_any));
        }
    };
}

meta_hello!(sophy);

macro_rules! evaluation_and_result {
    ($expression:expr) => {
        println!("{:?} = {:?}",
        stringify!($expression), 
        $expression)
    };
}

fn main() {
    say_yes!();
    sophy();

    evaluation_and_result!({
        let x = 1u64;
        x*x+2*x+1
    }) //"{ let x = 1u64; x * x + 2 * x + 1; }" = ()

}
