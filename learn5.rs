fn main(){
    let number1 = 54i32;
    let number2 : i32 = 64;
    println!("number is {}", number1);
    println!("number is {}", number2);

    let mut number = 1;
    println!("{}", number);
    number = 2;
    println!("{}", number);

    let num = 1;
    // num = 2; <-error!!!
    println!("{}", num);
}