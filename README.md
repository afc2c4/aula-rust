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
# RACIOCÍNIO UNIFICADO (Akita Mode: Abstração + Bits):
# Entenda: o 'Cargo.toml' é o Manifesto do seu projeto (Project Manifest), 
# onde ele descreve o que o projeto é e do que ele precisa para compilar e rodar,
# onde ele reúne os metadados do projeto e define dependências e configurações de compilação.
 
# Ele não é só uma lista de compras; é o contrato que define como o Orquestrador 
# de Build (Cargo) deve montar o seu linguição de bits. 
# Diferente do JavaScript, onde você baixa binários pré-compilados e reza pro 
# 'node_modules' não quebrar, o Rust baixa o código-fonte e compila tudo 
# localmente para garantir compatibilidade binária total com a sua CPU.

[dependencies]
# RACIOCÍNIO HUMANO: Precisamos de entropia (aleatoriedade) pra máquina jogar.
# RACIOCÍNIO DO COMPUTADOR: O Cargo resolve o grafo de dependências 
# (Dependency Graph), busca os bytes no crates.io e o compilador (rustc) 
# faz o Link Estático (Static Linking). O código da biblioteca morre dentro 
# do seu binário final, sem depender de DLLs externas no SO.
rand = "0.8.5"

# RACIOCÍNIO HUMANO: Queremos erros elegantes sem escrever centenas de linhas inúteis.
# RACIOCÍNIO DO COMPUTADOR: Essa crate habilita Macros Procedurais 
# (Procedural Macros). Em tempo de compilação (Compile-time), ela faz 
# Metaprogramação: lê suas structs e injeta instruções de baixo nível para 
# implementar as Traits de erro automaticamente. É o que chamamos de 
# Abstração de Custo Zero (Zero-Cost Abstraction).
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

Em muitas linguagens, desenvolvedores representam escolhas finitas usando strings (`"pedra"`, `"papel"`) ou inteiros (`0`, `1`, `2`). Isso é um anti-pattern conhecido como *Primitive Obsession*, onde usaríamos tipos primitivos demais (String, int, bool, etc.) para representar conceitos de negócio que mereciam tipos próprios.

Se usarmos uma string, o compilador não pode nos impedir de passar `"fogo"` ou `"lagarto"`, causando erros em tempo de execução.

Em Rust, usamos *Enums* (Tipos Algébricos de Dados). Eles garantem tipagem estrita (cada valor tem um tipo certo, e misturas erradas dão erro na compilação) e exaustividade ( em estruturas como match, você é obrigado a tratar todos os casos possíveis) em tempo de compilação. Se o nosso domínio define que existem apenas três jogadas possíveis, o sistema de tipos deve refletir isso matematicamente. O mesmo vale para o resultado da partida.

**Mapeamento de Arquivos e Módulos:**

1. Crie uma pasta chamada `domain` dentro de `src/`: `mkdir src/domain`
2. Crie o arquivo `src/domain/models.rs`.
3. Crie o arquivo `src/domain/mod.rs` (para expor os módulos).

**Código Incremental Estrito:**

Adicione o seguinte código no arquivo recém-criado `src/domain/models.rs`:

```rust
// RACIOCÍNIO HUMANO (A Abstração): 
// Pare de ser um "vibe coder" e aprenda Domain-Driven Design (DDD). No JavaScript, 
// você usaria strings ("pedra"). Se alguém digitar "PEDRA" ou "podra", seu 
// 'if' quebra em runtime. Usando Enums (Algebraic Data Types - ADTs), você cria 
// uma Linguagem Ubíqua (Ubiquitous Language) no código. O domínio diz que só 
// existem 3 opções; o compilador (rustc) vira seu sargento e não deixa o programa 
// nem compilar se você tentar inventar uma quarta opção.
//
// RACIOCÍNIO DO COMPUTADOR (A Escovação de Bits): 
// O compilador é preguiçoso. A macro #[derive] é Metaprogramação (Metaprogramming), onde o 
// o compilador gera código automaticamente para você.
// Em tempo de compilação, o Rust lê sua definição e "escreve" o código de baixo nível pra você:
// - Debug (Formatting): Gera as instruções pro binário saber como converter esses 
//   bits em texto legível para o 'stdout' (“saída padrão” de um programa).
// - PartialEq/Eq (Comparison): Gera a lógica de Comparação Bit-a-Bit (Bitwise Comparison) 
//   direto na CPU. No JS, comparar objetos é caro; aqui é uma instrução simples de registrador.
// 
// - Clone/Copy (Semântica de Memória): Este Enum é uma Tagged Union (União Etiquetada), QUE  
//   pode guardar um entre vários formatos possíveis, junto com uma etiqueta que diz qual formato está ativo naquele momento.
//   Ele ocupa apenas 1 byte na RAM. Avisamos ao compilador que é mais barato 
//   duplicar esse byte na Pilha (Stack) do que rastrear Posse (Ownership).
// 
//   É a famosa Abstração de Custo Zero (Zero-Cost Abstraction), onde
//   você escreve código em alto nível (mais claro e seguro), mas sem pagar desempenho extra em tempo de execução.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Jogada {
    Pedra,
    Papel,
    Tesoura,
}

// RACIOCÍNIO HUMANO (A Abstração): 
// Define o desfecho determinístico da rodada. No Rust, não existe "undefined". 
// Ou você tem um Resultado, ou o código não compila.
//
// RACIOCÍNIO DO COMPUTADOR (A Escovação de Bits): 
// Preste atenção no que NÃO está aqui: 'Clone' e 'Copy'. Por quê? Porque não 
// existe almoço grátis (No Free Lunch). O 'Resultado' é gerado pela engine, 
// lido por um Casamento de Padrões (Pattern Matching) na main e imediatamente 
// descartado (Dropped). Como ele não transita entre múltiplas threads ou 
// funções complexas, não faz sentido instruir o compilador a gerar código de 
// cópia para um dado que morre no mesmo Ciclo de Vida (Lifetime) em que nasceu.
#[derive(Debug, PartialEq, Eq)]
pub enum Resultado {
    Vitoria,
    Derrota,
    Empate,
}
```

