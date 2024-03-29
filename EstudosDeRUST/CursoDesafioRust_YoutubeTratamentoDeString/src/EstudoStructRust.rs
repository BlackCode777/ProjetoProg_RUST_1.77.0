/*

ACESSANDO AS PROPRIEDADES DE UM STRUCT EM RUST (OBJETO)

struct Veiculos {
    marca: String,
    modelo: String,
    ano: u32,
    cor: String,
}

let carro = Veiculos {
    marca: String::from("Fiat"),
    modelo: String::from("Hatch"),
    ano: 2020,
    cor: String::from("vermelho"),
};

println!("Marca: {}", carro.marca);
println!("Modelo {} ", carro.modelo);
println!("Ano: {}", carro.ano);
println!("Cor: {}", carro.cor);

**********************************
Classe ( Struct ) em Rust que herda as características de outra classe ( Struct )



    Estudo de struct em Rust

    Aqui eu defini uma struct chamada Veiculos
    A struct Veicullos tem 4 campos: marca, modeo, ano e cor

    Todos os campos são do tipo String, exceto o campo ano que é do tipo u32;
    u32 é um tipo de dado inteiro sem final negativo, ou seja, ele só aceita valores positivos
    Uma observação ineressante é que o Rust não aceita acentos, então a palavra Veiculo está sem acentuação
    Um struct é uma forma de organizar e agrupar os dados em um único tipo de dado

    Agora eu vou criar uma função main() para testar a struct Veiculos
    Dentro da função main() eu vou criar uma instanca de carro1 do tipo Veiculos


    Agora como acessar os valores contidos na instancia de carro1?
    Para acessar os valores de uma struct, eu uso o operador ponto (.)
    O operador ponto é usado para acessar os campos de uma struct
    carro1.marca;
    carro1.modelo;
    carro1.ano;
    carro1.cor;

    Agora vou fazer condicionais para verificar se o carro1 é novo ou usado
    Se o carro1.ano for maior que 2019, então o carro1 é novo
    Se o carro1.ano for menor ou igual a 2019, então o carro1 é usado
    Como usar a condicional if em Rust?

    // Interessante em rust não existe o conceito de herança
// como em outras linguagens, mas é possível simular
// a herança com o uso de traits, segue  exemplo:

// Criando uma classe Carro que herda de Veiculos

// criando uma classe Navio que herda de Veiculos

// Criando uma classe Bicicleta que herda de Veiculos

// Criando uma classe Avião que herda de Veiculos


// impl Veiculo for Navio{
//     marca: String,
//     comprimento: f32,

//     // Implementando métodos da trait Veiculo
//     fn acelerar(&self){
//         println!("Acelerar o Navio!");
//     }

//     fn frear(&self){
//         println!("Frear o Navio!")
//     }
// }

// impl Veiculo for Bicicleta{
//     // Definindo campos
//     marca: String,
//     tipo: String,
//     cor: String,
//     ari: u32,
//     numero_marchas: u32,

//     // Implementando os métodos da trait Veiculo
//     fn acelerar(&self){
//         println!("Pedalar a Bicilceta!");
//     }

//     fn frear(&self){
//         println!("Frear a Bicicleta!");
//     }
// }


// impl Veiculo for Carro{
//     // Definindo os campos da struct Carro
//     marca: String,
//     modelo: String,
//     ano: u32,
//     cor: String,
//     portas: u32,

//     // Implementando os métodos da trait Veiculo
//     fn acelerar(&self){
//         println!("Acelerar o Carro!");
//     }

//     fn frear(&self){
//         println!("Frear o Carro!");
//     }

// }


// impl Veiculo for Aviao{
//     // Definindo os campos da struct Aviao
//     marca: String,
//     modeo: String,
//     ano: u32,
//     cor: String,
//     envergadura: f32,

//     // Implementando os métodos da trait Veiculo
//     fn acelerar(&self){
//         println!("Acelerar o Avião!");
//     }

//     fn frear(&self){
//         println!("Frear o avião! Embora avião freia com o motor!")
//     }
// }

// struct Aviao;
// struct Carro;
// struct Bicicleta;




*/
