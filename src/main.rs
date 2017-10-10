extern crate rand;

fn main() { 
    // using random
    let x = rand::random::<u8>();
    println!("rust é {}", x);

    // using function add_num
    let y = add_num(1,2);
    println!("Result {}", y);

    // using function person
    let jose = Person::new(String::from("José"), 42);
    info(jose);

    // using function print
    let z = String::from("Rust !");
    print(&z);
    print(&z);

    // using tipos multaveis
    let mut vector:Vec<u8> = vec![1,2,3];
    vector.push(4);
    println!("Tipo multavel {:?}", vector)    
}

fn print(s:&String) {
    println!("Func Print {}", s)
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