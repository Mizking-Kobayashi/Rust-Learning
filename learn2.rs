fn main(){
    let name1 = "Alice";
    let name2 = "Bob";

    println!("{} loves {}", name1, name2);
    println!("{1} loves {0}", name1, name2);
    println!("{n1} loves {n2}", n1=name1, n2=name2);
}