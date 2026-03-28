Neste artigo prático, vamos construir um motor de Jokenpo (Pedra, Papel e Tesoura). Vamos aplicar conceitos de Domain-Driven Design (DDD), desacoplamento com Traits, paralelismo seguro e tratamento robusto de erros.

Prepare seu terminal. Vamos codificar bloco por bloco, e eu explicarei o raciocínio de engenharia por trás de cada decisão.

---

## Módulo 1: Fundamentos, Configuração e Cargo

### Subtópico: Inicialização do Workspace e Gestão de Dependências

**Raciocínio e Problema Arquitetural:**

Antes de escrevermos qualquer lógica, precisamos definir o alicerce do nosso projeto. O `cargo` não é apenas um compilador, é um gerenciador de pacotes e workspace completo. Para um jogo de Jokenpo, precisaremos de entropia (aleatoriedade para a jogada do computador) e de uma forma elegante de propagar erros, sem poluir nossa lógica de negócios com blocos de texto puro. Em sistemas corporativos, criar tipos de erro fortemente tipados é crucial para observabilidade e debug. Para isso, usaremos as crates `rand` (geração aleatória) e `thiserror` (macros para derivar a trait `Error` nativa do Rust de forma ergonômica).

**Mapeamento de Arquivos e Módulos:**

1. No seu terminal, rode: `cargo new jokenpo_ddd && cd jokenpo_ddd`
2. Abra o arquivo `Cargo.toml`.

**Código Incremental Estrito:**

Adicione as seguintes linhas no final do seu `Cargo.toml`:

```toml
[dependencies]
# AQUI: rand versão 0.8 para geração de números aleatórios na escolha da máquina.
rand = "0.8.5"
# AQUI: thiserror versão 1.0 para criarmos erros de domínio com a macro #[derive(Error)].
thiserror = "1.0.50"
```

**Como testar e Resultado Esperado:**

No terminal, execute o comando:

`cargo check`

*Resultado Esperado:* O Cargo fará o download das crates `rand` e `thiserror` (e suas dependências) e compilará o esqueleto inicial. Você deve ver `Finished dev [unoptimized + debuginfo] target(s)`. O projeto compila.

---

## Módulo 2: Modelagem de Domínio e Estruturas de Dados

### Subtópico: Tipos Algébricos de Dados (Enums) para o Domínio Ubíquo

**Raciocínio e Problema Arquitetural:**

Em muitas linguagens, desenvolvedores representam escolhas finitas usando strings (`"pedra"`, `"papel"`) ou inteiros (`0`, `1`, `2`). Isso é um anti-pattern conhecido como *Primitive Obsession*. Se usarmos uma string, o compilador não pode nos impedir de passar `"fogo"` ou `"lagarto"`, causando erros em tempo de execução.

Em Rust, usamos *Enums* (Tipos Algébricos de Dados). Eles garantem tipagem estrita e exaustividade em tempo de compilação. Se o nosso domínio define que existem apenas três jogadas possíveis, o sistema de tipos deve refletir isso matematicamente. O mesmo vale para o resultado da partida.

**Mapeamento de Arquivos e Módulos:**

1. Crie uma pasta chamada `domain` dentro de `src/`: `mkdir src/domain`
2. Crie o arquivo `src/domain/models.rs`.
3. Crie o arquivo `src/domain/mod.rs` (para expor os módulos).

**Código Incremental Estrito:**

Adicione o seguinte código no arquivo recém-criado `src/domain/models.rs`:

