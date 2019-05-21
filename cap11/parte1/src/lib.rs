#[cfg(test)]
mod tests {
	//usando tudo fora do modulo
	use super::*;
	//testes
    #[test]
	fn funciona() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
	
	#[test]
	fn teste_retangulo(){
		let retangulo1 = Retangulo{altura: 12, base: 4,};
		let retangulo2 = Retangulo{altura: 9, base: 3};
		assert!(retangulo1.esta_circunscrito(&retangulo2));
		assert_ne!(true, retangulo2.esta_circunscrito(&retangulo1));
	}

	#[test]
	fn teste_fibonacci(){
		assert_eq!(5, fibonacci(5));
	}
	
	#[test]
	fn conteudo_de_comprimentar(){
		let definir = true;
		if definir{
			let resultado = comprimento("Rodrigo");
			assert!(resultado.contains("Rodrigo"), "O comprimento não possuia o nome definido, ele possuia '{}'", resultado);
		}else{
			let resultado = comprimento("Fabrício");
			assert!(resultado.contains("Rodrigo"), "O comprimento não possuia o nome definido, ele possuia '{}'", resultado);
		}
	}
	
	#[test]
	#[should_panic]
	fn maior_do_que_deveria(){
		Chute::novo(200);
	}
}

//Estruturas
#[derive(Debug)]
pub struct Retangulo{
	altura: u32,
	base: u32,
}
pub struct Chute{
	valor: u32,
}

//implementações
impl Retangulo{
	pub fn esta_circunscrito(&self, outro: &Retangulo)->bool{
		self.base > outro.base && self.altura > outro.altura
	}
}
impl Chute{
	pub fn novo (valor:u32)->Chute{
		if valor < 1 || valor > 100{
			panic!("O valor deveria ser entre 1 e 100, porém o valor é de {}", valor);
		}
		
		Chute{
			valor
		}
	}
}


//funções
//uma simples função fibonacci
pub fn fibonacci(teto: u32)->u32{
	let mut numero = (0,1);
	let mut i = 0;
	
	while i < teto{
		numero = (numero.1, numero.0 + numero.1);
		i = i+1;
	}
	numero.0
}

pub fn comprimento(nome: &str)->String{
	format!("Olá {}",nome)
}