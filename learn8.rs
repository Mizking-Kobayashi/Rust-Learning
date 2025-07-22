use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main(){
    let xs : [i32; 5] = [1, 2, 3, 4, 5];
    let zeros : [i32; 10] = [0; 10];

    println!("xs[0] is {}", xs[0]);
    println!("xs[1] is {}", xs[1]);
    println!("xs is {:?}", xs);

    println!("zeros : {:?}", zeros);
    println!("size of zeros is {}", zeros.len());

    println!("{}", mem::size_of_val(&xs));

    analyze_slice(&xs);

    analyze_slice(&zeros[1 .. 4]);
    println!("zeros(1~3) = {:?}", &xs[1 .. 4]);
}