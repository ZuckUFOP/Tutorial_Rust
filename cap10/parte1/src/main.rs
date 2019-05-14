//strutura generica
struct Ponto2d<T,U>{
	x: T,
	y: U,
}
//enum generico
enum Marcacao<T>{
	Vermelho(T),
	Normal(T),
}
//implementação generica
impl<T,U> Ponto2d<T,U>{
	fn x(&self)->&T{
		&self.x
	}
	fn y(&self)->&U{
		&self.y
	}
}

fn main() {
	//declarando pontos
	let ponto_a = Ponto2d{x:1  ,y:5  };
	let ponto_b = Ponto2d{x:1.5,y:5.5};
	let ponto_c = Ponto2d{x:1.5,y:5  };
	
	//imprimindo valores
	println!("o ponto 'A' tem para x o valor {} e y o valor {}", ponto_a.x(), ponto_a.y());
	println!("o ponto 'B' tem para x o valor {} e y o valor {}", ponto_b.x(), ponto_b.y());
	println!("o ponto 'C' tem para x o valor {} e y o valor {}", ponto_c.x(), ponto_c.y());
	
	//declarando valores
	//let vetor_a = vec![Marcacao::Vermelho(ponto_a),Marcacao::Vermelho(ponto_b),Marcacao::Normal(ponto_c)];//não funciona pois são tipos diferentes
	let vetor_b = vec![Marcacao::Vermelho(2),Marcacao::Normal(4),Marcacao::Normal(9)];
	
	//imprimindo valores
	/*for ponto in vetor_a{
		match ponto{
			Marcacao::Vermelho(_)=>print!("Esse ponto é vermelho"),
			Marcacao::Normal(_)=>print!("Esse ponto é normal"),
		}
	}*/
	for valor in vetor_b{
		match valor{
			Marcacao::Vermelho(numero)=>println!("O numero {} é vermelho", numero),
			Marcacao::Normal(numero)=>println!("O numero {} é normal", numero),
		}
	}
}
