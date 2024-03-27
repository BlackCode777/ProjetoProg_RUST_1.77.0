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


2) - Ponto Flutuante: f32, f64
        Quantidade de caracteres que cabem em um byte: 8 bits = 1 byte
        exemplo: f32 = 32 bits
                 f64 = 64 bits

3) - Booleano: bool
        Valores: true ou false

4) - Caractere: char
        Tamanho: 4 bytes
        Exemplo: 'a', '1', '😀'

5) - Tupla: (T1, T2, T3, ..., Tn)
        Exemplo: (1, 2, 3, 4, 5)
                 (1, "Olá", 'a', true)

6) - Array: [T; N]
        Exemplo: [1, 2, 3, 4, 5]
                 ["Olá", "Mundo"]
                 ['a', 'b', 'c']

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

fn main() {}
