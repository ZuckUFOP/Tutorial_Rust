//para usar leitura de painel
use std::io;

//declaração de tipos enumerados
enum Moeda{
    Cinco,
    Dez,
    VinteCinco,
    Cinquenta,
    Real,
}
enum Nota{
    Dois,
    Cinco,
    Dez,
    Vinte,
}

//funções usando match para devolver um falor e usando o tipo enumerado da biblioteca basica Option(T)
fn valor_da_moeda(moeda:Option<Moeda>) -> f64{
    match moeda {
        Some(Moeda::Cinco) => 0.05,
        Some(Moeda::Dez) => 0.1,
        Some(Moeda::VinteCinco) => 0.25,
        Some(Moeda::Cinquenta) => 0.5,
        Some(Moeda::Real) => 1.0,
        None => 0.0,
    }
}
fn valor_da_nota(nota:Option<Nota>) -> f64{
    match nota {
        Some(Nota::Dois) => 2.0,
        Some(Nota::Cinco) => 5.0,
        Some(Nota::Dez) => 10.0,
        Some(Nota::Vinte) => 20.0,
        None => 0.0,
    }
}

fn main() {
    //acumulando dinheiro
    let mut acumulado = 0.0;
    //os preços dos cardapios
    let cardapio = [1.5, 3.5, 2.0, 2.5, 4.5, 2.75];
    //recebendo o dinheiro
    loop{
        //cardapio
        println!("Seu credito é {:?}", acumulado);
        println!("O cardapio é: ");
        println!("1-Cafe           {:?}  4-Achocolatado        {:?}", cardapio[0], cardapio[3]);
        println!("2-Leite          {:?}  5-Leite com Chocolate {:?}", cardapio[1],cardapio[4]);
        println!("3-Leite com Café {:?}  6-Descafeinado        {:?}", cardapio[2],cardapio[5]);
        println!("Por favor coloque o dinheiro");
        println!("(Se for colocar uma moeda escreva M e o valor. Exemplo M25 para uma moeda de 25 centavos)");
        println!("(Se for colocar uma nota escreva N e o valor. Exemplo V10 para uma nota de 10 reais)");
        println!("(Obs.: Nota maxima é de 20, não aceita moeda de 1 centavo)");
        println!("(Se a primeira letra escrita não for 'N' ou 'M' acabará a cessão de incerssão dinheiro)");
        //Lendo a entrada
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Falha ao ler a linha");
        //separando o tipo do dinheiro e o valor
        let tipo = &entrada[..1];
        let qnt = &entrada[1..];
        //selecionando as moedas e notas
        acumulado += match tipo.trim() {
            "M" => {
                //tratanto moedas
                let dinheiro = match qnt.trim(){
                    "5" => Some(Moeda::Cinco),
                    "10"=> Some(Moeda::Dez),
                    "25"=> Some(Moeda::VinteCinco),
                    "50"=> Some(Moeda::Cinquenta),
                    "1" => Some(Moeda::Real),
                     _  => None,
                };
                valor_da_moeda(dinheiro)
            },
            "N" =>{
                //tratando notas
                let dinheiro = match qnt.trim() {
                    "2" => Some(Nota::Dois),
                    "5" => Some(Nota::Cinco),
                    "10"=> Some(Nota::Dez),
                    "20"=> Some(Nota::Vinte),
                     _  => None,
                };
                valor_da_nota(dinheiro)
            },
             _  => 0.0,
        };
        //saindo do loop
        if (tipo.trim() != "M") && (tipo.trim() != "N") {break;}
    }
    //recebendo os pedidos
    loop{
        //cardapio
        println!("Seu credito é {:?}", acumulado);
        println!("O cardapio é: ");
        println!("1-Cafe           {:?}  4-Achocolatado        {:?}", cardapio[0], cardapio[3]);
        println!("2-Leite          {:?}  5-Leite com Chocolate {:?}", cardapio[1],cardapio[4]);
        println!("3-Leite com Café {:?}  6-Descafeinado        {:?}", cardapio[2],cardapio[5]);
        println!("Digite o número do item que você deseja");
        println!("(Se digitar o número que não está escrito terminará a sessão)");
        //lendo valor
        let mut entrada = String::new();
        io::stdin().read_line(&mut entrada).expect("Falha ao ler a linha");
        let entrada: u32 = match  entrada.trim().parse(){
            Ok(numero) => numero,
            Err(_) => continue,
        };
        //descontando valor usando match
        let preco = match entrada {
            1 => cardapio[0],
            2 => cardapio[1],
            3 => cardapio[2],
            4 => cardapio[3],
            5 => cardapio[4],
            6 => cardapio[5],
            _ => 0.0,//aceita todos os valores não escritos acima
        };
        //tratando saida
        if acumulado < preco{println!("Dinheiro insuficiente");}
        else {acumulado -= preco;}
        if entrada > 6{break;}
        else if acumulado == 0.0{break;}
    }
    //troco
    while acumulado > 0.05{
        //selecionando o dinheiro
        println!("acumulado: {:?}", acumulado);
        if acumulado > 20.0{
            println!("Entregando uma nota de 20 Reais");
            acumulado-=20.0;
        }else if acumulado > 10.0{
            println!("Entregando uma nota de 10 Reais");
            acumulado-=10.0;
        }else if acumulado > 5.0{
            println!("Entregando uma nota de 5 Reais");
            acumulado-=5.0;
        }else if acumulado > 2.0{
            println!("Entregando uma nota de 2 Reais");
            acumulado-=2.0;
        }else if acumulado > 1.0{
            println!("Entregando uma moeda de 1 Real");
            acumulado-=1.0;
        }else if acumulado > 0.5{
            println!("Entregando uma moeda de 50 centavos");
            acumulado-=0.5;
        }else if acumulado > 0.25{
            println!("Entregando uma moeda de 25 centavos");
            acumulado-=0.25;
        }else if acumulado > 0.1{
            println!("Entregando uma moeda de 10 centavos");
            acumulado-=0.1;
        }
    }
    if acumulado > 0.05{
        println!("Entregando uma moeda de 5 centavos");
        acumulado-=0.05;
    }
}
