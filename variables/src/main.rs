const PI: f64 = 3.14159265359;
fn main() {
    let a; // Declaration; without data type
    a = 5; // Assignment

    println!("{}", a);

    let b: i8; // Declaration; with data type 
    b = 5;

    println!("{b}");

    let t = true;        // Declaration + assignment; without data type
    let f: bool = false; // Declaration + assignment; with data type

    println!("{t}");
    println!("{f}");

    let (x, y) = (1, 2); // x = 1 and y = 2
    println!("{x}");
    println!("{y}");

    let mut z = 5;
    z = 6;
    println!("{z}");

    let z = { x + y }; // z = 3

    let z = {
        let x = 1;
        let y = 2;

        x + y
    }; // z = 3
    println!("{z}");

    let x: f64 = -20.48; // float
    let x: i64 = x.floor() as i64; // int
    println!("{}", x); // -21

    let s: &str = "hello"; // &str
    let s: String = s.to_uppercase(); // String
    println!("{}", s);// HELLO

    println!("π value is {}", PI);
}
