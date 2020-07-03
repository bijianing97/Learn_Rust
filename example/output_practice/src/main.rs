use std::fmt;
#[derive(Debug)]
struct MinMax(i64,i64);
impl fmt::Display for MinMax{
    fn fmt(&self, f:&mut fmt::Formatter)->fmt::Result{
        write!(f,"({},{})",self.0, self.1)
    }
}
struct  Point2D{
    x: f64,
    y: f64
}

struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self , f:&mut fmt::Formatter) -> fmt::Result{
        let vec = &self.0;
        write!(f,"[")?;

        for (count, v) in vec.iter().enumerate(){

            if count != 0{write!(f,",")?;}
            write!(f," {}: {}",count,v)?;
        }
        write!(f,"]")
    }
}

struct City{
    name:&'static str,
    lat : f32,
    lon : f32
}

impl fmt::Display for City{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
        let lon_c = if self.lon >= 0.0 {'E'} else {'W'};
        write!(f,"{} : {:.3}°{},{:.3}°{}. ",self.name, self.lat.abs(),lat_c,self.lon.abs(),lon_c)
    }
}

struct Color{
    red:u8,
    green:u8,
    blue:u8,
}

impl fmt::Display for Color{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f,"RGB:({},{},{}) 0x{:02X}{:02X}{:02X} ",self.red, self.green, self.blue,self.red, self.green, self.blue)
    }
}
fn main() {
    println!("{}",31);

    println!("{0}, this is {1} . {1}, this is {0}","Tom","mari");

    println!("{subject} ,{verb} ,{object}",object= "the lazy dog",verb = "jumps over",subject="fox");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);//:b以二进制输出
    
    println!("{number:>width$}",number = 1, width =6);

    println!("{number:>0width$}",number = 1, width =6);

    //println!("{}",Structure(3));
    // fmt::Debug ues the {:?} 
    // fmt::Display use the {}

    #[derive(Debug)]
    struct Structure(i32);

    #[derive(Debug)]
    struct Deep(Structure);

    let x = Structure(3);
    println!("{:?}",x);

    let goodval = MinMax(3,2);
    let badval  = MinMax(-3, -2);
    println!("badvalue is {bad},goodvalue is {good}",bad=badval,good=goodval);
    
    println!("badvalue is {:?}",badval);

    let v =List(vec![1, 2, 3]);
    println!("{}",v);

    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ].iter() {
        println!("{}", *city);
    }

    for color in [
        Color{red: 128, green:255, blue: 90},
        Color{red:   0, green:122, blue: 100},
    ].iter(){
        println!("{}",color);
    }
}
