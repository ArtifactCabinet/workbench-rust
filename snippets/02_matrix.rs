fn main(){

    const NROW:usize=2;
    const NCOL:usize=4;

    let mut matrix = [[0 as usize; NCOL]; NROW];
    //                      ^
    //                      | CRAP :)
     
    for row in 0..NROW{
        for col in 0..NCOL{
            matrix[row][col]= (row+1)*(col+1);
        }
    }

    for raw in 0..NROW{
        for col in 0..NCOL{
            print!("{} ",matrix[raw][col]);
        }
        println!();
    }
}
