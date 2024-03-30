// Date: 2021-09-26
// Author: Danilo Aparecido
// https://www.youtube.com/watch?v=KGsNhOo4nTc

//  Aula 06 - Rust - Estruturas de Controle

// Debug no rust com println!
// Instalando plugins para o VSCODE
// Rust (rls) e CodeLLDB
// gdb

/*
Claro! Aqui vai um exercício interessante que você pode tentar em Rust:
Exercício: Calculadora de Média Ponderada
Escreva um programa em Rust que calcule a média ponderada de uma série de notas de alunos.
O programa deve primeiro perguntar quantas notas serão fornecidas. Em seguida, para cada nota,
o programa deve solicitar a nota do aluno e o peso dessa nota. Ao final, o programa deve exibir
a média ponderada das notas. A média ponderada é calculada somando-se todas as notas multiplicadas
por seus respectivos pesos, dividido pela soma dos pesos.

Critérios:
1 - Use um loop para solicitar as notas e os pesos.
2 - Valide as entradas para garantir que notas e pesos sejam números positivos.
3 - Use um tipo de dado adequado para armazenar as notas e os pesos.
4 - O programa deve funcionar para qualquer quantidade de notas.
5 - Exiba a média ponderada com duas casas decimais.
Boa sorte! Se precisar de ajuda com a implementação, estou aqui.
*/

fn main() {
    println!("Rust - Estruturas de Controle");

    // para enviar o código no github para chat youtube
    //nome do usuario - nome do repositorio

    // Exercício 01
    // criar um programa de tabuada
    // tabuada do 1 ao 10
    // onde eu digito o multiplicador
    // exemplo
    // multiplicador = 2
    // e mostra os resultado
    // 2 x 1 = 2
    // 2 x 2 = 4
    // 2 x 3 = 6

    let mut multiplicador: i32 = 0;
    let mut resultado: i32 = 0;

    println!("Digite o multiplicador: ");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");
    let multiplicador: i32 = input
        .trim()
        .parse()
        .expect("Por favor, digite um número inteiro");

    for i in 1..=10 {
        let resultado = multiplicador * i;
        println!("{} x {} = {}", multiplicador, i, resultado);
    }
}

/*

**********************************
let mut number: i32 = 10;
    // loop - range
    for number: i32 in 1..10 {
        println!("O valor de nmber é: {}", number);
    }

    println!("*******************************");

    for number in 1..70 {
        println!("O valor de nmber é: {}", number);
    }

**********************************
while x < 20 {
        if x == 15 {
            x = x + 1;
            continue;
        }
        if x == 18 { // break - sai do loop
            break;
        }
        println!("O valor de x é: {}", x);
        x = x + 1;
    }

**********************************
loop
loop {
        println!("Loop infinito");
        x = x + 1;

        if x == 20 {
            continue;
        }

        if x == 20 {
            break;
        }
    }
***********************************

let numero: i8 = 3;
    let resultado: i8 = if numero > 3 { 4 } else { 6 };

    println!("O resultado é: {}", resultado);

fn mostra_dados() {
    let x: i8 = 10;
    let y: i8 = 20;
    let z: i8 = x + y;

    println!("O valor de x é: {}", x);
}

// Pather match
    let numero: i8 = 3;
    match numero {
        1 => println!("O número é 1"),
        2 => println!("O número é 2"),
        3 => println!("O número é 3"),
        _ => println!("O número não é 1, 2 ou 3"),
    }
*/
