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

#[test]
fn char_type() {
    let a = 'c';

    println!("{}", a);
}

#[test]
fn tuple() {
    /*
    bisa berisi lebih dari 1 tipe data
    length is final, tidak bisa berkurang atau bertambah
    */

    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data); // :? untuk debug information

    println!("{}", data.0);

    let b = data.1;
    println!("{}", b);
}

#[test]
fn destructuring_tuple() {
    let data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    let (a, _, c) = data;
    println!("{} {}", a, c);
}

#[test]
fn mut_tuple() {
    let mut data: (i32, f64, bool) = (10, 10.5, true);
    println!("{:?}", data);

    data.0 = 9;
    data.1 = 9.5;
    data.2 = false;
    println!("{:?}", data);
}

fn unit() {
    // tuple kosong
    println!("Hello");
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result);
}

#[test]
fn array() {
    /* 
    sama seperti tuple tapi tipe datanya sejenis 
    */

    let array: [i32; 5] = [1, 2, 3, 4, 5]; // secara explisit
    // let array = [1, 2, 3, 4, 5]; secara implisit
    println!("{:?}", array);

    println!("{}", array[0]);

    let length = array.len();
    println!("{}", length);
}

#[test]
fn mut_array() {
    let mut array = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    array[0] = 10;
    array[1] = 20;

    println!("{:?}", array);
}

#[test]
fn two_dimensional_array() {
    let array2d = [[1, 2, 3], [4, 5, 6]];
    println!("{:?}", array2d);

    println!("{}", array2d[1][0]);
}

#[test]
fn constant() {
    const MINIMUM: &str = "Hello World!";
    println!("{}", MINIMUM);
}