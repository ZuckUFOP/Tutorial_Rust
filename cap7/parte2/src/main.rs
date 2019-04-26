mod modulo{
    pub fn funcao_publica_do_modulo(){
        println!("Função publica do modulo acessada")
    }
    fn funcao_privada_do_modulo(){
        println!("Função privada do modulo acessada")
    }
    pub fn funcao_de_acesso(){
        println!("Função de acesso para a função privada");
        funcao_privada_do_modulo()
    }

    pub mod modulo_publico{
        pub fn funcao_publica_do_modulo(){
            println!("Função publica do modulo interno publico acessada")
        }
        fn funcao_privada_do_modulo(){
            println!("Função privada do modulo interno acessada")
        }
        pub fn funcao_de_acesso(){
            println!("Função de acesso para a função privadado modulo interno");
            funcao_privada_do_modulo()
        }
    }

    mod modulo_privado{
        pub fn funcao_publica_do_modulo(){
            println!("Função publica do modulo interno privado")
        }
        fn funcao_privada_do_modulo(){
            println!("Função privada do modulo interno acessada")
        }
        pub fn funcao_de_acesso(){
            println!("Função de acesso para a função privadado modulo interno");
            funcao_privada_do_modulo()
        }
    }

    pub fn funcao_de_acesso_2(){
        println!("Função de acesso para a função publica do modulo privadado interno");
        modulo_privado::funcao_publica_do_modulo()
    }

    pub fn funcao_de_acesso_3(){
        println!("Função de acesso para a função de acesso do modulo privadado interno");
        modulo_privado::funcao_de_acesso()
    }

    //pub fn funcao_de_acesso_4(){
    //    println!("Função de acesso para a função privada do modulo privadado interno");
    //    modulo_privado::funcao_privada_do_modulo()
    //}
}

fn main() {
    modulo::funcao_publica_do_modulo();
    //modulo::funcao_privada_do_modulo();
    modulo::funcao_de_acesso();
    println!("");

    modulo::modulo_publico::funcao_publica_do_modulo();
    //modulo::modulo_publico::funcao_privada_do_modulo();
    modulo::modulo_publico::funcao_de_acesso();
    println!("");

    //modulo::modulo_privado::funcao_publica_do_modulo();
    //modulo::modulo_privado::funcao_privada_do_modulo();
    //modulo::modulo_privado::funcao_de_acesso();
    //println!("");

    modulo::funcao_de_acesso_2();
    //modulo::funcao_de_acesso_4();
    modulo::funcao_de_acesso_3();
    println!("");

    pacote::funcao_externa_publica();
    pacote::funcao_externa_privada();
    pacote::funcao_de_acesso();
    println!("");
}
