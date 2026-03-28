mod domain;

use domain::models::Resultado;
// AQUI: Importamos a nova struct JogadorHumano junto com as dependências anteriores.
use domain::player::{Jogador, JogadorComputador, JogadorHumano};
use std::io::{self, Write};

fn main() {
    println!("=== BEM-VINDO AO JOKENPO EM RUST ===");
    
    // AQUI: Instanciamos nossas dependências concretas. Zero custo de memória extra.
    let humano = JogadorHumano;
    let computador = JogadorComputador;

    // AQUI: O Game Loop principal.
    loop {
        println!("\n--- Nova Rodada ---");
        
        // AQUI: O Polimorfismo brilha! Chamamos exatamente o mesmo método para entidades completamente diferentes.
        let jogada_humano = humano.escolher_jogada();
        let jogada_pc = computador.escolher_jogada();

        println!("Você escolheu: {:?}", jogada_humano);
        println!("O computador escolheu: {:?}", jogada_pc);

        // AQUI: Invocamos nossa regra de negócio do Módulo 2 e 3.
        let resultado = jogada_humano.avaliar(&jogada_pc);
        
        match resultado {
            Resultado::Vitoria => println!("🎉 Você VENCEU a rodada!"),
            Resultado::Derrota => println!("💀 Você PERDEU a rodada!"),
            Resultado::Empate => println!("🤝 Deu EMPATE!"),
        }

        print!("\nDeseja jogar novamente? (s/n): ");
        io::stdout().flush().unwrap();
        
        let mut continuar = String::new();
        io::stdin().read_line(&mut continuar).unwrap();
        
        // Avaliação simples da intenção do usuário para continuar ou quebrar o Game Loop.
        if continuar.trim().to_lowercase() != "s" {
            println!("Saindo do jogo... Obrigado por jogar!");
            break;
        }
    }
}