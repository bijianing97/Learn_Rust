#![allow(overflowing_literals)]
use std::convert::From;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::string::ToString;
use std::fmt;
#[derive(Debug)]
struct Number{
    value:i32,
}
impl From<i32> for Number{
    fn from(item:i32) -> Self{
        Number{value:item}
    }
}

#[derive(Debug,PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber{
    type Error= ();
    fn try_from(value:i32) -> Result<Self, Self::Error>{
        if value%2 == 0{
            Ok(EvenNumber(value))
        }else{
            Err(())
        }
    } 
}

struct Circle{
    radius:i32
}

impl ToString for Circle{
    fn to_string(&self) -> String{
        format!("Circle of radius{:?}",self.radius)
    }
}

struct MinMax(i64,i64);

impl fmt::Display for MinMax{
    fn fmt(&self,f:&mut fmt::Formatter) -> fmt::Result{
        write!(f,"({},{})",self.0,self.1)
    }
}

fn main(){
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}",decimal, integer, character);
    println!("{}",(-1_i8) as u8);

    let elem = 5u8;
    let mut vec = Vec::new();

    vec.push(elem);
    println!("{:?}",vec);

    let int = 5;
    let num= Number::from(32);
    let num1 :Number = int.into();
    println!("{:?}",num);
    println!("{:?}",num1);

    let minmax = MinMax(1,14);
    println!("{}",minmax);

    let circle = Circle{radius : 6};
    println!("{}",circle.to_string());

    let parsed :i32  = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed +turbo_parsed;
    println!("Sum:{:?}",sum);
}