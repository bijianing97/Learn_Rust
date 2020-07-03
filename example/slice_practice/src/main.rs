use std::fmt;
fn reverse(pair:(i32,bool)) -> (bool,i32){
    let (integer, boolean) = pair;
    (boolean,integer)
}

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);

impl fmt::Display for Matrix{
    fn fmt(&self ,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"( {} {} )",self.0, self.1);
        write!(f,"\n( {} {} )",self.2, self.3)
    }
}
fn transpose(pair:Matrix) -> Matrix{
    let result:Matrix = Matrix(pair.0,pair.2,pair.1,pair.3);
    result
}

fn analyze_size(slice:&[i32]){
    println!("first elemnent is {}",slice[0]);
    println!("array length is {}",slice.len());
}
fn main(){
    let a_float:f64 = 1.0;
    let a_integer   = 5i32; //后缀说明

    let default_float = 3.0;

    println!("{}",1_000_000u32);

    let tuple_of_tuple = ((1u8, 2u16, 2u32),(4u64, -1i8),-2i16);

    println!("{:#?}",tuple_of_tuple);
    
    let matrix = Matrix(1.1,1.2,1.3,1.4);
    println!("{}",matrix);
    println!("{}",transpose(matrix));

    let xs:[i32;5] =[1,2,3,4,5];
    // 所有元素可以初始化成相同的值
    let ys: [i32; 500] = [100; 500];

    analyze_size(&ys[1..5]);
}
