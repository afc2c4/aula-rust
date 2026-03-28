// AQUI: Derivamos traits essenciais. 
// PartialEq permite comparar (Pedra == Pedra).
// Clone e Copy permitem que a struct seja copiada bit a bit barateando a passagem de parâmetros (sem problemas de Ownership para tipos minúsculos).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Jogada {
    Pedra,
    Papel,
    Tesoura,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Resultado {
    Vitoria,
    Derrota,
    Empate,
}

use std::str::FromStr;
use crate::domain::errors::JokenpoError;

// AQUI: Implementamos a lógica como métodos associados à própria enumeração.
impl Jogada {
    // Recebe referências imutáveis (&self, &other) pois não precisamos modificar as jogadas para avaliá-las.
    pub fn avaliar(&self, oponente: &Jogada) -> Resultado {
        match (self, oponente) {
            (Jogada::Pedra, Jogada::Tesoura) |
            (Jogada::Papel, Jogada::Pedra) |
            (Jogada::Tesoura, Jogada::Papel) => Resultado::Vitoria,
            
            (a, b) if a == b => Resultado::Empate,
            _ => Resultado::Derrota,
        }
    }
}

// AQUI: Implementação padronizada do Rust para converter String em tipos.
impl FromStr for Jogada {
    type Err = JokenpoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "pedra" => Ok(Jogada::Pedra),
            "papel" => Ok(Jogada::Papel),
            "tesoura" => Ok(Jogada::Tesoura),
            // AQUI: Retornamos nosso erro customizado encapsulado no Result::Err
            invalido => Err(JokenpoError::EntradaInvalida(invalido.to_string())),
        }
    }
}

// AQUI: Esta configuração diz ao compilador para incluir este módulo APENAS durante 'cargo test'.
#[cfg(test)]
mod tests {
    // Importa tudo do módulo pai (models).
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_vitoria_pedra_sobre_tesoura() {
        // AQUI: Validando a regra central de negócios do nosso domínio.
        assert_eq!(Jogada::Pedra.avaliar(&Jogada::Tesoura), Resultado::Vitoria);
    }

    #[test]
    fn test_parse_sucesso() {
        let jogada = Jogada::from_str("  PaPeL ").unwrap();
        assert_eq!(jogada, Jogada::Papel);
    }

    #[test]
    fn test_parse_falha_erro_customizado() {
        let erro = Jogada::from_str("fogo");
        // AQUI: Assegura que o sistema retorna um Err, e não pânico, garantindo estabilidade do sistema.
        assert!(erro.is_err());
    }
}