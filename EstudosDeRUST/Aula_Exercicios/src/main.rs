fn main() {
    /*
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
