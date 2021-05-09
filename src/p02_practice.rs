pub fn add (a:i64,b:i64) -> i64 {
    a+b
}

pub trait IsOdd {
    fn is_odd(&self) -> bool;
}

impl IsOdd for usize {
    fn is_odd(&self) -> bool{
        self&1 != 0
    }
}


// filter pattern
fn filter(){
    let names = vec![1,2,3,4,5];
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add(){
        assert_eq!(add(1,2),3)
    }

    #[test]
    fn test_odd(){
        assert_eq!(IsOdd::is_odd( &3),true)
    }
}