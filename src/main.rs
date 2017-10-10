extern crate rand;

fn main() { 
    let x = rand::random::<u8>();
    println!("rust é {}", x);

    let y = add_num(1,2);
    println!("Result {}", y);

    let jose = Person::new(String::from("José"), 42);
    info(jose);
}

fn add_num(x:u8, y:u8)->u8{
    x+y
}

fn info(p:Person) {
    println!("{} tem {} anos", p.name, p.age)
}

struct Person {
    name: String,
    age: u8
}

impl Person {
    fn new(name:String, age:u8)->Person{
        Person{name, age}
    }
}


struct Nil; // unit struct

struct Pair(i32, f32); //tuple struct