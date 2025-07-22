fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (int_param, bool_param) = pair;

    //最後にセミコロンをつけなかったら返り値になる
    (bool_param, int_param)
}

fn main(){
    //タプルはさまざまな型を含めることができる
    let tup1 = (1i32, 12u32, true, false, 1.2f64, -43i32);
    println!("tup1.0 is {}", tup1.0);
    println!("tup1.1 is {}", tup1.1);

    let tup2 = ((1i32, 2u32), (2i32, 3u32), true);
    println!("tup2.0 is {:?}", tup2.0);
    println!("tup2.1 is {:?}", tup2.1);

    let tup3 = (15i32, true);
    println!("tup3 is {:?}", tup3);
    println!("reverse!!! is {:?}", reverse(tup3));

    let tup4 = (16i32,);
    println!("tup4 is {:?}", tup4);

    let tup5 = ("Alice", "Bob", "Carol", "Dave");
    let (a, b, c, d) = tup5;
    println!("a = {}, b = {}, c = {}, d = {}", a, b, c, d);
}