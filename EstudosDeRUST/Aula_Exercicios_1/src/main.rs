use std::io;
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;

// criar função de limpar tela
fn limpar_tela() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

// criar função de esperar
fn esperar(tempo: u64) {
    sleep(Duration::from_secs(tempo));
}

// criar função mostrar mensagem
fn mostrar_mensagem(mensagem: &str) {
    limpar_tela();
    println!("{}", mensagem);
    esperar(5);
}

// criar função para capturar notas capturar_notas_aluno
fn capturar_notas_aluno(nome_aluno: &String, notas: &mut Vec<f32>) {
    println!("Digite a nota do(a) {}: (ou 'fim' para concluir)", nome_aluno);
    let mut nota_str = String::new();
    io::stdin().read_line(&mut nota_str).unwrap();

    if nota_str.trim().to_lowercase().contains("fim") {
        limpar_tela();
        return;
    }

    let nota: f32 = match nota_str.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            mostrar_mensagem("Nota inválida, digite novamente ...");
            return capturar_notas_aluno(nome_aluno, notas);
        }
    };
    notas.push(nota);

    mostrar_mensagem("Nota adicionada com sucesso, vamos para a próxima nota ...");

    return capturar_notas_aluno(nome_aluno, notas);
}

// criar um struct
struct Aluno {
    nome: String,
    matricula: String,
    notas: Vec<f32>,
}

// criar cadastro de alunos
fn cadastro_aluno(alunos: &mut Vec<Aluno>) {
    let mut nome = String::new();
    let mut matricula = String::new();

    println!("Digite o nome do aluno: ");
    io::stdin().read_line(&mut nome).unwrap();
    nome = nome.trim().to_string();

    println!("Digite a matricula do aluno: ");
    io::stdin().read_line(&mut matricula).unwrap();
    matricula = matricula.trim().to_string();

    //Pega as notas do aluno
    let mut notas: Vec<f32> = Vec::new();
    capturar_notas_aluno(&nome, &mut notas);
    alunos.push(Aluno { nome: nome, matricula: matricula, notas: notas });
}

fn listar_alunos(alunos: &Vec<Aluno>) {
    limpar_tela();
    if alunos.len() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    for aluno in alunos.iter() {
        println!("{}", "-".repeat(40));
        println!("Nome: {}", aluno.nome);
        println!("Matricula: {}", aluno.matricula);
        println!("Notas: {:?}", aluno.notas);
    }

    println!("\n\nDigite enter para continuar...");
    let mut continuar = String::new();
    io::stdin().read_line(&mut continuar).unwrap();

    limpar_tela();
}

// criar função para alterar os dados do aluno
fn altera_dados_alunos(alunos: &mut Vec<Aluno>) {
    limpar_tela();
    if alunos.len() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    println!("Digite o nome do aluno que deseja alterar: ");
    let mut nome_aluno = String::new();
    io::stdin().read_line(&mut nome_aluno).unwrap();
    nome_aluno = nome_aluno.trim().to_string();

    let mut aluno_encontrado = false;
    for aluno in alunos.iter_mut() {
        if aluno.nome.to_lowercase().contains(&nome_aluno.to_lowercase()) {
            aluno_encontrado = true;

            println!("Digite o novo nome do aluno: ");
            let mut novo_nome = String::new();
            io::stdin().read_line(&mut novo_nome).unwrap();
            aluno.nome = novo_nome.trim().to_string();

            println!("Digite a nova matricula do aluno: ");
            let mut nova_matricula = String::new();
            io::stdin().read_line(&mut nova_matricula).unwrap();
            aluno.matricula = nova_matricula.trim().to_string();

            //Pega as notas do aluno
            let mut notas: Vec<f32> = Vec::new();
            capturar_notas_aluno(&aluno.nome, &mut notas);
            aluno.notas = notas;
            break;
        }
    }

    if !aluno_encontrado {
        mostrar_mensagem("Aluno não encontrado");
    }
}