Agora, registre o módulo no arquivo `src/domain/mod.rs`:

```rust
// RACIOCÍNIO UNIFICADO (Akita Mode: Abstração + Bits):
// Pare de ser amador. No JS, você dá um 'export' em tudo e reza. Aqui, 
// o 'mod' é uma Diretiva de Compilação (Compiler Directive). Você está 
// mandando o rustc dar um 'STOP' na main, ir no disco, ler 'models.rs', 
// fazer o parse pra Árvore Sintática Abstrata (AST) e compilar.
//
// O 'pub' (Public Visibility) é a sua única forma de alterar a Tabela de 
// Símbolos (Symbol Table). Sem o 'pub', o Linker não enxerga as structs 
// lá dentro. É assim que Rust garante que você não acesse o que não deve, 
// sem precisar de custos de runtime ou máquinas virtuais lentas.
pub mod models;
```

### A Vantagem Real em cima do JavaScript (Node.js)

1. **Custo de Resolução:** No JavaScript, o Node.js precisa percorrer o sistema de arquivos em **tempo de execução** (Runtime) toda vez que encontra um `require`, o que é lento. No Rust, toda a árvore de módulos é resolvida e "achatada" em **tempo de compilação**. O custo no executável final é **zero**.
2. **Segurança de Fronteira:** No JS, é muito fácil exportar algo por acidente. No Rust, se você não declarar `pub mod`, o compilador nem olha pro arquivo. Ele é cego por padrão. Isso força você a desenhar a arquitetura (Bounded Contexts) antes de sair cuspindo código.
3. **Compilação Incremental:** Como a árvore é explícita, o `cargo` sabe exatamente qual pedaço do gráfico de módulos mudou, recompilando apenas os bits necessários. No JS, os bundlers (Webpack/Vite) sofrem para fazer o mesmo "tree shaking" que o Rust faz nativamente.

E no topo do seu `src/main.rs`, adicione:

```rust
// RACIOCÍNIO HUMANO (A Abstração): 
// Pare de ser amador. No JavaScript (Node.js), você simplesmente faz um 'import' e 
// o runtime se vira pra achar o arquivo em tempo de execução. No Rust, você desenha 
// a árvore de módulos na mão. 'mod domain' define a fronteira do seu Bounded Context 
// (Contexto Delimitado). Você está dizendo: "Olha, existe um sub-sistema de regras 
// de negócio chamado 'domain', conecte ele agora".
//
// RACIOCÍNIO DO COMPUTADOR (A Escovação de Bits): 
// O arquivo 'main.rs' é o ponto zero do compilador (Crate Root). O 'rustc' é cego 
// pro seu HD. Esta linha é uma Diretiva de Compilação (Compiler Directive) estrita. 
// Ela manda a CPU fazer I/O no disco, ler 'domain/mod.rs', fazer o parse para a 
// Árvore Sintática Abstrata (AST) e instruir o Linker a juntar esses bits no 
// binário final. Sem isso, a pasta é ignorada (Dead Code Elimination).
//
// VANTAGEM SOBRE JAVASCRIPT: No JS, o custo de resolver onde o arquivo está 
// acontece em runtime, toda vez que o programa roda. No Rust, isso é resolvido 
// uma única vez em tempo de compilação. O custo no executável final é ZERO.
mod domain;

// RACIOCÍNIO HUMANO (A Abstração): 
// Todo software executável precisa de uma porta de entrada inquestionável. 
// A 'main' é o marco zero do seu fluxo de execução (Control Flow). 
// O 'println!' é só um feedback visual pra você saber que o processo não 
// "morreu" ao ser carregado pelo Sistema Operacional.
//
// RACIOCÍNIO DO COMPUTADOR (A Escovação de Bits): 
// O Linker mapeia a 'fn main()' para o Entry Point oficial exigido pelo SO. 
// Quando o binário entra na RAM, o ponteiro de instrução da CPU (Instruction Pointer) 
// aponta exatamente pro primeiro byte desta função. 
// O '!' em 'println!' avisa que isso é uma Macro (Metaprogramming). Em tempo de 
// compilação, o Rust expande isso gerando um código gigante de baixo nível para 
// fazer uma Syscall (Chamada de Sistema) segura, travando o buffer do terminal 
// (Standard Output) para garantir que você não tenha uma Falha de Segmentação.
//
// VANTAGEM SOBRE JAVASCRIPT: No JS, o 'console.log' é uma função dinâmica que 
// depende de um objeto global pesado no runtime. No Rust, o código de impressão 
// é injetado diretamente no binário como instruções nativas. É mais rápido e 
// consome ordens de grandeza menos memória.
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
Se o seu tratamento de erro é dar um `throw` numa string aleatória e torcer para o `try/catch` do outro lado da aplicação adivinhar o que aconteceu, você não está programando, você está jogando dados. Em engenharia de software de verdade, falhas são parte do seu **Domínio**.

O problema que resolvemos aqui é a **Sinalização de Falhas Determinística**. No JavaScript, você lança uma exceção que interrompe o fluxo de execução (Stack Unwinding) e consome recursos pesados do runtime. No Rust, usamos o sistema de tipos para dizer: "Esta função pode retornar uma `Jogada` ou este erro específico".

Usamos a crate `thiserror` para evitar o trabalho braçal de implementar manualmente as Traits nativas de exibição de texto (`Display`) e identificação de erro (`Error`). Isso garante que nossa abstração de alto nível não custe nem um ciclo de CPU extra no binário final.

Primeiro, em `src/domain/errors.rs`:

```rust
use thiserror::Error;