```rust
// RACIOCÍNIO HUMANO (A Abstração): 
// Entenda o seguinte: usar string de texto pura pra representar uma escolha finita é coisa de amador. 
// Em engenharia de verdade e modelagem de domínios (Domain-Driven Design), a gente restringe 
// as opções usando 'enums' (Tipos Algébricos de Dados). Isso cria uma fronteira rígida. 
// Não tem como o sistema aceitar um estado inválido se o domínio matematicamente só conhece três regras.
//
// RACIOCÍNIO DO COMPUTADOR (A Escovação de Bits): 
// Não tem mágica. O compilador (rustc) é burro, ele precisa que você diga tudo. 
// Essa macro #[derive(...)] é só metaprogramação para geração automática de código (Boilerplate) em tempo de compilação.
// - Debug: O compilador injeta o código necessário para serializar essa estrutura em texto para o 'stdout'.
// - PartialEq/Eq: Gera as instruções de CPU para fazer a comparação direta dos valores na memória (Bitwise Comparison).
// - Clone/Copy: Aqui é o trade-off de performance. Esse enum é minúsculo, cabe em um único byte. 
//   Avisamos ao compilador que é infinitamente mais barato para a CPU simplesmente duplicar esse byte 
//   direto na memória rápida (Stack) do que rastrear ponteiros e invocar as regras de posse de memória (Borrow Checker).
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Jogada {
    Pedra,
    Papel,
    Tesoura,
}

// RACIOCÍNIO HUMANO (A Abstração): 
// Representa o desfecho determinístico da nossa rodada, sem margem para ambiguidades.
//
// RACIOCÍNIO DO COMPUTADOR (A Escovação de Bits): 
// Preste atenção: a gente não usou 'Clone' ou 'Copy' aqui. Por quê? Porque não existe almoço grátis. 
// O 'Resultado' é gerado, lido num padrão condicional (Pattern Matching) para imprimir no terminal, 
// e imediatamente jogado fora. Ele não transita entre funções. Não há necessidade de instruir o 
// compilador a habilitar semântica de cópia se o ciclo de vida (Lifetime) da variável acaba na mesma instrução (Dropped).
#[derive(Debug, PartialEq, Eq)]
pub enum Resultado {
    Vitoria,
    Derrota,
    Empate,
}
```

Agora, registre o módulo no arquivo `src/domain/mod.rs`:

```rust
// RACIOCÍNIO HUMANO (A Abstração): 
// Iniciante de framework web acha que é só jogar arquivo numa pasta e a linguagem 
// tem a obrigação de adivinhar e carregar tudo magicamente. Em engenharia de software 
// de verdade, você define fronteiras estritas (Encapsulamento / Bounded Contexts). 
// A pasta 'domain' não é só uma pasta, é um módulo. O arquivo 'mod.rs' atua como o 
// porteiro. Ao declarar 'pub mod models', você define um contrato estrito dizendo: 
// "A API (Application Programming Interface) de modelos está exposta, podem usar". 
// Sem código espaguete.
//
// RACIOCÍNIO DO COMPUTADOR (A Escovação de Bits): 
// Não tem mágica de auto-discovery. O compilador (rustc) é propositalmente cego 
// para o sistema de arquivos (File System) do seu SO. Essa linha é uma diretiva 
// estrita (Compiler Directive). Ela manda a CPU ir no disco, ler o arquivo 'models.rs', 
// fazer o parse e gerar a árvore de compilação (AST - Abstract Syntax Tree). 
// O prefixo 'pub' (Public) é apenas uma flag: ele instrui o compilador a alterar 
// sua tabela de símbolos (Symbol Table), injetando as referências dessas structs 
// no escopo global (Namespace) para o Linker conseguir conectar com a main.rs. 
// Sem isso, é erro de escopo privado na hora da compilação.
pub mod models;
```

E no topo do seu `src/main.rs`, adicione:

