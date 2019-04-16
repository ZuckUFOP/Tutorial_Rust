//função declarada antes
fn funcao_com_retorno(x: u32) -> String{//para retornar uma string estatica sem perder a referencia
    //funciona com iterators
    for i in (1..x).rev(){
        println!("O numero {} foi interado", i);
    };
    String::from("for")
}
fn main() {
    //Deixe uma das linhas a baixo sem comentar para testar uma utilidade
    let num_immutavel = 17;//Esse valor não pode ser modificado
    //let num_immutavel = 10;//Esse valor não pode ser modificado
    //let num_immutavel = 32;//Esse valor não pode ser modificado

    let testado = if num_immutavel % 3 == 0 {
        let mut contador = 0;//Esse valor pode ser modificado
        let valor_do_loop = loop {
            println!("O contador atual é: {}", contador);
            //comente a proxima linha para torna-lo infinito
            if contador == 10 {break contador+1;}
            contador+=1;
        };
        println!("Valor final do loop é {}", valor_do_loop);
        String::from("loop infinito")
    }else if num_immutavel % 3 == 1 {
        //chamando função declarada depois
        funcao_sem_retorno();
        String::from("while")
    }else{
        //chamando função declarada antes
        funcao_com_retorno(10)
    };
    println!("O loop testado foi {:?}", testado);
}

//função declarada depois
fn funcao_sem_retorno(){
    let mut i = 10;
    println!("Contagem regressiva");
    while i > 0 {
        println!("{}", i);
        i = i-1;
    }
    print!("Boom");
}
