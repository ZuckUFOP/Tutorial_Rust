use std::fmt::Display;

//estruturas
struct TrechoImportante<'a> {
    parte: &'a str,
}
//implementações
impl <'a>TrechoImportante <'a>{
    fn anuncio_e_retorno(&self, anuncio: &str)->&str{
        println!("Atenção por favor: {}", anuncio);
        self.parte
    }
}

//definido funções
//O codigo a seguir não compila, pois o tempo de vida de str não foi definido
/*fn mais_longo(x:&str, y:&str)->&str{
    if x.len() > y.len(){
        x
    }else{
        y
    }
}*/
//compila, pois o tempo de vida foi especificado
fn mais_longo<'a>(x:&'a str, y:&'a str)->&'a str{//'a é um parametro generico que define o tempo de vida
    if x.len() > y.len(){
        x
    }else{
        y
    }
}
//não funciona pois resultado não vive o suficiente
/*fn mais_longo<'a>(x: &str, y: &str) -> &'a str {
    let resultado = String::from("String realmente grande");
    resultado.as_str()
}*/
fn anuncio_da_maior<'a, T>(x: &'a str, y: &'a str, anuncio: T) -> &'a str
    where T: Display
{
    println!("Anuncio! {}", anuncio);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main(){
    //Tempo de vida de variaveis e funções
    {
        //definindo variaveis
        let frase1 = String::from("Jesus chorou");//String
        let frase2 = "Não matarás";//literal

        //coletando resultado
        let resultado1 = mais_longo(frase1.as_str(), frase2);
        println!("A maior frase é {}", resultado1);

        //coletando resultado de uma segunda maneira
        /*{
            let resultado2 = maiLongo(frase1.as_str(), frase2);
        }*/
        //apresentará um erro pois resultado2 não existe mais
        //println!("A maior frase é {}", resultado2);

        //funciona pois o tempo de vida definido está com resultado2
        let resultado2;
        {
            resultado2 = mais_longo(frase1.as_str(), frase2);
        }
        println!("A maior frase é {}", resultado2);

        //não funciona
        /*let resultado3;
        {
            let frase3 = String::from("Amaras teu próximo como a ti mesmo");
            //aki ocorre um emprestimo
            resultado3 = mais_longo(frase2, frase3.as_str());
        }//sai do escopo
        //usa aqui
        println!("A maior frase é {}", resultado3);*/
    }
    //tempo de vida estruturas e implementações
    {
        //definindo variaveis
        let versiculo = String::from("Finalmente, irmãos, tudo o que for verdadeiro, tudo o que for nobre, tudo o que for correto, tudo o que for puro, tudo o que for amável. Tudo o que for de boa fama, se houver algo de excelente ou digno de louvor, pensem nessas coisas.");
        let parte_a = versiculo.split('.').next().expect("Não foi encontrado nenhum '.'");
        let marcar = TrechoImportante{parte: parte_a};
        let endereco = "Filipenses 4:8";
        let versiculo = String::from("Finalmente, irmãos, tudo o que for verdadeiro, tudo o que for nobre, tudo o que for correto, tudo o que for puro, tudo o que for amável. Tudo o que for de boa fama, se houver algo de excelente ou digno de louvor, pensem nessas coisas.");

        println!("{}", marcar.anuncio_e_retorno(endereco));
        println!("{}",anuncio_da_maior(marcar.parte, endereco, versiculo));
    }
    //Obs.: &'static str= "Qualquer texto"; é um tipo especial de tempo de vida que dura o programa inteiro
}