```rust
// RACIOCÍNIO HUMANO (A Abstração): 
// Iniciante acostumado com framework web mágico acha que é só criar pasta e jogar arquivo 
// que o código se auto-descobre. Em engenharia de software de verdade, a gente é explícito. 
// A declaração 'mod' é você desenhando a arquitetura na mão: "Olha, existe uma fronteira 
// de regra de negócio (Bounded Context) chamada 'domain', conecte ela à aplicação principal".
//
// RACIOCÍNIO DO COMPUTADOR (A Escovação de Bits): 
// Não tem mágica de auto-load. O arquivo 'src/main.rs' é o ponto zero do compilador 
// (Crate Root). Quando o compilador (rustc) esbarra no 'mod domain;', ele pausa o parsing 
// atual, faz I/O no disco buscando 'domain.rs' ou 'domain/mod.rs', converte esse texto 
// em uma Árvore Sintática Abstrata (AST) em memória e instrui o vinculador (Linker) a 
// juntar o código de máquina gerado no binário final. Sem essa instrução estrita, a 
// pasta é ignorada e descartada (Dead Code Elimination).
mod domain;

// RACIOCÍNIO HUMANO (A Abstração): 
// Todo software executável precisa de uma porta de entrada inquestionável. A 'main' é 
// de onde o fluxo de execução (Control Flow) parte. O 'println!' é só um feedback 
// visual básico pra gente ter certeza de que o processo acordou e não morreu no meio do caminho.
//
// RACIOCÍNIO DO COMPUTADOR (A Escovação de Bits): 
// O Linker mapeia a 'fn main()' para o ponto de entrada (Entry Point) oficial que o 
// Sistema Operacional (OS) exige para alocar o processo. Quando o OS joga seu binário 
// na memória RAM, o ponteiro de instrução da CPU (Instruction Pointer) é cravado 
// exatamente no primeiro byte dessa função. E presta atenção: aquele '!' no 'println!' 
// significa que não é uma função, é uma macro (Metaprogramming). Em tempo de compilação, 
// o Rust expande isso gerando um código gigante de baixo nível para travar o buffer 
// de saída do terminal (Standard Output) com segurança, garantindo que não vai dar 
// falha de segmentação (Segmentation Fault).
fn main() {
    println!("Jokenpo inicializado!");
}
```

**Como testar e Resultado Esperado:**

No terminal, execute:

`cargo check`

*Resultado Esperado:* Compilação bem-sucedida. O compilador deve passar sem erros, confirmando que os módulos foram linkados corretamente. Pode haver warnings de "código não utilizado" (dead code), o que é perfeitamente normal nesta etapa.

---

## Módulo 3: Lógica de Negócios e Tratamento de Erros

### Subtópico: Pattern Matching Exaustivo e Erros Customizados

**Raciocínio e Problema Arquitetural:**

Precisamos resolver dois problemas:

1. Como converter uma entrada de texto do usuário (ex: "pedra") para nossa enumeração de domínio com segurança?
2. Como calcular quem ganha?

Rust brilha com a expressão `match`. Diferente de um `switch/case` clássico, o `match` em Rust é *exaustivo*. Se adicionarmos `Spock` ao nosso enum amanhã, o código nem compila até tratarmos a nova regra.

Para tratar a conversão de texto, criaremos um Erro Customizado e usaremos a trait nativa `std::str::FromStr`. O retorno será um `Result<Jogada, JokenpoError>`, obrigando quem chamar o método a lidar com a possibilidade de falha.

**Mapeamento de Arquivos e Módulos:**

1. Crie o arquivo `src/domain/errors.rs`.
2. Edite `src/domain/mod.rs` para incluir os erros.
3. Edite `src/domain/models.rs` para adicionar a implementação.

**Código Incremental Estrito:**

Primeiro, em `src/domain/errors.rs`:

