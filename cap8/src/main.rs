use std::collections::HashMap;//pegando HashMaps

enum Numero{
    Int(i32),
    Float(f64),
    Texto(String),
}

fn main() {
    //Vetores
    {
        //Declarando Vetores
        let vetor1 = vec![1,2,3,4];//vetor imutavel preenchido
        let mut vetor2: Vec<u32> = Vec::new();//vetor mutavel vazio

        //preenchendo o vetor mutavel
        vetor2.push(1);
        vetor2.push(2);
        vetor2.push(3);
        vetor2.push(4);
        vetor2.push(5);

        //lendo valores do vetor
        //lendo diretamente do vetor
        let terceiro: &u32 = &vetor2[2];
        println!("O terceiro valor do vetor vale {}", terceiro);

        //lendo atraves de get
        match vetor1.get(2) {
            Some(numero) => println!("este elemento tem o valor de {}", numero),
            None => println!("Não existe este elemento"),
        }

        //Lendo valores invalidos
        //Se descomentar as linhas abaixo o programa irá compilar, porém durante a execução irá crashar
        //let numero_inexistente: &u32 = &vetor2[7];
        //println!("O numero é {}", numero_inexistente);

        //Com o get
        match vetor2.get(7) {
            Some(numero) => println!("este elemento tem o valor de {}", numero),
            None => println!("Não existe este elemento"),
        }

        //Se descomentarmos as duas linhas abaixo irá apresentar um erro de compilação, porém se só uma delas executará normalmente
        //vetor2.push(6);
        //println!("O terceiro valor do vetor ainda vale {}", terceiro);

        //usando iteradores
        for i in &vetor1{
            println!("i= {}", i);
        }

        println!("");

        for i in &mut vetor2{
            println!("i= {}", i);
            *i +=10;
        }

        println!("");

        for i in &mut vetor2{
            println!("i= {}", i);
        }

        //usando enum
        let vetor1 = [
            Numero::Int(5),
            Numero::Float(5.3),
            Numero::Texto(String::from("Cinco"))
        ];

        println!("");

        for i in &vetor1{
            match i {
                Numero::Int(numero)  => println!("i= {}", numero),
                Numero::Float(numero)=> println!("i= {}", numero),
                Numero::Texto(numero)=> println!("i= {}", numero),
            }
        }
    }
    println!("");
	//Strings
    {
		//Declarações
		let mut string1 = String::new();//String vazia mutavel
		let dados = "Tic";//não é uma string, é um literal
		let string2 = dados.to_string();//Isso é uma string
		let string3 = "Tac".to_string();//isso também pode
		let string4 = String::from("Toe");//isso também pode
		
		//todos funcionam
		println!("os dados são {}", dados);
		println!("s1 tem a seguinte informação {}", string1);
		println!("s2 tem a seguinte informação {}", string2);
		println!("s3 tem a seguinte informação {}", string3);
		println!("s4 tem a seguinte informação {}", string4);
		println!("");
		
		//Acrescentando um literam str
		string1.push_str(dados);//Funciona
		println!("s1 tem a seguinte informação {}", string1);
		
		//Acrescentando um char
		string1.push('-');
		println!("s1 tem a seguinte informação {}", string1);
		println!("");
		
		//Concatenano strings
		let string5 = string2 + &string3;//Obs string2 é perdido
		//println!("s2 tem a seguinte informação {}", string2);//não funciona
		println!("s5 tem a seguinte informação {}", string5);
		
		string1.clear();//esvaziando string
		println!("s1 tem a seguinte informação {}", string1);
		
		string1 = string5 + "-" + &string4;//Obs string5 é perdido
		//println!("s5 tem a seguinte informação {}", string5);//não funciona
		println!("s1 tem a seguinte informação {}", string1);
		println!("");
		
		//arrumando parametros
		string1.clear();//esvaziando
		let string2 = dados.to_string();
		
		//usando format
		string1 = format!("{}-{}-{}",string2,string3,string4);//nada é perdido
		println!("s1 tem a seguinte informação {}", string1);
		println!("s2 tem a seguinte informação {}", string2);
		println!("s3 tem a seguinte informação {}", string3);
		println!("s4 tem a seguinte informação {}", string4);
	}
	println!("");
    //HashMaps
    {
		//Declarando outros itens importantes para o exemplo
		let vetor1 = vec![15, 12, 9, 8];
		let vetor2 = vec!['a','b', 'c', 'd'];
		let texto = "Eu já alguma vez contei a definição de insanidade? Insanidade repetir e repetir a mesma coisa esperando que ela dê um resultado diferente. Isso é loucura. Então... eu já te contei a definição de insanidade?";
		
		//Declarando hashmaps
		let mut hash1 = HashMap::new();
		let hash2: HashMap<_,_> = vetor1.iter().zip(vetor2.iter()).collect();
		let mut hash3 = HashMap::new();
		
		//inserindo valores
		hash1.insert('f',4);
		hash1.insert('g',0);
		hash1.insert('h',12);
		hash1.insert('i',20);
		
		//recuperando valores
		let dado = hash1.get(&'a');
		println!("O valor da chave a é: {:?}", dado);
		
		
		//atualizando valores
		hash1.insert('a',18);
		let dado = hash1.get(&'a');
		println!("O valor da chave a é: {:?}", dado);
		
		let dado = hash1.entry('b').or_insert(3);
		println!("O valor da chave b é: {:?}", dado);
		hash1.entry('z').or_insert(3);
		println!("");
		
		//imprimindo hash1
		println!("{:?}", hash1);
		println!("");
		
		//looping no hash
		for (chave,valor) in &hash2{
			println!("{}: {}",chave, valor);
		}
		println!("");
		
		//um contador de palavras separados por espaço
		for palavra in texto.split_whitespace(){
			let contador = hash3.entry(palavra).or_insert(0);
			*contador += 1;
		}
		println!("{:?}", hash3);
	}
}