// RACIOCÍNIO UNIFICADO (Akita Mode):
// No JS, você tem o 'throw' que é um "GoTo" glorificado e imprevisível. Aqui, o JokenpoError 
// é um Cidadão de Primeira Classe (First-Class Citizen). Se o usuário digitar "fogo", 
// isso não é um erro de sistema, é um evento de domínio: 'EntradaInvalida'.
//
// PRO COMPUTADOR: A diretiva #[derive(Error, Debug)] aciona uma Macro Procedural. 
// Em tempo de compilação, o rustc pausa, lê sua struct e "escreve" o código de 
// baixo nível pra CPU saber como formatar esses bytes no terminal. 
//
// VANTAGEM SOBRE JS: Enquanto no Node.js você gasta memória rastreando a Stack Trace 
// inteira de uma exceção, o Rust usa Enums (Tagged Unions) que ocupam o espaço exato 
// do dado na Stack. É a prova de que não existe almoço grátis: você tem a conveniência 
// da mensagem bonitinha #[error("...")] sem pagar o pato da performance em runtime.
#[derive(Error, Debug)]
pub enum JokenpoError {
    // O '{0}' instrui o compilador a injetar a String interna na mensagem de texto 
    // gerada, resolvendo o apontamento de memória automaticamente para você.
    #[error("Jogada inválida: {0}. Escolha pedra, papel ou tesoura.")]
    EntradaInvalida(String),
}
```

Em engenharia de software de verdade, o código não se "auto-descobre".

O problema que resolvemos aqui é o do **Encapsulamento e Granularidade**. No JavaScript (Node.js), você simplesmente faz um `require('./errors')` ou um `import` e o runtime se vira para achar o arquivo no disco em tempo de execução. Isso é "mágico", lento e perigoso, porque incentiva código espaguete onde qualquer um acessa qualquer coisa. No Rust, o sistema de módulos é uma **árvore explícita**.

A pasta `domain` não existe para o compilador até que você a declare na raiz. E o arquivo `mod.rs` é o "contrato de exportação" dessa pasta. Ele decide o que é privado (detalhe de implementação) e o que é público (API). Sem o `pub mod`, seu arquivo `errors.rs` é apenas lixo no HD que o compilador ignora para economizar processamento (**Dead Code Elimination**).

Exponha o módulo adicionando em `src/domain/mod.rs`:

```rust
pub mod models;
// RACIOCÍNIO UNIFICADO (Akita Mode: Abstração + Bits):
// Pare de ser amador. No JS, você dá um 'export' em tudo e reza pro runtime 
// achar. Aqui, o 'mod' é uma Diretiva de Compilação (Compiler Directive) 
// estrita. Você está mandando o rustc dar um 'STOP', ir no disco, ler 
// 'errors.rs', fazer o parse pra Árvore Sintática Abstrata (AST) e compilar.
//
// O 'pub' (Public Visibility) é a sua única forma de alterar a Tabela de 
// Símbolos (Symbol Table) do compilador. Sem o 'pub', o Linker não enxerga 
// os endereços de memória dessas structs. É assim que Rust garante que 
// você não acesse o que não deve, sem precisar de custos de runtime ou 
// máquinas virtuais pesadas.
pub mod errors;
```

### A Vantagem Real em cima do JavaScript (Node.js)

1. **Custo de Resolução Zero:** No JavaScript, o Node.js precisa percorrer o sistema de arquivos em **tempo de execução** toda vez que encontra um import, o que consome I/O e CPU desnecessariamente. No Rust, toda a árvore é resolvida e "achatada" em **tempo de compilação**. O custo no executável final é **zero**.
2. **Segurança de Fronteira:** No JS, é muito fácil exportar um objeto por acidente e vazar estado interno. No Rust, se você não declarar `pub mod`, o compilador sequer olha pro arquivo. Ele é cego por padrão. Isso te força a desenhar os **Bounded Contexts** do seu Jokenpo antes de sair cuspindo código.
3. **Binário Otimizado:** Como a árvore é explícita, o compilador sabe exatamente o que não está sendo usado e simplesmente não inclui esses bits no executável. O "Tree Shaking" do Rust é nativo e agressivo, enquanto no JS você depende de ferramentas externas como Rollup ou Webpack que muitas vezes falham.

Agora e agora vamos blindar o domínio e escova os bits para que a CPU não trabalhe à toa.

O problema aqui é o da **Primitive Obsession** (Obsessão por Primitivos) e o custo de **Runtime Exceptions**. No JavaScript, você faria um `parseInt` ou um `if(s === "pedra")` e torceria para não vir um `undefined`. No Rust, usamos o bloco `impl` para colar a lógica no dado (Domain Driven Design) e a Trait `FromStr` para criar uma barreira física: o dado sujo do mundo externo (Terminal/Rede) morre na entrada, e apenas tipos válidos entram no coração do sistema.

Agora, adicione as regras de negócio em `src/domain/models.rs` (no final do arquivo):

```rust
use std::str::FromStr;
use crate::domain::errors::JokenpoError;

