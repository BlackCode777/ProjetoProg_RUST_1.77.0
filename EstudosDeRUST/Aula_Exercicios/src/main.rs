// Verificador de numeros primos

fn main() {
    
    // Estudo e entendimento de referência de memória
    let 

    // Memória static
    // É criada fora na função main()
    // Ela é alocada na memória durante a compilação
    // Ela é alocada na memória durante a execução
    // Ela é uma variavel unsafe
    // Ela é uma variavel global
    // Ela é uma variavel constante
    // Ela é uma variavel imutável
    // Ela é uma variavel que não pode ser alterada
    // Se for alterada, o compilador irá gerar um erro
    // Mas pode ser mutavel de usar unsafe
    // Exemplo
    // static mut NUMBER: i32 = 10;
    // const NUMBER: i32 = 10;
    // Quando devo trabalhar com static?
    // Quando eu preciso de uma variavel global
    // Quando eu preciso de uma variavel constante
    // Quando eu preciso de uma variavel imutável

    // Memória Stack
    // O que são variáveis stack
    // São variáveis que são alocadas na pilha
    // São variáveis que são alocadas na memória durante a execução
    // Quais são os tipo de variáveis stack
    // Exemplos de variáveis stack
    // let number: i32 = 10;
    // let number: f32 = 10.0;
    // let number: char = 'a';
    // let number: bool = true;
    // let number: String = String::from("Rust");
    // let number: Vec<i32> = Vec::new();
    // let number: [i32; 5] = [1, 2, 3, 4, 5];
    // let number: (i32, f32, char) = (10, 10.0, 'a');
    // let number: Option<i32> = Some(10);
    // let number: Result<i32, String> = Ok(10);
    // let number: Box<i32> = Box::new(10);
    // let number: Rc<i32> = Rc::new(10);
    // let number: Arc<i32> = Arc::new(10);
    // let number: Mutex<i32> = Mutex::new(10);
    // let number: Cell<i32> = Cell::new(10);
    // let number: RefCell<i32> = RefCell::new(10);
    // let number: Ref<i32> = Ref::new(10);
    // let number: RefMut<i32> = RefMut::new(10);
    // let number: RwLock<i32> = RwLock::new(10);
    // let number: RwLockReadGuard<i32> = RwLockReadGuard::new(10);
    // let number: RwLockWriteGuard<i32> = RwLockWriteGuard::new(10);
    // let number: Cow<i32> = Cow::new(10);
    // let number: BTreeMap<i32> = BTreeMap::new();
    // let number: HashMap<i32> = HashMap::new();
    // let number: BTreeSet<i32> = BTreeSet::new();
    // let number: HashSet<i32> = HashSet::new();
    // let number: LinkedList<i32> = LinkedList::new();
    // let number: VecDeque<i32> = VecDeque::new();
    // let number: BinaryHeap<i32> = BinaryHeap::new();
    // let number: Rc<i32> = Rc::new(10);

    // Memória Heap
    // O que são variáveis heap
    // São variáveis que são alocadas no heap
    // São variáveis que são alocadas na memória durante a execução
    // Quais são os tipo de variáveis heap
    // Exemplos de variáveis heap
    // let number: String = String::from("Rust");
    // let number: Vec<i32> = Vec::new();
    // let number: Box<i32> = Box::new(10);
    // let number: Rc<i32> = Rc::new(10);
    // let number: Arc<i32> = Arc::new(10);
    // let number: Mutex<i32> = Mutex::new(10);
    // let number: Cell<i32> = Cell::new(10);
    // let number: RefCell<i32> = RefCell::new(10);
    // let number: Ref<i32> = Ref::new(10);
    // let number: RefMut<i32> = RefMut::new(10);
    // let number: RwLock<i32> = RwLock::new(10);
    // let number: RwLockReadGuard<i32> = RwLockReadGuard::new(10);
    // let number: RwLockWriteGuard<i32> = RwLockWriteGuard::new(10);
    // let number: Cow<i32> = Cow::new(10);
    // let number: BTreeMap<i32> = BTreeMap::new();
    // let number: HashMap<i32> = HashMap::new();
    // let number: BTreeSet<i32> = BTreeSet::new();
    // let number: HashSet<i32> = HashSet::new();
    // let number: LinkedList<i32> = LinkedList::new();
    // let number: VecDeque<i32> = VecDeque::new();
    // let number: BinaryHeap<i32> = BinaryHeap::new();
    // let number: Rc<i32> = Rc::new(10);
}

/*

&s1 - com isso se pega a referência da memória onde o dados está guardado
conceito de onwership / reborrowing
Borrowing
Endereçamento de memória
( Variavel do tipo copy no rust )
// Transferencia de propriedade
    let s1: String = String::from("Rust");
    let s2: String = s1.clone();
    println!("O valor de s1 é: {} ref. {:p}", s1, &s1);
    println!("O valor de s2 é: {} ref. {:p}", s2, &s2);

    let mut x: i32 = 10;
    let mut y = &x; // copia dados de x para y - mesmo endereço de memória
    println!("O valor de x é: {}, {:p}", x, &x);
    println!("O valor de y é: {}, {:p}", y, y);



    **********************************************
fn main() {
    /*
    **********************************************

    **********************************************

    **********************************************
        Exercício - Calculadora de média ponderada

        Escreva um programa em Rust que calcule a média ponderada de uma série
        de notas de alunos. O programa deve primeiro perguntar quantas notas serão
        fornecidas. Em seguida, para cada nota, o programa deve solicitar a nota do
        aluno e o peso dessa nota. Ao final, o programa deve exibir a média ponderada
        das notas. A média ponderada é calculada somando-se todas as notas multiplicadas
        por seus respectivos pesos, dividido pela soma dos pesos.

        Critérios:

        1 - Use um loop para solicitar as notas e os pesos.
        2 - Valide as entradas para garantir que notas e pesos sejam números positivos.
        3 - Use um tipo de dado adequado para armazenar as notas e os pesos.
        4 - O programa deve funcionar para qualquer quantidade de notas.
        5 - Exiba a média ponderada com duas casas decimais.

    */

    println!("Calculadora de média ponderada");

    println!("Digite a quantidade de notas: ");
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Falha ao ler a linha");

    let quantidade_notas: i32 = input
        .trim()
        .parse()
        .expect("Por favor, digite um número inteiro");

    let mut notas: Vec<f32> = Vec::new();

    let mut pesos: Vec<f32> = Vec::new();

    for i in 0..quantidade_notas {
        println!("Digite a nota {}: ", i + 1);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        let nota: f32 = input
            .trim()
            .parse()
            .expect("Por favor, digite um número real");

        notas.push(nota);

        println!("Digite o peso da nota {}: ", i + 1);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Falha ao ler a linha");

        let peso: f32 = input
            .trim()
            .parse()
            .expect("Por favor, digite um número real");

        pesos.push(peso);
    }

    let mut soma_notas: f32 = 0.0;
    let mut soma_pesos: f32 = 0.0;

    for i in 0..quantidade_notas {
        soma_notas += notas[i as usize] * pesos[i as usize];
        soma_pesos += pesos[i as usize];
    }

    let media_ponderada: f32 = soma_notas / soma_pesos;

    println!("A média ponderada é: {:.2}", media_ponderada);
}


*/
