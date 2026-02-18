struct Aluno{
    nome: String,
    idade: u8,
    nota: Vec<f32>,
}

impl Aluno{
    fn mostrar(&self){
        println!("Nome: {}", self.nome);
        println!("Idade: {}", self.idade);
        println!("Nota: {:?}", self.media());
        
    }
}

impl Aluno{
    fn metodo_statico(param1: String, param2: u8) -> Aluno{
        println!("Param1: {}, Param2: {}", param1, param2);
        Aluno{
            nome: param1,
            idade: param2,
            nota: vec![],
        }
    }
    fn media_com_ponto_extra(&self, ponto_extra: f32) -> f32{
        let mut media = 0.0;
        for i in &self.nota{
            media += i;
        }
        media = media / self.nota.len() as f32;
        media += ponto_extra;
        media
    }
    fn media(&self) -> f32{
        let mut media = 0.0;
        for i in &self.nota{
            media += i;
        }
        media / self.nota.len() as f32
    }
}

fn main(){
    let aluno = Aluno{
        nome: String::from("João".to_string()),
        idade: 20,
        nota: vec![8.0, 9.0, 10.0]
    };

    aluno.mostrar();

    let extra = 2.0;
    //println!("Media com ponto extra: {}", aluno.media_com_ponto_extra(1.0));
    println!("Media com ponto extra: {}", aluno.media_com_ponto_extra(extra));
    Aluno::metodo_statico("João".to_string(), 20);
}