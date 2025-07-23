enum Person {
    Name(String),
    Age(u8),
    Id(u32),
    Comment(String),
}

fn attention(info: Person){
    match info {
        Person::Name(s) => println!("Name = {}", s),
        Person::Age(i) => println!("Age = {}", i),
        Person::Id(i) => println!("Id = {}", i),
        Person::Comment(s) => println!("Comment = {}", s),
    }
}

fn main(){
    let name = Person::Name(String::from("Alice"));
    let age = Person::Age(20u8);
    let id = Person::Id(72512345u32);
    let comment = Person::Comment(String::from("Hi, I'm Alice, a student of keio university."));

    
    attention(name);
    attention(age);
    attention(id);
    attention(comment);
}