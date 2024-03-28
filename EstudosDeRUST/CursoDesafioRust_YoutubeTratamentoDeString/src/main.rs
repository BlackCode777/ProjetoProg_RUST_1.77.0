use inflector::Inflector;

fn main() {
    let mut s = String::from("Hello, world!");
    println!("{}", s);

    // to_uppercase
    println!("{}", s.to_uppercase());

    // to_lowercase
    println!("{}", s.to_lowercase());

    // replace
    println!("{}", s.replace("world", "Rust"));

    // split
    for word in s.split(", ") {
        println!("{}", word);
    }

    // camelcase
    let camel_case = "hello_world".to_camel_case();
    println!("{}", camel_case);

    // snakecase
    let snake_case = "HelloWorld".to_snake_case();
    println!("{}", snake_case);
}
