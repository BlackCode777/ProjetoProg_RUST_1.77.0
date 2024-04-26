

// Trait
trait Animal {
    fn nome(&self) -> String;
    fn som(&self) -> String;
    fn comer(&self) -> String;
}

struct Dog;
struct Cat;
struct Galinha;

// Struct
struct Cachorro {
    nome: String,
}

impl Animal for Galinha {
    fn nome(&self) -> String {
        "Galinha".to_string()
    }

    fn som(&self) -> String {
        "Cocoricó".to_string()
    }

    fn comer(&self) -> String {
        "Comendo milho".to_string()
    }
}

impl Animal for Cat{
    fn nome(&self) -> String {
        "Gato".to_string()
    }

    fn som(&self) -> String {
        "Miau".to_string()
    }

    fn comer(&self) -> String {
        "Comendo ração".to_string()
    }
}

// Implementação do Trait
impl Animal for Cachorro {
    fn nome(&self) -> String {
        self.nome.clone()
    }

    fn som(&self) -> String {
        "au au".to_string()
    }

    fn comer(&self) -> String {
        "comendo ração".to_string()
    }
}


fn main(){

    let cachorro = Cachorro {
        nome: "Rex".to_string(),
    };

    let gato = Cat;
    let galinha = Galinha;

    println!("{} diz {}", cachorro.nome(), cachorro.som());
    println!("{}", cachorro.comer());

    println!("{} diz {}", gato.nome(), gato.som());
    println!("{}", gato.comer());

    println!("{} diz {}", galinha.nome(), galinha.som());
    println!("{}", galinha.comer());
    
}