```rust
use thiserror::Error;

// RACIOCÍNIO UNIFICADO (Abstração Humana + Escovação de Bits):
// Iniciante de linguagem dinâmica acha que tratamento de erro é só dar 'throw' numa string 
// aleatória e rezar pro lado de lá não crashear. Em engenharia de software robusta, erros 
// são cidadãos de primeira classe (First-Class Citizens) do seu Domínio. Nós modelamos 
// falhas de forma estrita e finita usando Enums (Tipos Algébricos de Dados).
//
// Só que o compilador (rustc) não aceita qualquer coisa como erro. Para esse Enum se 
// integrar com o resto do ecossistema, ele precisa assinar o contrato da interface nativa 
// 'std::error::Error' e saber se imprimir na tela ('std::fmt::Display'). Escrever essas 
// implementações na mão gera dezenas de linhas de código inútil (Boilerplate). 
//
// É aqui que usamos a crate 'thiserror'. A diretiva #[derive(Error)] é metaprogramação 
// (Metaprogramming) pura. Em tempo de compilação, o compilador pausa, lê a sua Árvore 
// Sintática (AST - Abstract Syntax Tree), esbarra no #[error("...")] e gera dinamicamente 
// todas as instruções de baixo nível de CPU para formatar os bytes do texto. Ele já cria o 
// código que injeta a String interna '{0}' no lugar certo da mensagem, resolvendo o 
// apontamento de memória para você. Isso é o que chamamos de abstração com zero custo de 
// performance em tempo de execução (Zero-Cost Abstraction).
#[derive(Error, Debug)]
pub enum JokenpoError {
    #[error("Jogada inválida: {0}. Escolha pedra, papel ou tesoura.")]
    EntradaInvalida(String),
}
```

Exponha o módulo adicionando em `src/domain/mod.rs`:

```rust
pub mod models;
pub mod errors; // Adicione esta linha
```

Agora, adicione as regras de negócio em `src/domain/models.rs` (no final do arquivo):

```rust
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
```

**Como testar e Resultado Esperado:**

No terminal, execute:

`cargo check`

*Resultado Esperado:* O código compila sem erros. Os módulos se conversam perfeitamente.

---

## Módulo 4: Padrões de Projeto e Modularidade com Traits

### Subtópico: Injeção de Comportamento e Polimorfismo Estático

**Raciocínio e Problema Arquitetural:**

Nossa engine já sabe calcular vitórias. Mas *quem* joga? Pode ser um humano digitando ou um bot aleatório. Em Arquitetura de Software, queremos desacoplar a "Regra do Jogo" do "Fornecedor da Jogada".

Em Rust, alcançamos isso via *Traits* (interfaces). Definimos uma Trait `Jogador` com um método `escolher_jogada`. Criaremos a implementação concreta `JogadorComputador`. Esse padrão permite plugar facilmente um `JogadorHumano` depois, sem alterar a engine principal, respeitando o Princípio Aberto-Fechado (OCP) do SOLID.

**Mapeamento de Arquivos e Módulos:**

1. Crie o arquivo `src/domain/player.rs`.
2. Exponha-o em `src/domain/mod.rs`.

**Código Incremental Estrito:**

Em `src/domain/player.rs`:

```rust
use crate::domain::models::Jogada;
use rand::Rng;

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
```

Atualize o `src/domain/mod.rs`:

```rust
pub mod models;
pub mod errors;
pub mod player; // Adicione esta linha
```

**Como testar e Resultado Esperado:**

No terminal, execute:

`cargo check`

*Resultado Esperado:* Compilação bem-sucedida com as bibliotecas do `rand` sendo corretamente utilizadas para resolver a abstração da nossa Trait.

---

## Módulo 5: Gerenciamento de Memória e Concorrência

### Subtópico: Simulação Paralela com Arc e Mutex

**Raciocínio e Problema Arquitetural:**

E se quisermos simular 10 rodadas simultâneas (em threads separadas) de Computador vs Computador para ver se o randomizador é justo, e salvar o total de vitórias do "Player 1" em um placar global?

Em C++, compartilhar estado entre threads facilmente resulta em *Data Races* (condições de corrida) e falhas de segmentação. 

Rust garante *Fearless Concurrency* (Concorrência sem Medo). Para um estado mutável compartilhado entre threads, o compilador nos obriga a usar primitivas de sincronização. 

Usaremos `Arc` (Atomic Reference Counting) para permitir múltiplos "donos" seguros da referência da variável, e `Mutex` (Mutual Exclusion) para garantir que apenas uma thread altere o placar por vez.

**Mapeamento de Arquivos e Módulos:**

1. Altere o arquivo `src/main.rs`.

**Código Incremental Estrito:**

Substitua todo o conteúdo de `src/main.rs` por:

