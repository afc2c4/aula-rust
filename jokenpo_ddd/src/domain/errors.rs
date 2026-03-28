use thiserror::Error;

// AQUI: Usamos a macro do thiserror para automatizar a implementação de std::error::Error e Display.
#[derive(Error, Debug)]
pub enum JokenpoError {
    #[error("Jogada inválida: {0}. Escolha pedra, papel ou tesoura.")]
    EntradaInvalida(String),
}