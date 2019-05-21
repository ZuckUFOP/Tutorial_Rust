# Tutorial_Rust
Um pequeno tutorial de Rust em português usando "The Book" como base para os algoritmos

Capitulo 1: Ensina a instalar e compilar "Hello World" (Cargo new "nome_do_projeto", Cargo Build, Cargo Run)

Capitulo 2: Ensina a criar um "Guess Game" (Apenas traduzido)

Capitulo 3: Mostra o que Rust possui em comum com a maioria das linguagens

Capitulo 4: Apresenta o Conceito de Owership(Propriedade) que torna automático o uso de desalocamento de espaço

Capitulo 5: Mostra structs(Estruturas), e methods(metodos)

Capitulo 6: Tipos enumerados

Capitulo 7: Começo da introdução a modulos e libs

	-Parte 1: Cria lib com Cargo new --lib "nome_do_projeto"
	
	-Parte 2: Mostra funcionamento dos modulos

Capitulo 8: Colecionadores: vetores strings e Hash map

Capitulo 9: Tratamento de Erro

Capitulo 10: Programação genérica

	-Parte 1: Declaração de structs e funções que usam tipos genericos
	
	-Parte 2: Usando "trait" para definir caracteristicas comuns à structs
	
	-Parte 3: Limitando uma função generica à alguns tipos
	
	-Parte 4: Aprofundando em Tempo de vida de Variáveis e programando tempo de vidas genéricos

Capitulo 11: Básico sobre testes
	
	-Parte 1: Como definir e escrever testes em rust, usando assert!, assert_ne! e assert_eq!
	
	-Parte 2: Outras especificações de teste
	
		. Cargo test -- --test-threads=num, onde num é um numero natural qualquer: define o numero de threads rodando o teste
		
		. Cargo test -- --nocapture: não imprime na tela os prints do teste
		
		. Cargo test nome_da_função: define testar apenas as funções com este prefixo (Obs.: Se existir apenas uma com esse 
		prefixo apenas ela é rodada)
		
		. #[ignore]: um atributo para ignorar determinada função de teste
		
		. Cargo test -- --ignored: roda teste de todas as funções mesmo se elas tiverem o atributo ignore



