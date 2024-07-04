fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello Test!");
}

#[test]
fn test_variable() {
    let name1 = "Dimas";
    let name2 = "Saputro";
    println!("Hello {} {}", name1, name2);
}

#[test]
fn test_mutable() {
    let mut name = "Dimas";
    println!("Hello {}", name);
    
    name = "Eko";
    println!("Hello {}", name);
}

#[test]
fn static_typing() {
    let name = "Dimas Saputro";
    println!("Hello {}", name);

    // name = 10;
    println!("Hello {}", name);
}

#[test]
fn shadowing() {
    let name = "Dimas Saputro";
    println!("Hello {}", name);

    let name = 10;
    println!("Hello {}", name);
}

#[test]
fn comment() {
    // komentar satu baris
    /*
    komentar 
    lebih dari satu baris
    */
    println!("comment"); // ini juga komentar
}

#[test]
fn explicit() {
    let age: i32= 20;
    println!("{}", age);
}

#[test]
fn number() {
    let a = 2.5;
    let b = 10;

    println!("{} {}", a, b);
}

#[test]
fn number_conversion() {
    let a: i8 = 10;
    println!("{}", a);

    let b: i16 = a as i16;
    println!("{}", b);

    // integer overflow
    let d: i64 = 10000000000000000;
    let e: i8 = d as i8;
    println!("{}", e); // output: 0
}

#[test]
fn numeric_operator() {
    let a = 10;
    let b = 10;
    let c = a * b;
    println!("{}", c);
    let d = a + b;
    println!("{}", d);
    let e = a - b;
    println!("{}", e);
    let f = a + b;
    println!("{}", f);
}

#[test]
fn augmented_assignment() {
    let mut a = 10;
    println!("{}", a);
    a += 10;
    println!("{}", a);
    a -= 10;
    println!("{}", a);
    a *= 10;
    println!("{}", a);
    a /= 10;
    println!("{}", a);
    a %= 10;
    println!("{}", a);
}

#[test]
fn boolean() {
    let a = true;
    let b: bool = false;
    println!("{} {}", a, b);
}


