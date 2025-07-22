fn main(){
    println!("1 + 2 = {}", 1u32 + 2);
    println!("1 - 2 = {}", 1i32 - 2);

    // println!("1 - 2 = {}", 1u32 - 2);    <- error!!!

    println!("Not true is {}", !true);
    println!("true and true is {}", true && true);
    println!("true and false is {}", true && false);
    println!("true or true is {}", true || true);
    println!("true or false is {}", true || false);
    println!("false or false is {}", false || false);

    println!("0011 and 1111 is {:04b}", 0b0011u32 & 0b1111);
    println!("1011 and 1000 is {:04b}", 0b1011u32 | 0b1000);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0011 << 2 is {:b}", 0b0011u32 << 2);
}