// criar função para excluir aluno
fn excluir_aluno(alunos: &mut Vec<Aluno>) {
    limpar_tela();
    if alunos.len() == 0 {
        mostrar_mensagem("Nenhum aluno cadastrado");
        return;
    }

    println!("Digite o nome do aluno que deseja excluir: ");
    let mut nome_aluno = String::new();
    io::stdin().read_line(&mut nome_aluno).unwrap();
    nome_aluno = nome_aluno.trim().to_string();

    let mut aluno_encontrado = false;
    for (pos, aluno) in alunos.iter().enumerate() {
        if aluno.nome.to_lowercase().contains(&nome_aluno.to_lowercase()) {
            aluno_encontrado = true;
            alunos.remove(pos);
            mostrar_mensagem("Aluno excluido com sucesso");
            break;
        }
    }

    if !aluno_encontrado {
        mostrar_mensagem("Aluno não encontrado");
    }
}

// Criando um enumerable
enum VersaoIp {
    V4,
    V6
}

struct Servidores{
    nome: String,
    tipo_ip: VersaoIp
}

// fn mostrar_servidores(servidor: Servidores){
//     println!("Nome: {}", servidor.nome);
//     println!("Tipo IP: {:?}", servidor.tipo_ip);
// }

fn main() {



    /*
    === Passo 1: ===
    Sua misão é contruir um menu de sistema console
    O que você deseja fazer ?
    1 - Cadastrar aluno { iniciando cadastro de aluno }
    2 - Alterar aluno  { iniciando alteracao de aluno }
    3 - Excluir aluno { iniciando exclusão de aluno }
    4 - Listar alunos { listando alunos }
    5 - Sair do programa

    === Passo 2: ====
    Agora que vc já sabe criar uma função que vc já sabe como utilizar
    um array ou tupla ou vetor

    faça a implementação da opção 1 e da opção 4
    o que colocar no cadastro de aluno
    nome, matricula, notas{vetor(f32)}

    === Passo 3: ====
    Agora que vc já conhece o struct, implemente os passos 2 e 3

    amanha
    - Corrigir exercicio 3
    - modulos (definição de arquitetura)
    - melhorar o exercicio - separando em modulos
    - enum (aprofundar um pouco mais)
    - hashmap (aprofundar um pouco mais)
    - struct (aprofundar um pouco mais) -> Métodos consigo fazer POO
    */

    let mut alunos: Vec<Aluno> = Vec::new();
    loop {
        println!("\n");
        println!("Digite uma das opções abaixo:");
        println!("-----------------------------");
        println!("1 - Cadastrar aluno");
        println!("2 - Alterar aluno");
        println!("3 - Excluir aluno");
        println!("4 - Listar alunos");
        println!("5 - Sair do programa");
        println!("-----------------------------");

        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Falha ao ler a opção");

        let opcao: u8 = match opcao.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                continue;
            }
        };
        match opcao {
            1 => {
                cadastro_aluno(&mut alunos);
            }
            2 => {
                altera_dados_alunos(&mut alunos);
            }
            3 => {
                excluir_aluno(&mut alunos);
            }
            4 => {
                listar_alunos(&alunos);
            }
            5 => {
                println!("Saindo do programa...");
                break;
            }
            _ => {
                mostrar_mensagem("Opção inválida, tente novamente...!");
            }
        }
        limpar_tela();
    }
}

/*

Revisar os exercicios da aulas 15 / 16/ 17
Estudar tuplas / HashMap / Struct / Enums / Modulos

EXERCÍCIO - Criar um cadastro de alunos. com vetores
    let mut nome: String = String::new();
    let mut matricula: String = String::new();

    captura o nome digitado pelo usuario
    captura a nota digitada pelo usuario
    vetor para receber as notas
*/

// fn esperar() {}
// fn mostrar_mensagem() {}
// fn cadastro_alunos() {}
// fn capturar_notas_aluno() {}
