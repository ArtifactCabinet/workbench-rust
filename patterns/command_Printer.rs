// This trait will be implemented in a lot of entities
// If you want, you can put them into Box<dyn Print>

// Thus, you can put Boxed "Print" into a execution queue

// TODO: other ways to do this>

pub trait Print {
    fn execute(&self) -> &str;
}

// Two commands PrintA & PrintB, 
// that share same Print trait

pub struct PrintA;
impl Print for PrintA {
    fn execute(&self) -> &str{
        "Print -> A"
    }
}

pub struct PrintB;
impl Print for PrintB {
    fn execute(&self) -> &str{
        "Print -> B"
    }
}

struct SlbPrinter{
    commands: Vec<Box<dyn Print>>,
    //        ^ heap allocation container
    // TODO: what is a dyn dispatch? Can it be used somewhere else?

}

impl SlbPrinter {
    fn new() -> Self  {
    //          ^ syntax sugar, Self
        Self {commands:vec![]}
    }

    fn add_printer(&mut self, cmd: Box<dyn Print>){
        self.commands.push(cmd);
    }

    fn execute(&self) -> Vec<&str>{
        self.commands
                .iter()
                .map(|cmd| cmd.execute())
                .collect()
    }

}

fn main(){
    let mut my_printer = SlbPrinter::new();

    my_printer.add_printer(Box::new(PrintA));
    my_printer.add_printer(Box::new(PrintB));
    my_printer.add_printer(Box::new(PrintA));
    my_printer.add_printer(Box::new(PrintB));
    my_printer.add_printer(Box::new(PrintB));
    my_printer.add_printer(Box::new(PrintB));

            
    for x in my_printer.execute(){
        println!("{} ",x);
    }


}