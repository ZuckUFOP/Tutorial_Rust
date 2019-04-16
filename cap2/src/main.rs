use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Advinhe o Número");
    //gera um numero aleatório
    let numero_secreto = rand::thread_rng().gen_range(1,101);
    //looping infinito
    loop{
        println!("Escreva seu chute");
        //declarando uma string
        let mut chute = String::new();
        //leitura de console
        io::stdin().read_line(&mut chute).expect("Falha ao ler a linha");
        //tenta transformar em um inteiro de 32 bits
        let chute: u32= match  chute.trim().parse(){
            Ok(numero) => numero,
            Err(_) => continue
        };
        println!("Você chutou: {}", chute);
        //operador match que compara condições
        match chute.cmp(&numero_secreto) {
            Ordering::Less => println!("Muito Pequeno"),
            Ordering::Greater => println!("Muito Grande"),
            Ordering::Equal => {
                println!("Você acertou");
                break;
            }
        }
    }
}
