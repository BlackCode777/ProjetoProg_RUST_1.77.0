// Para rodar os programas em RUST, é necessário ter o compilador instalado.
// nome do compilador do RUST: rustc
// para rodar o programa, basta digitar no terminal:
// 1) - rustc HelloWordRUST.rs
// 2) - ./HelloWordRUST
// 3) - cargo run
// 4) - cargo build
// 5) - cargo check
// 6) - cargo test
// 7) - cargo doc
// 8) - cargo new nome_do_projeto --bin
// 9) - cargo new nome_do_projeto --lib
// 10) - cargo run --bin
// 11) - cargo run --lib
// 12) - cargo build --bin
// 13) - cargo build --lib
// 14) - rustc nome_do_arquivo.rs
// 15) -  cargo init -> para inicializar o diretório como um projeto RUST

mod helloWorldRUST() {
    let hello:String = "Hello, World Euu criei!";
    println!("{}", hello);
}

mod cool {
    pub fn function() {
        println!("called `cool::function()`");
    }
}

mod my {
    fn function() {
        println!("called `my::function()`");
    }

    mod cool {
        pub fn function() {
            println!("called `my::cool::function()`");
        }
    }

    // mod helloWorldRUST{
    //     println!("Hello, World Euu criei!")
    // }

    pub fn indirect_call() {
        print!("called `my::indirect_call()`, that\n> ");
        self::function();
        function();
        self::cool::function();
        //super::function();
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}
