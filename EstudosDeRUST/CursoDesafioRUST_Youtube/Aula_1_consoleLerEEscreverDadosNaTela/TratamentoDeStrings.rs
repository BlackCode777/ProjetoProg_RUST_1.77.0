fn main() {
    // Tratamento de Strings em Rust
    let string1: String = String::from("Tratando String em Rust");
    let string2: &str = "Tratando String em Rust"; // &str = Slice de String
    let string3: String = "".to_string(); // &str = Slice de String
    let string4: String = String::new();
    let string5: String = format!("{} {}", "Tratando", "String em Rust"); // Concatenação de Strings
    let string6: String = "Tratando".to_string() + " String em Rust"; // Concatenação de Strings

    println!("String 1: {}", string1);
    println!("String 2: {}", string2);
    println!("String 3: {}", string3);
    println!("String 4: {}", string4);
    println!("String 5: {}", string5);
    println!("String 6: {}", string6);
}
