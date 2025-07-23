#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
    id: String,
}

//関数名はスネークケース(snake_case)がいいらしい
fn disp_user(info: &Person) -> (&String, u8, &String){
    println!("name = {}, age = {}, id = {}", &info.name, info.age, &info.id);
    (&info.name, info.age, &info.id)
}
fn main(){
    let peter = Person { 
        name: String::from("Peter"),
        age: 18u8,
        id: String::from("Funny")
    };

    let alice = Person {
        name: String::from("Alice"),
        age: 20u8,
        id: String::from("Sad")
    };

    disp_user(&peter);
    disp_user(&alice);
}