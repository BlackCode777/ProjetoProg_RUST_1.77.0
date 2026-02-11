fn main() {
    println!("Hello, world!");

    // Variaveis e constantes em rust
    let mut x: i32 = 10; // variavel mutável

    // Imutabilidade - por padrão, as variáveis em Rust são imutáveis
    // O que significa i32?
    // i32 é um tipo de dado inteiro de 32 bits com sinal. Ele pode armazenar valores inteiros que variam de -2.147.483.648 a 2.147.483.647.
    // O tipo i32 é comumente usado para representar números inteiros em Rust, e é uma escolha padrão para a maioria dos casos em que você precisa de um número inteiro. 
    // Ele é eficiente em termos de memória e desempenho, e é amplamente suportado em operações aritméticas e outras manipulações de dados.

    println!("O valor de x é: {}", x);
    x = 20;
    println!("O valor de x é: {}", x);
    const PI: f64 = 3.14159; // constante
    println!("O valor de PI é: {}", PI);

    // Tipos de dados em rust
    let inteiro: i32 = 42;
    let flutuante: f64 = 3.14;
    let booleano: bool = true;
    let caractere: char = 'R';
    let string: String = String::from("Rust");
    println!("Inteiro: {}, Flutuante: {}, Booleano: {}, Caractere: {}, String: {}", inteiro, flutuante, booleano, caractere, string);

}
