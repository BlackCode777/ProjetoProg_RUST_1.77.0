
trait Veiculo { // Definindo a trait Veiculo que vai ser usada para simular a herança
    fn acelerar(&self) -> &String;
    fn frear(&self) -> &String;
}

struct Navio{ // Definindo a struct Navio que vai herdar as características da trait Veiculo
    marca: String,
    envergadura: f32,
    comprimento: f32,
}

impl Veiculo for Navio{ // Implementando a trait Veiculo para a struct Navio que herda as características da trait Veiculo
    fn acelerar(&self) -> &String
    fn frear(&self) -> &String
}


use chrono::Utc;
use std::io;

fn main() {

    // Aula 5
    // https://www.youtube.com/watch?v=G_1gkAcu1yw
    
    // Usando a biblioteca chrono
    // https://docs.rs/chrono/0.4.19/chrono/
    let mut ano_string = String::new();
    io::stdin().read_line(&mut ano_string).expect("Falha ao ler a linha");

    let ano_usuario: i32 = ano_string.trim().parse().expect("Falha ao converter o valor");
    let ano_atual: i32 = Utc::now().year();

}

/*
********************************
CONDICIONAIS EM RUST

if carro1.ano > 2019 {

        Se o carro1.ano for maior que 2019
        println!("O carro1 é novo");
    } else {
        println!("O carro1 é usado");
    };

     Agora preciso validar se o ano de fabricação do carro é maior do que 2020
     e se a cor do carro é vermelha

    Se o carro1.ano for maior que 2020 e a cor do carro1 for vermelha

    if carro1.ano > 2020 && carro1.cor == "vermelho" {
        println!("O carro1 é novo {} e a cor é {} ", carro1.ano, carro1.cor);

    Se o carro1.ano for menor ou igual a 2020 e a cor do carro1 for diferente de vermelho

    } else {
        println!("O carro1 é usado {} e a cor é {} ", carro1.ano, carro1.cor);
    };


**********************************
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

*/