```rust
mod domain;

use domain::models::Resultado;
use domain::player::{Jogador, JogadorComputador};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // AQUI: Criamos um estado protegido. Mutex garante mutabilidade segura; Arc garante ciclos de vida entre threads.
    let vitorias_p1 = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        // AQUI: Clonamos apenas a referência do Arc (barato), aumentando o contador de referências.
        let contador_clonado = Arc::clone(&vitorias_p1);
        
        // AQUI: A thread assume o Ownership de suas variáveis (clausura move).
        let handle = thread::spawn(move || {
            let p1 = JogadorComputador;
            let p2 = JogadorComputador;
            
            let jogada_p1 = p1.escolher_jogada();
            let jogada_p2 = p2.escolher_jogada();
            let resultado = jogada_p1.avaliar(&jogada_p2);

            println!("Thread {}: {:?} vs {:?} -> {:?}", i, jogada_p1, jogada_p2, resultado);

            if resultado == Resultado::Vitoria {
                // AQUI: Travamos o Mutex para alterar o valor interior com segurança.
                let mut num = contador_clonado.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    // Aguarda todas as threads terminarem.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total de vitórias do P1 na simulação: {}", *vitorias_p1.lock().unwrap());
}
```

**Como testar e Resultado Esperado:**

No terminal, execute:

`cargo run`

*Resultado Esperado:* O compilador irá construir o executável e rodá-lo. Você verá 10 linhas no terminal mostrando os resultados de cada thread ocorrendo em ordem possivelmente bagunçada (característica do paralelismo), seguidas pelo somatório de vitórias do Player 1 impresso no final. Sem falhas de segmentação, garantido matematicamente.

---

## Módulo 6: Testes, Otimização e Distribuição

### Subtópico: Testes Unitários e Perfil de Release

**Raciocínio e Problema Arquitetural:**

Engenharia de software madura exige testes automatizados para blindar a regra de negócio contra regressões. Em Rust, os testes unitários residem no mesmo arquivo do código fonte, dentro de um módulo isolado pela macro `#[cfg(test)]`. Isso garante que o código de teste não vá para o binário final distribuído para os clientes, economizando espaço e memória. Após validar as regras, compilamos a aplicação habilitando otimizações avançadas do LLVM (loop unrolling, inlining agressivo, remoção de código morto).

**Mapeamento de Arquivos e Módulos:**

1. Adicione os testes no final de `src/domain/models.rs`.

**Código Incremental Estrito:**

Adicione o bloco abaixo no **final** de `src/domain/models.rs`:

```rust
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
```

**Como testar e Resultado Esperado:**

1. **Rodar os Testes:**
    
    No terminal, execute:
    
    `cargo test`
    
    *Resultado Esperado:* A saída mostrará `running 3 tests`, listando `test_vitoria_pedra_sobre_tesoura`, `test_parse_sucesso` e `test_parse_falha_erro_customizado` todos com o status `ok`.
    
2. **Compilar para Produção (Otimização):**
    
    No terminal, execute:
    
    `cargo build --release`
    
    *Resultado Esperado:* O Cargo fará uma nova compilação no perfil `release`. Levará um pouco mais de tempo, pois o compilador aplica as otimizações do LLVM. O binário altamente otimizado será gerado em `target/release/jokenpo_ddd`.
    

---

---

 Agora vamos dar vida ao nosso jogo permitindo a interação com o usuário pelo terminal.

Como combinamos, vou detalhar o raciocínio arquitetural e de ciência da computação por trás de cada decisão antes de tocarmos no código, mantendo nossa estrutura de módulos.

---

## Expandindo o Módulo 4: Padrões de Projeto e Modularidade com Traits

### Subtópico: Interação de I/O (Terminal) e o Padrão "Retry" para Contratos Inflexíveis

**Raciocínio e Problema Arquitetural:**

Temos um problema clássico de engenharia de software aqui: como mapear a entrada imprevisível do mundo externo (um usuário digitando no teclado) para um modelo de domínio estrito e determinístico?

