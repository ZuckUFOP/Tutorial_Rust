//definindo estruturas
//Pessoa na academia
struct Pessoa{
    nome: String,
    energia: u16,
    fisico: Fisico
}
//Criando o tipo Fisico para a pessoa
struct Fisico (u16, u16, u16);
//Criando o tipo Trino a ser feito pela pessoa
struct Treino (u16, u16, u16, u16);

//Metodos de Pessoa
impl Pessoa {
    //Construtor
    fn cria(n:String, e: u16, torso:u16, braco:u16, perna:u16)->Pessoa{
        Pessoa{nome: n, energia: e, fisico:Fisico(torso,braco,perna)}
    }
    //O treino em si
    fn treina(&self, tipo:&Treino)->Pessoa{
        //self serve para acessar os itens interiores da struct, toda vez que tais valores são necessários
        //é presciso passar &self como referencia
        if self.energia > tipo.3{
            //acessando as posiçoes das tuplas detro de pessoa e Treino
            Pessoa {nome: self.nome.clone(),
                energia: self.energia - tipo.3,
                fisico:Fisico(self.fisico.0 + tipo.0*(100 - self.fisico.0)/100,
                    self.fisico.1 + tipo.1*(100 - self.fisico.1)/100,
                    self.fisico.2 + tipo.2*(100 - self.fisico.2)/100)}
        }else{
            //Se não for possivel gastar toda a energia gaste a porcentagem possivel
            let difereca = (self.energia/tipo.3)*100;
            Pessoa {nome:self.nome.clone(),
                energia:0,
                fisico:Fisico(self.fisico.0 + (tipo.0*(100 - self.fisico.0)/100)*difereca/100,
                    self.fisico.1 + (tipo.1*(100 - self.fisico.1)/100)*difereca/100,
                    self.fisico.2 + (tipo.2*(100 - self.fisico.2)/100)*difereca/100)}
        }
    }
    //Imprime os resultados do treino no dia
    fn resultado(&self){
        println!("Nome do atleta: {:?}", self.nome);
        println!("Energia restante no dia: {:?}", self.energia);
        println!("Fisico atual:");
        println!("Torso: {:?}", self.fisico.0);
        println!("Braço: {:?}", self.fisico.1);
        println!("Perna: {:?}", self.fisico.2);
    }
}

fn main(){
    //Cria uma instancia de Pessoa
    let mut _p1 = Pessoa::cria(String::from("Jojo"), 100, 50, 15, 25);
    //Cria a sequencia de treinos
    let vt = [Treino(15,20,20,20),Treino(5,30,45,35),Treino(5,30,30,20),Treino(10,20,25,20), Treino(10,10,10,10)];
    //itera todas as posiçoes
    for i in vt.iter() {
        _p1 = _p1.treina(i);
    }
    //imprime resultado
    _p1.resultado();
}
