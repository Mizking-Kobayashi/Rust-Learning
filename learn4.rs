fn main(){
    println!("{number:0>5}", number=1);
    println!("{number:0>5}", number=12);
    println!("{number:0>5}", number=123);
    println!("{number:0>5}", number=1234);
    println!("{number:0>5}", number=12345);

    println!("");

    println!("{number:>width$}", number=3, width=5);

    println!("");
    println!("{number:0<5}", number=1);
    println!("{number:0<5}", number=12);
    println!("{number:0<5}", number=123);
    println!("{number:0<5}", number=1234);
    println!("{number:0<5}", number=12345);
}
