use std::fmt::Display;

//struturas
struct Par<T> {
    x: T,
    y: T,
}

//implementações de estruturas
impl<T> Par<T> {
    fn novo(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}
impl<T: Display + PartialOrd> Par<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("O maior do par é x = {}", self.x);
        } else {
            println!("O maior do par é y = {}", self.y);
        }
    }
}
//função generica maior
fn maior<T: PartialOrd + Copy>(lista: &[T]) -> T {
    let mut maior = lista[0];
    for &item in lista.iter() {
        if item > maior {
            maior = item;
        }
    }
    maior
}

fn main() {
	//testando função maior
    let lista_de_numeros = vec![34, 50, 25, 100, 65];
    let resultado = maior(&lista_de_numeros);
    println!("O maior número é {}", resultado);
    let lista_de_caracteres = vec!['y', 'm', 'a', 'q'];
    let resultado = maior(&lista_de_caracteres);
    println!("O maior caracter é {}", resultado);
	
	//testando par
	let par_de_numeros = novo(34,56);
	par_de_numeros.cmp_display;
	let par_de_caracteres = novo('a', 'z');
	par_de_caracteres.cmp_display;
}