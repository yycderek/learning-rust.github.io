fn main() {
    println!("Hello World");
    println!("{}, {}", "Hello", "World");
    let (greeting, name) = ("Hello", "World");
    println!("{greeting}, {name}");

    println!("{:?}", [1, 2, 3]); // [1, 2, 3]
    println!("{:#?}", [1, 2, 3]);

    let x = format!("{}, {}!", "Hello", "world");
    println!("{}", x); // Hello, world!

    // 💡 Rust has a print!() macro as well
    print!("Hello, world!"); // Without new line
    println!(); // A new line

    print!("Hello, world!\n");
}