// RACIOCÍNIO UNIFICADO (Akita Mode):
// Entenda: Lógica de Domínio (Domain Logic) DEVE estar grudada no dado. 
// No binário, o bloco 'impl' instrui o rustc a associar funções ao Nome de Espaço (Namespace). 
// Métodos não ocupam espaço na RAM; são apenas endereços (Instruction Pointers) 
// no executável. É o ápice do desacoplamento sem custo de runtime.
impl Jogada {
    
    // RACIOCÍNIO HUMANO: Pra decidir o vencedor, usamos análise combinatória estrita.
    // PRO COMPUTADOR: Note as referências imutáveis (&). Passamos apenas endereços 
    // da Pilha (Stack Pointers), sem mover Posse (Ownership). O LLVM transforma 
    // o match de tuplas em uma Tabela de Saltos (Jump Table) na CPU.
    // VANTAGEM SOBRE JS: No JavaScript, comparar objetos ou strings exige percorrer 
    // a Heap. No Rust, é O(1) - uma instrução direta de comparação de bits.
    pub fn avaliar(&self, oponente: &Jogada) -> Resultado {
        match (self, oponente) {
            // Casamento de Padrões (Pattern Matching) com operador OR (|).
            (Jogada::Pedra, Jogada::Tesoura) |
            (Jogada::Papel, Jogada::Pedra) |
            (Jogada::Tesoura, Jogada::Papel) => Resultado::Vitoria,
            
            // Cláusula de Guarda (Guard Clause) para comparação bit-a-bit (Bitwise Comparison).
            (a, b) if a == b => Resultado::Empate,
            
            // Coringa (Wildcard / Catch-all) para o resto (Default Case).
            _ => Resultado::Derrota, 
        }
    }
}

