use std::io;
use std::fs::File;
use std::io::ErrorKind;

fn main() {
	//Descomente a parte que quiser testar
	//parte_1();
	//parte_2();
	//parte_3();
	let r = parte_4();
}
//Todos os codigos comentados podem ser descomentados para testar o resultado, porém a maioria delas encerra o programa
fn parte_1(){
	//Erros inrecuperaveis
	//chamer panic
	//panic!("crash and burn");
	//Alocar vetor e selecionar posição invalida sem tratar o erro
	let v = vec![1, 2, 3];
	v[99];
}
//Uma maneira de lidar com erros esperados
fn parte_2(){
	//tentando abrir um arquivo sem tratar erro
	//let a = File::open("Olá.txt");
	//não tratando o possivel erro
	/*
	let a = match a{
		Ok(arquivo) => arquivo,
		Err(erro) => {
			panic!("Aconteceu um problema em abrir o arquivo: {:?}", erro)
		},
	};
	*/
	//Tratando o possivel erro com varios matchs
	/*
	let a = match a{
		Ok(arquivo) => arquivo,
		Err(erro) => match erro.kind(){
			ErrorKind::NotFound => match File::create("Olá.txt"){
				Ok(arquivo_criado) => arquivo_criado,
				Err(erro2) => panic!("Não foi possivel criar o arquivo, pois: {:?}", erro2)
			}
			other_error => panic!("Aconteceu um problema em abrir o arquivo: {:?}", erro),
		},
	};*/
	//tentando abrir o arquivo já tratando o erro sem usar match
	let a = File::open("Olá.txt").unwrap_or_else(|erro| {
		if erro.kind() == ErrorKind::NotFound {
			File::create("Olá.txt").unwrap_or_else(|erro| {
				panic!("Não foi possivel criar o arquivo, pois: {:?}", erro)
			})
		} else {
			panic!("Aconteceu um problema em abrir o arquivo: {:?}", erro);
		}
	});
}

//Usando unwrap e expect
fn parte_3(){
	let a = File::open("Olá.txt").unwrap().expect("Falha em abrir o arquivo");
}

//Propagando erros
fn parte_4()->Result<String, io::Error>{
	//forma comum
	/*
	let a = File::open("Olá.txt");
	let mut a = match a {
		Ok(arquivo) => arquivo,
		Err(erro) => return Err(erro),
	};
	let mut s = String::new();
	match a.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(erro) => Err(erro),
	}*/
	//usando o operador ?
	let mut a = File::open("Olá.txt")?;
	let mut s = String::new();
	f.read_to_string(&mut s)?;
	Ok(s)
}