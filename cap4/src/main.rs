fn main() {
    println!("Propriedade com valores sem ser String");
    let i=4;//declarando uma variavel quaquer
    println!("O valor de i é: {:?}", i);
    let i = i+5;//sobrescreve i
    println!("O valor de i é: {:?}", i);
    {
        let i = i -4;//aqui usamos o valor de i anterior para criar um novo i
        println!("O valor de i é: {:?}", i);
    }//i o interno está agora fora de escopo e é descartado, porém o i anterior é mantido
    //como o i n foi realmente sobreescrito como antes ele é resgatado
    println!("O valor de final de i é: {:?}?", i);

    //agora com strings
    println!("Propriedade com valores sendo Strings");
    let mut s1 = String::from("Um Lannister");
    println!("O valor de s1 é: {:?}", s1);
    //altera valor de s1
    s1.push_str(" sempre paga seu debitos.");
    println!("Novo valor de s1 é: {:?}", s1);

    //"clonagem" de string
    let s2 = s1;
    //se a linha a seguir for para o código irá resultar em erro pois s1 saiu de escopo na linha anterior
    //println!("O valor de s1 é: {:?}", s1);
    println!("O valor de s2 é: {:?}", s2);

    //Agora clonagem para valer
    let s1 = s2.clone();
    println!("O valor de s1 é: {:?}", s1);
    println!("O valor de s2 é: {:?}", s2);

    //com funções
    //dar valor de string
    let s2 = dar_valor();
    println!("O valor de s2 é: {:?}", s2);

    //sem problemas
    inteiro_eh_copia(i);
    println!("Fora da função, o valor de i é: {:?}", i);
    //com problemas
    string_nao_eh(s2);
    //irá gerar erro pois s2 está fora do escopo
    //println!("Fora da função, o valor de s2 é: {:?}", s2);

    //corrigindo o problema mencionado anteriormente
    let s1 = pega_e_devolve(s1);
    println!("Fora da função, o valor de s1 é: {:?}", s1);

    //Outra solução
    let _len = taman_do(&s1);
    println!("Fora da função, o valor de s1 é: {:?} e seu tamanho é: {:?}", s1, _len);

    //retorno de multiplos valores
    let (s2, _len) = taman_do_a(s1);//se não entendeu a referrencia T_T & s1 sai do escopo
    println!("O tamn de '{:?}' é {:?}", s2, _len);

    //modificando strings
    let mut s2 = s2;
    modifica_string(&mut s2);
    println!("Fora da função, o valor de s2 é: {:?}", s2);

    //Outras notas importantes
    {
        let r1 = &s2;
        println!("O valor de r1 é: {:?}", r1);
        //não pode ter dois emprestimos mutaveis ao mesmo tempo (descomente todas as linhas abaixo para ver o erro)
        //let mut r2 = &s2
        //println!("Fora da função, o valor de r1 é: {:?}", r1);
        //println!("Fora da função, o valor de r1 é: {:?}", r2);
    }//r1 sai do escopo então n existe fora das chaves
    //println!("Fora da função, o valor de r1 é: {:?}", r1);

    //descomente todas as linhas abaixo para visualizar o erro
    let r2 = &s2;
    let r3 = &s2;
    //não se pode ter um emprestimo mutavel depois de imutaveis
    //let mut r1 = &s2
    println!("Fora da função, o valor de r3 e r2 são: '{:?}' e '{:?}' respectativamente", r2,r3);
    //println!("Fora da função, o valor de r1 é: {:?}", r1);

    //descomentar aqui e a funçao retorna_Referencia para ver erro
    //let s1 = retorna_Referencia();
    //println!("O valor de s1 é: {:?}", s1);

    //Trabalhando agora com fatias
    println!("Fatias");
    let s1 = String::from("Winter is coming");

    //pegando o começo
    let p1 = &s1[0..6];//Esses dois são quiva_lentes
    //let p1 = &s1[..6];//Esses dois são quiva_lentes

    //pegando o meio
    let p2 = &s1[7..9];

    //pegando o fim
    let p3 = &s1[10..16];//Esses dois são equiva_lentes
    //let p3 = &s1[10..];

    println!("No final os tres pedaços formam a frase(sem os espaços): {:?} {:?} {:?}", p1,p2,p3);

    //strings literais são considerados fatias então funcionando com eles também
    //let s1 = "Winter is coming";

    //pegando o começo
    //let p1 = &s1[0..5];//Esses dois são quiva_lentes
    //let p1 = &s1[..5];//Esses dois são quiva_lentes

    //pegando o meio
    //let p2 = &s1[7..8];

    //pegando o fim
    //let p3 = &s1[10..15];//Esses dois são equiva_lentes
    //let p3 = &s1[10..];

    //println!("No final os tres pedaços formam a frase(sem os espaços): {:?} {:?} {:?}", p1,p2,p3);

    //Fatias também funcionam com vetores
    let vet = [6,5,4,3,2,1];
    let p1 = &vet[..4];
    let p2 = &vet[4..];

    for j in p2{
        println!("{:?} é um valor que está na parte 2", j);
    }
    for j in p1 {
        println!("{:?} é um valor que está na parte 1", j);
    }
}

//Código das Funções
fn dar_valor()->String{
    let st = String::from("Um Lannister");
    st
}
fn inteiro_eh_copia(x:u32){
    println!("Dentro da função o valor de i é: {:?}", x);
}
fn string_nao_eh(st:String){
    println!("Dentro da função, o valor de s2 é: {:?}", st);
}
fn pega_e_devolve(st:String)->String{
    println!("Dentro da função, o valor de s1 é: {:?}", st);
    st
}
fn taman_do(a:&String)->usize{
    a.len()
}
fn taman_do_a(a:String)->(String,usize){
    let taman = a.len();
    (a,taman)
}
fn modifica_string(st:&mut String){
    st.push_str(" Tyrion Lannister");
}
//Função abaixo n funciona descomente para testar, usar funçao dar_valor() para funcionar como essa
//fn retorna_Referencia()->&String{
//  let s = String::from("Hello");
//  &s
//}
