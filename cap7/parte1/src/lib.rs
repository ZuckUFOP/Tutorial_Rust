#[cfg(test)]
pub fn funcao_externa_publica(){
    println!("Função externa publica acessada")
}
fn funcao_privada_do_modulo(){
    println!("Função externa privada acessada")
}
pub fn funcao_de_acesso(){
    println!("Função de acesso para a função privada");
    funcao_privada_do_modulo()
}
