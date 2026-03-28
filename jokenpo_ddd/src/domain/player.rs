use crate::domain::models::Jogada;
use rand::Rng;
// AQUI: std::io para manipular entrada/saída, e Write para forçar a exibição do print! antes do input.
use std::io::{self, Write};
// AQUI: Importamos a trait FromStr para que o compilador saiba que a string pode chamar o método parse().
use std::str::FromStr;

// AQUI: Contrato de comportamento. Qualquer struct que implementar isso pode jogar.
pub trait Jogador {
    fn escolher_jogada(&self) -> Jogada;
}

pub struct JogadorComputador;

impl Jogador for JogadorComputador {
    fn escolher_jogada(&self) -> Jogada {
        let mut rng = rand::thread_rng();
        // AQUI: O método gen_range sorteia um número de 0 a 2. O pattern matching mapeia para o Domínio.
        match rng.gen_range(0..=2) {
            0 => Jogada::Pedra,
            1 => Jogada::Papel,
            _ => Jogada::Tesoura,
        }
    }
}

pub struct JogadorHumano;

impl Jogador for JogadorHumano {
    fn escolher_jogada(&self) -> Jogada {
        // AQUI: Loop infinito que atua como um 'Retry Pattern'. Só sairemos dele com um input válido.
        loop {
            print!("Digite sua jogada (pedra, papel, tesoura): ");
            // AQUI: Como print! não tem quebra de linha, o buffer do stdout pode reter o texto. O flush() força a exibição na tela.
            io::stdout().flush().unwrap();

            let mut input = String::new();
            
            // AQUI: read_line exige uma referência mutável (&mut) da String. Ele não toma o Ownership da variável, apenas a preenche.
            if io::stdin().read_line(&mut input).is_err() {
                println!("Erro ao ler o terminal. Tente novamente.");
                continue;
            }

            // AQUI: Chamamos explicitamente nossa trait FromStr do Módulo 3.
            // O pattern matching extrai o valor em caso de Ok, ou mostra nosso erro de domínio customizado no terminal em caso de Err.
            match Jogada::from_str(&input) {
                Ok(jogada) => return jogada, // Retorno antecipado, quebra o loop e cumpre o contrato da Trait.
                Err(erro_dominio) => println!("{}", erro_dominio),
            }
        }
    }
}