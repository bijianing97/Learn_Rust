#[allow(dead_code)]
#[derive(Debug)]
struct rectangle{
    width:u32,
    height:u32,
}

impl rectangle{
    fn area(&self) -> u32{
        self.width*self.height
    }

    fn can_hold(&self, other: &rectangle) -> bool{
        return self.width > other.width && self.height > other.height;
    }
}

fn area(rectangle:&rectangle)->u32{
    rectangle.width*rectangle.height
}

#[derive(Debug)]

struct Person<'a>{
    name:&'a str,
    age:u8,
}


struct Pair (i32,i32);

struct Point{
    x:f32,
    y:f32,
}

#[allow(dead_code)]
struct Rectangle{
    p1:Point,
    p2:Point,
}

#[derive(Debug)]
struct User{
    username:String,
    email:String,
    sign_in_account:u64,
    active:bool,
}

fn build_user(email:String, username:String) -> User{
    User{
        email,
        username,
        active:true,
        sign_in_account:1,
    }
}

enum VeryVerboseEnumOfThingsToDowithNumbers{
    Add,
    Subtact,
}
type Operations = VeryVerboseEnumOfThingsToDowithNumbers;

impl VeryVerboseEnumOfThingsToDowithNumbers{
    fn run(&self, x:i32, y:i32) -> i32{
        match self {
            Self::Add => x+y,
            Self::Subtact => x-y,
        }
    }
}
enum Status {
    Rich,
    Poor
}
// enum创建链表

use List::*;

enum List{
    Cons(u32,Box<List>),
    Nil,//末节点，链表结束标志
}

impl List{
    fn new() -> List{
        Nil
    }

    fn prepend(self,elum:u32)->List{
        Cons(elum,Box::new(self))
    }

    fn len(&self) -> u32{
        match self{
            Cons(_,ref tail) => tail.len()+1,
            Nil => 0,
        }
    }

    fn stringify(&self) -> String{
        match self{
            Cons(head,ref tail)=>{
                format!("{},{}",head,tail.stringify())
            },
            Nil =>{
                format!("Nil")
            },
        }
    }
}



fn main(){
    let s = build_user("XXXXXX@qq.com".to_string(),"bjn".to_string());
    println!("{:#?}",s);
    let rect1 = rectangle{width:32,height:34};
    println!("{:#?}",rect1);
    println!("The area is {}",area(&rect1));
    println!("area of the rectangle is {} ",rect1.area());
    let rect2 = rectangle{width:33,height:35};
    println!("the rect2 can hold rect1 : {}",rect2.can_hold(&rect1));
    use Status::{Rich,Poor};
    use Status::*;
    let status = Poor;
    match status{
        Rich => println!("wow,so rich!"),
        Poor => println!("wow, so poor!"),
    }

    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("length :{}",list.len());
    println!("{}",list.stringify());

    fn calculate_length(s: &String) -> usize {
        let leng = s.len();
        leng
    }
    let s1 = calculate_length(&String::from("hello"));
    println!("length is {}",s1);
    
}
