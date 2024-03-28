/*

Variáveis e Tipos de Dados em Rust
1) - Inteiros: i8, i16, i32, i64, i128, u8, u16, u32, u64, u128, isize, usize
        Quantidade de caracteres que cabem em um byte: 8 bits = 1 byte
        exemplo: i8 = -128 a 127
                 u8 = 0 a 255
                 i16 = -32768 a 32767
                 u16 = 0 a 65535
                 i32 = -2147483648 a 2147483647
                 u32 = 0 a 4294967295
                 i64 = -9223372036854775808 a 9223372036854775807
                 u64 = 0 a 18446744073709551615
                 i128 = -170141183460469231731687303715884105728 a 170141183460469231731687303715884105727
                 u128 = 0 a 340282366920938463463374607431768211455
                 isize = depende do sistema operacional (32 ou 64 bits)
                 usize = depende do sistema operacional (32 ou 64 bits)

        Exemplo de código com variáveis inteiras de i8:
        let x: i8 = 10;
        let y: i8 = -10;
        let soma: i8 = x + y;
        Resultado: 0
        println!("Soma: {}", soma);

        Exemplo de código com variáveis inteiras de i16:
        let x: i16 = 10;
        let y: i16 = -10;
        let soma: i16 = x + y;
        Resultado: 0
        println!("Soma: {}", soma);

        Exemplo de código com variáveis inteiras de i32:
        let x: i32 = 10;
        let y: i32 = -10;
        let soma: i32 = x + y;
        Resultado: 0
        println!("Soma: {}", soma);

        Exemplo de código com variáveis inteiras de i64:
        let x: i64 = 10;
        let y: i64 = -10;
        let soma: i64 = x + y;
        Resultado: 0
        println!("Soma: {}", soma);

        Exemplo de código com variáveis inteiras de i128:
        let x: i128 = 1005981760517468179772955289344;
        let y: i128 = 1232681260876;
        let soma: i128 = x - y;
        Resultado: 0
        println!("Soma: {}", soma);

        Exemplo de código com variáveis inteiras de isize:
        let x: isize = 10;
        let y: isize = -10;
        let divisao: isize = x / y;
        Resultado: 0
        println!("Soma: {}", divisao);

        Exemplo de código com variáveis inteiras de usize:
        let x: usize = 10;
        let y: usize = 10;
        let multiplicacao: usize = x * y;
        Resultado: 100
        println!("Soma: {}", multiplicacao);

2) - Ponto Flutuante: f32, f64
        Quantidade de caracteres que cabem em um byte: 8 bits = 1 byte
        exemplo: f32 = 32 bits
                 f64 = 64 bits

        Exemplo de código com variáveis de ponto flutuante de f32:
        let x: f32 = 10.5;
        let y: f32 = -10.5;
        let soma: f32 = x + y;
        Resultado: 0
        println!("Soma: {}", soma);

        Exemplo de código com variáveis de ponto flutuante de f64:
        let x: f64 = 10.5;
        let y: f64 = -10.5;
        let soma: f64 = x * y;
        Resultado: -110.25
        println!("Soma: {}", soma);

        Exemplo de código com variáveis de ponto flutuante de f64:
        let x: f64 = 10.5;
        let y: f64 = -10.5;
        let soma: f64 = x / y;
        Resultado: -1
        println!("Soma: {}", soma);

        Exemplo de código com variáveis de ponto flutuante de f64:
        let x: f64 = 10.5;
        let y: f64 = -10.5;
        let soma: f64 = x - y;
        Resultado: 21
        println!("Soma: {}", soma);

3) - Booleano: bool
        Valores: true ou false

        Exemplo de código com variáveis booleanas:
        let x: bool = true;
        let y: bool = false;
        let z: bool = x && y;
        Resultado: false
        println!("Soma: {}", z);

        Exemplo de código com variáveis booleanas:
        let x: bool = true;
        let y: bool = false;
        let z: bool = x || y;
        Resultado: true
        println!("Soma: {}", z);

        Exemplo de código com variáveis booleanas:
        let x: bool = true;
        let y: bool = false;
        let z: bool = !x;
        Resultado: false
        println!("Soma: {}", z);

        // Estudo de caso da tabela verdade
        let x: bool = true;
        let y: bool = false;
        let z: bool = x && y;
        println!("{} AND {} = {}", x, y, z);

        let x: bool = true;
        let y: bool = false;
        let z: bool = x || y;
        println!("{} OR {} = {}", x, y, z);

        let x: bool = true;
        let z: bool = !x;
        println!("NOT {} = {}", x, z);

4) - Caractere: char
        Tamanho: 4 bytes
        Exemplo: 'a', '1', '😀'

        Exemplo de código com variáveis de caractere:
        let x: char = 'a';
        let y: char = '1';
        let z: char = '😀'
        let w: char = x + y + z;
        println!("Soma: {}", x);
        println!("Soma: {}", y);
        println!("Soma: {}", z);

5) - Tupla: (T1, T2, T3, ..., Tn)
        Exemplo: (1, 2, 3, 4, 5)
                 (1, "Olá", 'a', true)

                 Exemplo de código com variáveis de tupla:
                        let x: (i32, i32, i32) = (1, 2, 3);
                        let y: (i32, i32, i32) = (4, 5, 6);
                        let z: (i32, i32, i32) = x + y;
                        println!("Soma: {:?}", z);

6) - Array: [T; N]
        Exemplo: [1, 2, 3, 4, 5]
                 ["Olá", "Mundo"]
                 ['a', 'b', 'c']

        Exemplo de código com variáveis de array:
        let x: [i32; 5] = [1, 2, 3, 4, 5];
        let y: [i32; 5] = [6, 7, 8, 9, 10];
        let z: [i32; 5] = x + y;
        println!("Soma: {:?}", z);

        Exemplo de código com variáveis de array:
        let x: [i32; 5] = [1, 2, 3, 4, 5];
        let y: [i32; 5] = [6, 7, 8, 9, 10];
        let z: [i32; 5] = x - y;
        println!("Soma: {:?}", z);

        Exemplo de código com variáveis de array:
        let x: [i32; 5] = [1, 2, 3, 4, 5];
        let y: [i32; 5] = [6, 7, 8, 9, 10];
        let z: [i32; 5] = x * y;
        println!("Soma: {:?}", z);

        Exemplo de código com variáveis de array:
        let x: [i32; 5] = [1, 2, 3, 4, 5];
        let y: [i32; 5] = [6, 7, 8, 9, 10];
        let z: [i32; 5] = x / y;
        println!("Soma: {:?}", z);

        // Adicionando um elemento no fim do array
        let mut x: [i32; 5] = [1, 2, 3, 4, 5];
        x.push(6);
        println!("Array: {:?}", x);

        // Removendo um elemento no fim do array
        let mut x: [i32; 5] = [1, 2, 3, 4, 5];
        x.pop();
        println!("Array: {:?}", x);

        // Acessando um elemento do array
        let x: [i32; 5] = [1, 2, 3, 4, 5];
        let y: i32 = x[0];
        println!("Elemento: {}", y);

        // Adicionando um elemento no início do array
        let mut x: [i32; 5] = [1, 2, 3, 4, 5];
        x.insert(0, 0);
        println!("Array: {:?}", x);

        // Removendo um elemento no início do array
        let mut x: [i32; 5] = [1, 2, 3, 4, 5];
        x.remove(0);
        println!("Array: {:?}", x);

        // Acessando um elemento na metade do array
        let x: [i32; 5] = [1, 2, 3, 4, 5];
        let y: i32 = x[2];
        println!("Elemento: {}", y);

        // Algoritmo para busca mai simples
        let x: [i32; 5] = [1, 2, 3, 4, 5];
        let y: i32 = 3;
        let mut pos: i32 = -1;
        for i in 0..x.len() {
            if x[i] == y {
                pos = i as i32;
                break;
            }
        }
        println!("Posição: {}", pos);

7) - String: String
        Exemplo: "Olá, Mundo!"

8) - Slice: &str
        Exemplo: "Olá, Mundo!"
                 "Olá"
                 "Mundo"

9) - Referência: &T
        Exemplo: &1
                 &"Olá"
                 &'a'

10) - Mutável: mut
        Exemplo: let mut x: i32 = 10

11) - Constante: const
        Exemplo: const PI: f32 = 3.14

12) - Função: fn
        Exemplo: fn main() {}

13) - Estruturas: struct
        Exemplo: struct Pessoa {
                    nome: String,
                    idade: i32,
                    altura: f32
                 }

14) - Enumeração: enum
        Exemplo: enum Cor {
                    Vermelho,
                    Verde,
                    Azul
                 }

15) - Vetor: Vec<T>
        Exemplo: let v: Vec<i32> = Vec::new();

16) - Hash Map: HashMap<K, V>
        Exemplo: use std::collections::HashMap;
                 let mut map: HashMap<i32, String> = HashMap::new();

17) - Result: Result<T, E>
        Exemplo: let x: Result<i32, String> = Ok(10);

18) - Option: Option<T>
        Exemplo: let x: Option<i32> = Some(10);

19) - Loop: loop
        Exemplo: loop {
                    println!("Olá, Mundo!");
                 }

20) - While: while
        Exemplo: let mut x: i32 = 0;
                 while x < 10 {
                    println!("Olá, Mundo!");
                    x += 1;
                 }

21) - For: for
        Exemplo: for x in 0..10 {
                    println!("Olá, Mundo!");
                 }

22) - If: if
        Exemplo: let x: i32 = 10;
                 if x > 0 {
                    println!("Olá, Mundo!");
                 }  else {
                    println!("Tchau, Mundo!");
                 }

23) - Match: match
        Exemplo: let x: i32 = 10;
                 match x {
                    0 => println!("Zero"),
                    1 => println!("Um"),
                    _ => println!("Outro valor")
                 }

24) - Trait: trait
        Exemplo: trait Animal {
                    fn som(&self);
                 }

25) - Implementação: impl
        Exemplo: struct Cachorro;
                 impl Animal for Cachorro {
                    fn som(&self) {
                        println!("Au, Au!");
                    }
                 }

26) - Módulo: mod
        Exemplo: mod animal {
                    pub trait Animal {
                        fn som(&self);
                    }
                 }

27) - Pacote: crate
        Exemplo: crate::animal::Animal

28) - Biblioteca Padrão: std
        Exemplo: use std::io;

29) - Biblioteca Externa: extern
        Exemplo: extern crate rand;

30) - Macro: macro
        Exemplo: println!("Olá, Mundo!");

31) - Closure: |args| {}
        Exemplo: let soma = |x, y| x + y;

32) - Generics: T
        Exemplo: struct Ponto<T> {
                    x: T,
                    y: T
                 }

33) - Lifetimes: 'a
        Exemplo: struct Ponto<'a> {
                    x: &'a i32,
                    y: &'a i32
                 }

34) - Unsafe: unsafe
        Exemplo: unsafe {
                    println!("Olá, Mundo!");
                 }

35) - Ponteiro: *
        Exemplo: let x: i32 = 10;
                 let p: *const i32 = &x;


Tipos de memória em Rust:
1) - Stack: LIFO (Last In, First Out)
2) - Heap: Memória alocada dinamicamente
3) - Data: Variáveis globais - Static
4) - Text: Código binário

Diferença entre memória Static, Heap, Stack, Data e Text:
1) - Static: Memória alocada em tempo de compilação
2) - Heap: Memória alocada em tempo de execução
3) - Stack: Memória alocada em tempo de execução
4) - Data: Memória alocada em tempo de compilação
5) - Text: Memória alocada em tempo de compilação

Qual a melhor forma de usar os tipos de memória do Rust?
1) - Stack: Variáveis com tamanho fixo
2) - Heap: Variáveis com tamanho variável
3) - Data: Variáveis globais
4) - Text: Código binário






*********************************************************
rustfmt - Formata o código de acordo com as regras do Rust
cargo fmt - Formata o código de acordo com as regras do Rust

ENTENDENDO O CÓDIGO:

1. fn main() - Função principal do programa
2. println!() - Macro que imprime uma mensagem na tela
3. let mut palpite: String - Declaração de uma variável mutável chamada palpite do tipo String
4. String::new() - Cria uma nova instância de String
5. std::io::stdin() - Acessa o input do usuário
6. .read_line(&mut palpite) - Lê a linha digitada pelo usuário e armazena na variável palpite
7. .expect() - Trata o erro caso ocorra
8. println!("Seu palpite: {}", palpite) - Imprime o palpite do usuário na tela

*/

