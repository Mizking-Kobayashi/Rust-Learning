const A_CHAR : u8 = 0x41u8;
type Int = i32;
type Unint = u32;

fn main(){
    let character_a = A_CHAR as char;

    println!("charcter = {}", character_a);

    let mut vec = Vec::new();

    let num1 = 32u8;
    let num2 = 64u8;
    
    vec.push(num1);
    vec.push(num2);

    println!("vector = {:?}", vec);

    let num3 : Int = 32;
    let num4 : Unint = 53;

    println!("{}", num3 as Unint + num4);
}