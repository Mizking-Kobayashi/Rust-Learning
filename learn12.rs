#[allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main(){
    use crate::Status::{Rich, Poor};
    use crate::Work::*;

    let status = Poor;
    let work = Civilian;

    match status {
        Rich => println!("You are a rich!!"),
        Poor => println!("You have no many..."),
    }

    match work {
        Civilian => println!("You are a civilian."),
        Soldier => println!("You are a soldier"),
    }
}