// RACIOCÍNIO HUMANO: Criamos um Bounded Context (Contexto Delimitado). 
// PRO COMPUTADOR: Ao assinar o contrato da Trait 'FromStr', você habilita 
// Abstração de Custo Zero. O 'Result' é uma União Etiquetada (Tagged Union) 
// que obriga você, em tempo de compilação, a tratar o Erro.
// VANTAGEM SOBRE JS: No Node.js, você usaria try/catch, que "fura" a Stack 
// de execução e é lento. Aqui, o erro é apenas um valor de retorno 
// que o compilador não deixa você ignorar. Sem bugs de "undefined" em produção.
impl FromStr for Jogada {
    type Err = JokenpoError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Sanitização (Sanitization) e normalização (Normalization).
        match s.trim().to_lowercase().as_str() {
            "pedra" => Ok(Jogada::Pedra),
            "papel" => Ok(Jogada::Papel),
            "tesoura" => Ok(Jogada::Tesoura),
            // Encapsulamento em Erro de Domínio (Domain Error Handling).
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

Aqui, a gente usa abstrações pra guiar o compilador a gerar o binário mais eficiente possível, sem desperdiçar um único ciclo de clock.

Em `src/domain/player.rs`:

```rust
use crate::domain::models::Jogada;
use rand::Rng;

// RACIOCÍNIO UNIFICADO (Akita Mode):
// No JavaScript ou TypeScript, uma interface é apenas uma "sugestão" que o transpiler 
// joga no lixo assim que gera o .js (Type Erasure). Aqui, a 'trait' é um contrato 
// físico. O compilador usa isso pra realizar a Monomorfização (Monomorphization): 
// ele gera uma versão do código pra cada tipo que implementa a Trait.
// 
// PRO COMPUTADOR: Isso habilita o Despacho Estático (Static Dispatch). Diferente 
// do Java ou C# que usam tabelas de métodos virtuais (Vtable) lentas que exigem 
// saltos indiretos na memória, o Rust resolve o endereço da função em tempo de 
// compilação. É performance de C puro com a segurança de uma linguagem moderna.
pub trait Jogador {
    fn escolher_jogada(&self) -> Jogada;
}

// RACIOCÍNIO HUMANO: Definimos o Bot como uma entidade isolada.
// PRO COMPUTADOR: Esta é uma 'Unit-like Struct'. Ela é um Tipo de Tamanho Zero 
// (Zero-Sized Type - ZST). No binário final, ela ocupa EXATAMENTE 0 bytes na RAM. 
// VANTAGEM SOBRE JS: No Node.js, qualquer "class Player {}" vazia vira um objeto 
// pesado na Heap com metadados e cadeia de protótipos. No Rust, é custo zero.
pub struct JogadorComputador;

impl Jogador for JogadorComputador {
    fn escolher_jogada(&self) -> Jogada {
        // 'thread_rng' acessa o gerador do SO. Isso pode envolver uma Syscall 
        // (Chamada de Sistema) para buscar entropia do hardware. 
        let mut rng = rand::thread_rng();

        // RACIOCÍNIO HUMANO: O 'match' garante que a máquina nunca esqueça uma regra.
        // PRO COMPUTADOR: O compilador transforma esse match em uma Tabela de Saltos 
        // (Jump Table) na CPU. Em vez de avaliar múltiplos 'if/else' (Branching) 
        // que podem causar erros de previsão de salto (Branch Misprediction), 
        // a CPU pula direto pro endereço de memória do variante do Enum. 
        // É complexidade O(1), a velocidade máxima que o silício permite.
        match rng.gen_range(0..=2) {
            0 => Jogada::Pedra,
            1 => Jogada::Papel,
            _ => Jogada::Tesoura, // (Catch-all) para satisfazer a exaustividade.
        }
    }
}
```

Agora, a árvore de módulos é um contrato explícito de visibilidade e gerenciamento de recursos.

Atualize o `src/domain/mod.rs`:

```rust
pub mod models;
pub mod errors;

// RACIOCÍNIO UNIFICADO (Akita Mode: Abstração + Bits):
// No JavaScript (Node.js), você daria um 'import' ou 'require' e o runtime 
// tentaria resolver o caminho no disco toda vez que o código rodasse. 
// Aqui, o 'mod' é uma Diretiva de Compilação (Compiler Directive) estrita. 
// Você está ordenando que o rustc saia da memória, vá ao HD, leia 'player.rs' 
// e gere a Árvore Sintática Abstrata (AST) desse sub-sistema.

// O 'pub' (Public) altera a Symbol Table do compilador. Sem ele, os outros 
// módulos não conseguem enxergar os endereços de memória das structs e traits 
// de jogadores. É o Bounded Context (Contexto Delimitado) do DDD na prática: 
// você escolhe o que expõe e o que protege. Sem essa linha, o compilador 
// aplica Dead Code Elimination e joga seu arquivo no lixo pra economizar binário.

pub mod player;
```

### A Vantagem Real em cima do JavaScript (Node.js)

1. **Custo de Runtime ZERO:** No JS, a resolução de módulos acontece em **tempo de execução**, o que é lento e custa I/O. No Rust, o gráfico de dependências é "achatado" em **tempo de compilação**. O binário final já sabe exatamente onde cada função está.
2. **Encapsulamento de Verdade:** No JS, é muito fácil exportar algo por engano e vazar estado. No Rust, tudo é **privado por padrão**. Se você não for explícito com `pub mod`, o compilador te blinda de criar código espaguete.
3. **Segurança de Linkagem:** No ecossistema Node, você pode ter erros de "module not found" no meio da execução se esquecer um arquivo. No Rust, se a árvore de módulos não estiver perfeita, o programa sequer é gerado. **É impossível subir um binário incompleto para produção.**

**Como testar e Resultado Esperado:**

No terminal, execute:

`cargo check`

*Resultado Esperado:* Compilação bem-sucedida com as bibliotecas do `rand` sendo corretamente utilizadas para resolver a abstração da nossa Trait.

---

## Módulo 5: Gerenciamento de Memória e Concorrência

### Subtópico: Simulação Paralela com Arc e Mutex

**Raciocínio e Problema Arquitetural:**

Compartilhar memória entre múltiplas threads é como dar uma única caneta para dez pessoas escreverem na mesma folha ao mesmo tempo. Em linguagens "vibe code", você teria um **Data Race** (Condição de Corrida) e seu programa ia cuspir lixo ou explodir. No Rust, o compilador é o seu sargento: ele te obriga a usar padrões de sincronização que garantem a integridade dos bits.

O problema que resolvemos aqui é a **Sincronização de Estado**. No JavaScript (Node.js), você vive no mundo do *Event Loop* (Single-threaded). Para paralelismo real, você usaria *Worker Threads*, mas a comunicação entre elas é feita via troca de mensagens (*message passing*), o que envolve copiar dados e custa caro. No Rust, as threads compartilham o **mesmo espaço de endereçamento**, mas com segurança garantida pelo **Borrow Checker**.

Usamos o **Arc** (Atomic Reference Counting) para permitir que múltiplas threads possuam o ponteiro do placar, e o **Mutex** para garantir que apenas uma CPU por vez altere o bit do contador.

**Mapeamento de Arquivos e Módulos:**

1. Altere o arquivo `src/main.rs`.

**Código Incremental Estrito:**

Substitua todo o conteúdo de `src/main.rs` por:

```rust
// RACIOCÍNIO UNIFICADO (Akita Mode): 
// No Rust, concorrência não é "mágica" de runtime. É engenharia estrita.
mod domain;

use domain::models::Resultado;
use domain::player::{Jogador, JogadorComputador};
// Arc: Atomic Reference Counting (Posse compartilhada entre threads).
// Mutex: Mutual Exclusion (Garante que apenas uma thread acesse o dado).
use std::sync::{Arc, Mutex}; 
use std::thread;

fn main() {
    // RACIOCÍNIO HUMANO: Criamos um placar que todas as threads podem "tocar".
    // RACIOCÍNIO DO COMPUTADOR: Alocamos um inteiro na Heap. O Mutex cria uma barreira 
    // de memória (Memory Barrier). O Arc coloca um contador atômico ao lado do dado. 
    // Enquanto o contador for > 0, o SO não desaloca essa memória (RAII).
    let vitorias_p1 = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        // RACIOCÍNIO DO COMPUTADOR: O clone do Arc NÃO copia o dado (o número 0). 
        // Ele apenas incrementa o contador atômico na CPU (instrução LOCK INC). 
        // É uma operação de nanossegundos que evita o uso de ponteiros inseguros.
        let contador_clonado = Arc::clone(&vitorias_p1);
        
        // thread::spawn solicita ao Kernel do SO a criação de uma thread nativa.
        // O 'move' transfere a Posse (Ownership) das variáveis capturadas para a thread.
        let handle = thread::spawn(move || {
            let p1 = JogadorComputador;
            let p2 = JogadorComputador;
            
            let jogada_p1 = p1.escolher_jogada();
            let jogada_p2 = p2.escolher_jogada();
            let resultado = jogada_p1.avaliar(&jogada_p2);

            println!("Thread {}: {:?} vs {:?} -> {:?}", i, jogada_p1, jogada_p2, resultado);

            if resultado == Resultado::Vitoria {
                // RACIOCÍNIO HUMANO: "Peraí, vou travar o placar pra anotar meu ponto".
                // RACIOCÍNIO DO COMPUTADOR: Invocamos uma Syscall (Futex no Linux). 
                // Se o Mutex estiver travado, a thread entra em 'Sleep' e sai da CPU. 
                // O 'unwrap' trata o 'Poisoning' (se uma thread morrer com o cadeado na mão).
                let mut num = contador_clonado.lock().unwrap();
                *num += 1; // Dereferencing: alteramos o valor direto no endereço de memória.
            }
        });
        handles.push(handle);
    }

    // O 'join' suspende a thread principal até que o Program Counter (PC) 
    // de todas as threads filhas chegue ao fim. Sem isso, o processo morre antes 
    // dos trabalhadores terminarem.
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Total de vitórias do P1: {}", *vitorias_p1.lock().unwrap());
}
```

### A Vantagem Real em cima do JavaScript (Node.js)

1. **Overhead de Memória:** No JavaScript, cada *Worker* carrega uma nova instância da V8, consumindo dezenas de megabytes. No Rust, cada thread consome apenas o tamanho da sua **Stack** (geralmente 2MB), compartilhando o código binário e os dados via Arc.
2. **Paralelismo vs Concorrência:** No Node.js, você tem concorrência (várias coisas "andando" ao mesmo tempo no Event Loop). No Rust, você tem **Paralelismo Real**: os 10 jogos de Jokenpo rodam em núcleos diferentes da CPU simultaneamente.
3. **Segurança em Compile-time:** No JS, se você usar `SharedArrayBuffer` sem `Atomics.add`, você vai ter corrupção de memória e o JS não vai te avisar. No Rust, se você tentar acessar o contador sem dar o `.lock()`, o código **simplesmente não compila**.

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
// RACIOCÍNIO UNIFICADO: Ativando a blindagem do motor de regras.
#[cfg(test)] // (Conditional Compilation): Só compila se o flag 'test' estiver ativo.
mod tests {
    use super::*; // (Scope Mapping): Puxa 'Jogada' e 'Resultado' do módulo pai sem re-alocar símbolos.
    use std::str::FromStr;

    #[test] // (Test Metadata): Marca a função como um entry-point para o runner do Cargo.
    fn test_vitoria_pedra_sobre_tesoura() {
        // RACIOCÍNIO DO COMPUTADOR: Executa 'avaliar' e faz a comparação direta de bits (Bitwise Comparison).
        // Se o resultado na Stack for diferente de 'Vitoria', dispara um Panic e encerra o processo de teste.
        assert_eq!(Jogada::Pedra.avaliar(&Jogada::Tesoura), Resultado::Vitoria);
    }

    #[test]
    fn test_parse_sucesso() {
        // RACIOCÍNIO HUMANO: Garante que espaços e letras maiúsculas não quebrem o I/O.
        let jogada = Jogada::from_str("  PaPeL ").unwrap();
        assert_eq!(jogada, Jogada::Papel);
    }

    #[test]
    fn test_parse_falha_erro_customizado() {
        // RACIOCÍNIO DO COMPUTADOR: Checa se o 'Result' retornou a variante 'Err' (Error Variant Checking).
        // Diferente do JS, onde você daria um 'catch', aqui validamos o estado do Enum retornado.
        let erro = Jogada::from_str("fogo");
        assert!(erro.is_err());
    }
}
```

**Raciocínio Humano (A Abstração):**
O problema que resolvemos aqui é o **Medo de Refatoração (Fear of Refactoring)**. Sem testes, se você decidir mudar a regra do Jokenpo amanhã, você vai quebrar o sistema e só descobrir quando o usuário reclamar. Em Rust, os testes moram no mesmo arquivo do código (In-source testing), garantindo que a documentação técnica (os testes) nunca fique defasada em relação à implementação.

**Raciocínio do Computador (A Escovação de Bits):**
A macro `#[cfg(test)]` é uma **Diretiva de Compilação Condicional (Conditional Compilation)**. Ela instrui o compilador (`rustc`) a tratar esse bloco como "código morto" durante um `cargo build` normal. Ele literalmente deleta esses bytes do binário final. A CPU nunca vai carregar essas instruções em produção. Só quando você roda `cargo test` é que o compilador ativa o **Test Harness** (Equipamento de Teste) nativo e gera um executável separado para validação.

**Vantagem sobre o JavaScript:**
No ecossistema Node.js, você precisa de bibliotecas externas (Jest, Mocha, Vitest) que incham o seu `package.json` e exigem configurações complexas de *bundlers* para garantir que o código de teste não vá parar no seu *bundle* de produção. Em Rust, isso é nativo, garantindo **Zero-Cost Abstraction** (Abstração de Custo Zero): você tem a conveniência dos testes sem pagar 1 byte de overhead no executável final.

**Como testar e Resultado Esperado:**

1. **Rodar os Testes:**
    
    No terminal, execute:
    
    `cargo test`
    
    *Resultado Esperado:* 
    
    A saída mostrará `running 3 tests`, listando `test_vitoria_pedra_sobre_tesoura`, `test_parse_sucesso` e `test_parse_falha_erro_customizado` todos com o status `ok`.
    
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
// RACIOCÍNIO UNIFICADO (Akita Mode: Abstração + Bits):
// No JavaScript, você tem o 'console' e o 'process' jogados no escopo global (Global Scope). 
// É conveniente? Sim. É porco? Com certeza. O Rust segue a filosofia de "não pague pelo 
// que não usa". O 'use' não é um 'import' de runtime que carrega arquivos pesados; 
// é só um aviso pro compilador (rustc) mapear nomes na Tabela de Símbolos (Symbol Table).
//
// std::io::{self, Write}:
// - io (Module): Abre o sub-sistema de entrada e saída.
// - Write (Trait): Aqui está o segredo. Stdout é "Line Buffered" (Bufferizado por Linha). 
//   Se você der um 'print!' sem '\n', os bits ficam presos no buffer da RAM e não chegam 
//   no terminal. Importamos a interface 'Write' para ter acesso à instrução 'flush' 
//   (Buffer Flush), que força a Syscall do SO a despejar os dados na tela antes de 
//   travar o programa esperando o usuário.
use std::io::{self, Write};

// std::str::FromStr (Trait):
// No JS, você faz um 'parseInt()' ou uma gambiarra qualquer e reza pra não vir um 'NaN'. 
// Em Rust, usamos Polimorfismo Ad-hoc (Ad-hoc Polymorphism) via Traits. Ao importar 
// 'FromStr', você habilita o compilador a "enxergar" que uma string pode se transformar 
// no seu tipo 'Jogada' através de um contrato padronizado. É o que chamamos de 
// Interface de Parsing (Parsing Interface).
use std::str::FromStr;
```

Agora, adicione o seguinte bloco de código no **final** do arquivo `src/domain/player.rs`:

```rust
// RACIOCÍNIO UNIFICADO (Akita Mode: Abstração + Bits):
// No JS, você faria uma classe com constructor vazio. Aqui, 'pub struct JogadorHumano;' 
// é uma 'Unit-like Struct'. Pro computador, isso é um ZST (Zero-Sized Type). 
// Ocupa 0 bytes na RAM. É só um marcador pro compilador associar métodos.
pub struct JogadorHumano;

impl Jogador for JogadorHumano {
    fn escolher_jogada(&self) -> Jogada {
        // RACIOCÍNIO HUMANO: Implementamos o 'Retry Pattern'. Se o usuário for um 
        // imbecil e digitar errado, o sistema não crasha; ele pergunta de novo.
        //
        // RACIOCÍNIO DO COMPUTADOR: 'loop' gera uma instrução JMP (Jump) incondicional 
        // no nível da CPU. Diferente do JS, onde loops infinitos podem travar o Event Loop 
        // se não forem asíncronos, aqui temos controle total do fluxo de execução.
        loop {
            print!("Digite sua jogada (pedra, papel, tesoura): ");
            
            // RACIOCÍNIO DO COMPUTADOR: 'stdout' é bufferizado por linha. Como o 'print!' 
            // não tem '\n', os bits ficam "presos" na RAM. O 'flush()' é a instrução 
            // que força a CPU a despejar esses bytes na tela antes de parar tudo.
            io::stdout().flush().unwrap();

            // 'String::new()' faz uma alocação dinâmica na HEAP. 
            let mut input = String::new();
            
            // 'read_line' faz uma Syscall pro SO. Passamos uma referência mutável (&mut). 
            // Não estamos movendo a posse; estamos dando permissão pro SO preencher 
            // esse endereço de memória com o que vier do teclado.
            if io::stdin().read_line(&mut input).is_err() {
                println!("Erro de I/O. Tentando novamente...");
                continue; // (Control Flow Optimization)
            }

            // RACIOCÍNIO HUMANO: Aqui está a vantagem em cima do JavaScript. 
            // No JS, você faria um 'parse' e torceria pra não vir um 'NaN' ou 'undefined'. 
            // No Rust, a Trait 'FromStr' te obriga a lidar com o 'Result'. 
            // Ou o dado entra no Domínio (Ok), ou o erro é tratado na hora (Err).
            match Jogada::from_str(&input) {
                // (Early Return): cumpre o contrato da Trait e desaloca o buffer da String.
                Ok(jogada) => return jogada, 
                // Usamos nosso erro customizado do Módulo 3 para feedback visual.
                Err(erro_dominio) => println!("{}", erro_dominio),
            }
        }
    }
}
```

### Por que Rust é superior ao JavaScript aqui?

1. **Controle de Buffer:** No Node.js, você raramente controla quando o `stdout` é despejado. Se o evento de loop estiver ocupado, sua mensagem pode atrasar. No Rust, o `flush()` garante que o usuário veja a pergunta no exato ciclo de CPU que você definiu.
2. **Exaustividade vs. NaN:** Se você tentar converter uma string pra número no JS e falhar, você ganha um `NaN` (Not a Number) e o programa continua rodando em estado zumbi até explodir lá na frente. No Rust, o contrato `FromStr` te obriga a retornar um `Result` (Sum Type). O compilador **não te deixa ignorar o erro**.
3. **Zero Runtime Overhead:** No JS, cada import adiciona peso na árvore de objetos em tempo de execução. No Rust, as Traits são resolvidas via **Monomorfização (Monomorphization)** em tempo de compilação. O custo em runtime é literalmente **zero**.

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
// RACIOCÍNIO UNIFICADO (Akita Mode): 
// 'mod domain' não é um import de biblioteca, é uma Compiler Directive (Diretiva de Compilação). 
// No JS, o Webpack ou o Vite tentam resolver caminhos. Aqui, você está instruindo o Linker 
// do rustc a mapear os bits dos módulos no binário final. Sem isso, a pasta 'domain' 
// é apenas lixo no seu HD (Dead Code Elimination).
mod domain;

// 'use' é apenas Namespace Aliasing. Diferente do 'require' do Node.js, ele não toca no 
// sistema de arquivos em runtime. Ele só mapeia nomes na Tabela de Símbolos (Symbol Table) 
// em tempo de compilação. Zero overhead de processamento.
use domain::models::Resultado;
use domain::player::{Jogador, JogadorComputador, JogadorHumano};
use std::io::{self, Write};

fn main() {
    println!("=== BEM-VINDO AO JOKENPO EM RUST ===");
    
    // RACIOCÍNIO HUMANO: Injetamos as peças concretas (Dependency Injection).
    // RACIOCÍNIO DO COMPUTADOR: Como são structs vazias, o Rust não aloca nada na Heap. 
    // São apenas "marcos" pro compilador saber qual implementação de Trait chamar.
    let humano = JogadorHumano;
    let computador = JogadorComputador;

    // O "Big Loop" (Game Loop). No nível do processador, isso gera uma instrução 
    // incondicional JMP (Jump) de volta para o topo da função. 
    // Vantagem sobre o JS: No Node.js, um loop síncrono travaria o Event Loop. 
    // Aqui, temos controle total da thread principal do Sistema Operacional.
    loop {
        println!("\n--- Nova Rodada ---");
        
        // Aqui o polimorfismo brilha via Traits. O compilador sabe exatamente qual 
        // byte chamar para o humano e qual chamar para o bot (Static Dispatch).
        let jogada_humano = humano.escolher_jogada();
        let jogada_pc = computador.escolher_jogada();

        println!("Você escolheu: {:?}", jogada_humano);
        println!("O computador escolheu: {:?}", jogada_pc);

        // 'avaliar' recebe uma referência (&). Estamos passando apenas o endereço 
        // de memória (Pointer) da stack, sem mover a posse (Ownership). 
        // O custo de passar 8 bytes (um ponteiro) é desprezível.
        let resultado = jogada_humano.avaliar(&jogada_pc);
        
        // Pattern Matching: o compilador transforma isso numa Jump Table (Tabela de Saltos). 
        // Diferente de um 'switch' do JS que pode ser O(n), aqui é O(1). 
        // A CPU pula direto pro bloco de código correto baseado nos bits do Enum.
        match resultado {
            Resultado::Vitoria => println!("🎉 Você VENCEU a rodada!"),
            Resultado::Derrota => println!("💀 Você PERDEU a rodada!"),
            Resultado::Empate => println!("🤝 Deu EMPATE!"),
        }

        print!("\nDeseja jogar novamente? (s/n): ");
        // Stdout é bufferizado por linha. O 'flush()' é uma Syscall que força a 
        // CPU a despejar o buffer na tela antes do programa travar no I/O.
        io::stdout().flush().unwrap(); 
        
        let mut continuar = String::new();
        // Chamada de sistema para ler o stdin. Passamos a referência mutável (&mut) 
        // para o SO preencher a memória que já alocamos, evitando re-alocações inúteis.
        io::stdin().read_line(&mut continuar).unwrap();
        
        // Sanitização básica. 'trim()' e 'to_lowercase()' limpam o lixo do buffer.
        if continuar.trim().to_lowercase() != "s" {
            println!("Saindo... Obrigado por jogar!");
            break; // Quebra o fluxo (Control Flow) e encerra o processo (Exit Code 0).
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