// Imports
use inflector::Inflector;

fn main(){



}

/**
 
 
fn main() {
    // Tratamento de strings em Rust

    // Declaração de uma variável mutável chamada palpite do tipo String

    // Usando métodos da String em Rust



        let mut palpite: "string rust".to_string();
        println!("String: {}", palpite);

        // to_uppercase - Converte a string para maiúscula
        let s1: String = "Olá, Mundo!".to_uppercase();
        println!("String: {}", s1);

        // to_lowercase - Converte a string para minúscula
        let s2: String = "Olá, Mundo!".to_lowercase();
        println!("String: {}", s2);

        // capitalize - Converte a primeira letra da string para maiúscula
        let s3: String = "olá, mundo!".to_title_case();
        println!("String: {}", s3);

        // to_snake_case - Converte a string para snake_case
        let s4: String = "Olá, Mundo!".to_snake_case();
        println!("String: {}", s4);

        // to_kebab_case - Converte a string para kebab-case
        let s5: String = "Olá, Mundo!".to_kebab_case();
        println!("String: {}", s5);

        // to_camel_case - Converte a string para camelCase
        let s6: String = "Olá, Mundo!".to_camel_case();
        println!("String: {}", s6);

        // to_pascal_case - Converte a string para PascalCase
        let s7: String = "Olá, Mundo!".to_pascal_case();
        println!("String: {}", s7);

        // to_sentence_case - Converte a string para Sentence case
        let s8: String = "Olá, Mundo!".to_sentence_case();
        println!("String: {}", s8);

        // to_title_case - Converte a string para Title Case
        let s9: String = "Olá, Mundo!".to_title_case();
        println!("String: {}", s9);

        // contando o número de caracteres da string
        let contaChar: usize = "Olá, Mundo!".chars().count();
        println!("String: {}", contaChar);    
}

 * 
 * 
 * // Usado o trim para remover os espaços em branco
    let s: String = " Olá, Mundo! ".trim();
    println!("String: {_  _}", s);

    // retirando os espaços em branco da direita
    let s1: String = " Olá, Mundo! ".trim_end();
    println!("String: {_  _}", s1);

    // retirando os espaços em branco da esquerda
    let s2: String = " Olá, Mundo! ".trim_start();
    println!("String: {_  _}", s2);

    // replace - substitui uma string por outra
    let s3: String = "Olá, Mundo!".replace("Olá", "Rust");
    println!("String: {}", s3);
 */