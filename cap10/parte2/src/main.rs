//Definindo carateristicas publicas
pub trait Resumo{
	fn resumindo_autor(&self)->String;
	fn resumindo(&self)->String{
		format!("(Leia mais em {:?}...)", self.resumindo_autor())
	}
}
pub trait Mostra{
	fn mostra(&self) ->&String;
}

//Definindo estruturas publicas
pub struct ArtigoNoticia{
	pub local: String,
	pub titulo: String,
	pub autor: String,
	pub conteudo: String,
}
pub struct Tweet{
	pub usuario: String,
	pub conteudo: String,
	pub retweet: bool,
	pub resposta: bool,
}

//implementação das caracteristicas para suas estruturas
impl Resumo for ArtigoNoticia{
	fn resumindo_autor(&self)->String{
		format!("{}, por {} ({})",self.titulo,self.autor,self.local)
	}
}
impl Mostra for ArtigoNoticia{
	fn mostra(&self)->&String{
		&self.conteudo
		
	}
}
impl Resumo for Tweet{
	fn resumindo_autor(&self)->String{
		format!("@{}",self.usuario)
	}
}
impl Mostra for Tweet{
	fn mostra(&self)->&String{
		&self.conteudo
	}
}

//Funções que recebem caracteristicas como parametro
pub fn notificar_autor(item: impl Resumo){
	println!("{:?} publicou algo.", item.resumindo_autor())
}
pub fn notificar<T: Resumo>(item: T){
	println!("Últimas noticias! {:?}", item.resumindo());
}
pub fn informacao<T: Resumo+Mostra>(item: T){
	println!("{:?}",item.resumindo());
	println!("{:?}",item.mostra());
}

fn main() {
	let tweet = Tweet{usuario: String::from("fabricio_pereira"), 
						conteudo: String::from("Sobrevivendo, como sempre é o que importa"), 
						retweet: false, 
						resposta: false,};
	let noticia = ArtigoNoticia{local: String::from("Ouro Preto"), autor:String::from("Jessica Alves"), titulo:String::from("Os Yankees do ICEB são Processados por plágio"), 
					conteudo:String::from("Nessa Terça-feira, 14/05/2019, o clube ubiversitário da Universidade federal de Ouro Preto (UFOP), os Yankees do ICEB, foram processados por plagiar o nome do clube de Baisebol Americano, 'The Yankees', sem a permição do clube em questão...")};
	println!("Uma notificação: {:?}", tweet.resumindo());
	notificar_autor(tweet);
	notificar(noticia);
	let noticia = ArtigoNoticia{local: String::from("Ouro Preto"), autor:String::from("Jessica Alves"), titulo:String::from("Os Yankees do ICEB são Processados por plágio"), 
					conteudo:String::from("Nessa Terça-feira, 14/05/2019, o clube ubiversitário da Universidade federal de Ouro Preto (UFOP), os Yankees do ICEB, foram processados por plagiar o nome do clube de Baisebol Americano, 'The Yankees', sem a permição do clube em questão...")};
	informacao(noticia);
}
