#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn passa() {
        assert_eq!(10, imprime_e_retorna(10));
    }
	
	#[test]
	fn passa_msm(){
		assert_eq!(100,imprime_e_retorna(100));
	}
	
	#[test]
	#[ignore]
	fn nao(){
		assert_eq!(5, imprime_e_retorna(10));
	}
}

fn imprime_e_retorna(numero: u32)->u32{
	println!("O número é {}", numero);
	numero
}