A nossa interface (a Trait `Jogador`) exige o seguinte contrato: `fn escolher_jogada(&self) -> Jogada`. Repare que a assinatura da função **não** retorna um `Result` ou `Option`. Ela promete que, custe o que custar, devolverá uma `Jogada` válida. No entanto, ler do `stdin` (entrada padrão) é inerentemente suscetível a falhas (erros de I/O ou o usuário digitando algo inválido como "fogo").

Para resolver essa discrepância de design sem poluir a nossa Trait pura com lógicas de erro de I/O, utilizamos o **Padrão Retry (Tentativa)** encapsulado na implementação concreta. Criaremos um loop infinito na struct `JogadorHumano` que fará a leitura do terminal. Se o usuário errar, nós interceptamos o erro (usando nossa conversão do Módulo 3) e forçamos uma nova iteração. O método só retorna (e quebra o loop) quando obtiver sucesso, cumprindo o contrato da interface com segurança.

**Mapeamento de Arquivos e Módulos:**

1. Abra o arquivo `src/domain/player.rs`.

**Código Incremental Estrito:**

Adicione as seguintes importações no **topo** do arquivo `src/domain/player.rs` (logo abaixo dos `use` existentes):

```rust
// AQUI: std::io para manipular entrada/saída, e Write para forçar a exibição do print! antes do input.
use std::io::{self, Write};
// AQUI: Importamos a trait FromStr para que o compilador saiba que a string pode chamar o método parse().
use std::str::FromStr;
```

Agora, adicione o seguinte bloco de código no **final** do arquivo `src/domain/player.rs`:

```rust
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
```

**Como testar e Resultado Esperado:**

No terminal, execute o comando:

`cargo check`

*Resultado Esperado:* O código deve compilar sem erros. O compilador atesta que a Trait `Jogador` foi perfeitamente implementada para `JogadorHumano` e que os tipos batem.

---

## Expandindo o Módulo 5: Gerenciamento de Memória e Concorrência

### Subtópico: Substituição do Paralelismo pelo Loop de Eventos Síncrono (Main Flow)

**Raciocínio e Problema Arquitetural:**

No passo anterior, testamos nossa engine criando 10 threads concorrentes (`Arc` e `Mutex`) onde o computador jogava contra si mesmo. Porém, sistemas interativos de terminal que dependem do `stdin` não lidam bem com múltiplas threads tentando capturar a entrada do teclado simultaneamente (ocasionaria uma condição de corrida no bloqueio de I/O do sistema operacional).

Para transformar nossa simulação em um jogo real, vamos descartar a concorrência assíncrona da `main` e implementar um **Game Loop (Loop de Jogo)** síncrono. Esta é a arquitetura base de praticamente qualquer motor de jogos do mercado. Aqui aplicaremos a **Injeção de Dependência**: instanciaremos o `JogadorHumano` e o `JogadorComputador` lado a lado. A lógica de avaliação do jogo continuará sem fazer ideia de quem gerou a jogada, provando o alto nível de desacoplamento que nossas Traits garantiram.

**Mapeamento de Arquivos e Módulos:**

1. Abra o arquivo `src/main.rs`.

**Código Incremental Estrito:**

**Substitua** todo o conteúdo do seu arquivo `src/main.rs` pelo bloco abaixo:

```rust
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
```

**Como testar e Resultado Esperado:**

No terminal, execute o comando:

`cargo run`

*Resultado Esperado:* 1. O terminal exibirá o título e pedirá sua jogada: 

`Digite sua jogada (pedra, papel, tesoura):`.

2. Se você digitar algo errado (ex: `fogo`), o programa **não vai quebrar (panic)**. Ele usará nosso Tratamento de Erro para exibir `Jogada inválida: fogo. Escolha pedra, papel ou tesoura.` e pedirá a entrada novamente.

3. Se você digitar uma jogada válida (ex: `pedra`), o motor irá sortear a jogada do computador, calcular o vencedor e exibir o resultado visualmente.

4. O programa perguntará se você deseja continuar. Digitar `s` inicia o ciclo novamente, qualquer outra coisa encerra o executável com segurança.