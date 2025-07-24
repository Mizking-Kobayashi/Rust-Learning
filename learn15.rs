use std::convert::From;

#[derive(Debug)]
#[allow(dead_code)]

struct Number {
    val : i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number{ val : item}
    }
}

fn main(){
    let num1 = Number::from(31);
    println!("num1 = {:?}", num1);

    let int = 128i32;
    let num2 : Number = int.into();
    println!("num2 = {:?}", num2);
}