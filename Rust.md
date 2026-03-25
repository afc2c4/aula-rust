Excelente que você está avançando para o próximo nível com seus alunos. O conceito de imutabilidade é a base da **segurança de memória** do Rust. Para um programador vindo do JavaScript ou Python, essa é a primeira "parede" onde o compilador mostra quem manda.

Aqui está o roteiro da aula, focando no raciocínio por trás do código:

---

## 1. Preparação do Ambiente

Para manter a organização, peça aos alunos para criarem um novo projeto usando o **Cargo** (o gerenciador de pacotes do Rust).

* **Onde escrever:** No terminal do VS Code ou do sistema.
* **Comando:**
    ```bash
    cargo new aula_imutabilidade
    cd aula_imutabilidade
    ```
* **Arquivo:** Abra o arquivo `src/main.rs`. É aqui que toda a mágica acontece.

---

## 2. O Raciocínio: Por que ser Imutável?

Antes de digitar, explique o **"Porquê"**:
Em Rust, o compilador é como um revisor de texto rigoroso. Se você diz que algo vale $5$, ele assume que será $5$ para sempre. Isso evita que uma função lá na frente mude o valor por acidente, causando bugs difíceis de encontrar em sistemas complexos.

### Passo A: Provocando o Erro
Vamos escrever o código que **não funciona** primeiro. Isso ajuda o aluno a ler a mensagem de erro do compilador.

No arquivo `src/main.rs`, apague o que estiver lá e digite:

```rust
fn main() {
    // 1. Declaramos a variável sem 'mut'
    let x = 5;
    println!("O valor de x é: {x}");

    // 2. Tentamos mudar o valor (Isso vai dar erro)
    x = 6; 
    println!("O valor de x agora é: {x}");
}
```

**O que está acontecendo aqui?**
* `let x = 5;`: Você criou uma "vaga" na memória e colocou o número 5. Como não usou `mut`, essa vaga foi "lacrada".
* `x = 6;`: Você tentou romper o lacre.



---

## 3. Testando e Lendo o Erro

Peça para os alunos executarem o comando:
* **Comando:** `cargo run`

O Rust não vai apenas dizer "Erro". Ele vai dizer exatamente:
`cannot assign twice to immutable variable x`. 
Ele até sugere: `consider making this binding mutable: mut x`.

---

## 4. O Raciocínio: Tornando a Variável Mutável

Agora, explicamos que a mutabilidade em Rust é uma **escolha consciente**. Você precisa pedir permissão ao compilador.

### Passo B: Aplicando a Solução
Vamos ajustar o código no mesmo arquivo `src/main.rs`:

```rust
fn main() {
    // Adicionamos a palavra-chave 'mut'
    // Raciocínio: "Compilador, eu pretendo alterar esse valor depois."
    let mut x = 5; 
    println!("O valor inicial de x é: {x}");

    // Agora a reatribuição é permitida
    x = 6; 
    println!("O valor alterado de x é: {x}");
}
```

**Por que isso é melhor que o JS?**
No JS, o padrão é ser mutável (com `let`). No Rust, o padrão é ser seguro (imutável). Se você esquecer um `mut` que não precisava, o Rust vai te dar um aviso (warning) dizendo que a variável não precisava ser mutável, ajudando você a manter o código limpo.

---

## 5. Resumo para os Alunos

| Conceito | Código | Comportamento |
| :--- | :--- | :--- |
| **Imutável (Padrão)** | `let x = 5;` | Valor fixo. Mais segurança e performance. |
| **Mutável** | `let mut x = 5;` | Permite alteração. Use apenas quando necessário. |

**Comando final para testar tudo:**
```bash
cargo run
```

---

Este é o conceito que separa os "aventureiros" dos programadores Rust profissionais. No JavaScript, o *Garbage Collector* limpa a bagunça para você. No Rust, o **Ownership** (Propriedade) garante que a memória seja limpa no momento exato em que não é mais necessária, sem pausar o programa.

Para o **Tauri**, isso é vital: quando o frontend envia um texto para o backend (Rust), quem "manda" naquele texto? Se o Rust processar e apagar, o frontend não pode mais pedir para ler.

Aqui está o roteiro para sua aula:

---

## 1. Preparação do Projeto

Vamos criar um novo projeto para isolar esse estudo.

* **Onde escrever:** No terminal.
* **Comando:**
    ```bash
    cargo new aula_ownership
    cd aula_ownership
    ```
* **Arquivo:** Abra o `src/main.rs`.

---

## 2. O Raciocínio: A Regra do "Dono Único"

Explique aos alunos: "Imagine que uma `String` é um livro físico valioso. Só pode haver **um** dono por vez. Se eu dou o livro para você, eu não o tenho mais."

### Passo A: Criando o "Objeto de Valor"
Diferente de números simples, a `String` vive na memória *Heap* (mais complexa). Vamos criar uma no `src/main.rs`:

```rust
fn main() {
    // Raciocínio: Criamos um dado na memória. 'usuario' é o dono atual.
    let usuario = String::from("Alexandre");
    
    println!("O dono atual é: {usuario}");
}
```

---

## 3. O Problema: A Transferência (Move)

Agora, vamos simular o que acontece no Tauri quando passamos um dado para uma função de processamento.

### Passo B: Criando a função de processamento
Adicione esta função **fora** do `main`:

```rust
fn processar_nome(nome: String) {
    println!("Processando o nome: {nome}");
} // <-- Aqui, 'nome' sai de escopo e o Rust APAGA a memória.
```

### Passo C: O erro de "Uso após Movimento"
Tente usar a variável no `main` após passá-la para a função:

```rust
fn main() {
    let usuario = String::from("Alexandre");

    // Raciocínio: Ao passar 'usuario', o Ownership é TRANSFERIDO.
    // 'usuario' não é mais o dono. O novo dono é o parâmetro 'nome' da função.
    processar_nome(usuario);

    // ERRO! Tentamos usar algo que não nos pertence mais.
    // println!("Tentando usar de novo: {usuario}"); 
}
```

---

## 4. Testando e Lendo o Erro

* **Comando:** `cargo run`

O compilador vai dar um erro clássico: `value borrowed here after move`.
Ele até desenha uma seta indicando onde o valor foi "movido" (`move occurs because usuario has type String`).

---

## 5. A Solução para o Tauri: Referências (Borrowing)

No Tauri, geralmente não queremos "perder" a variável, apenas "emprestá-la". Usamos o símbolo `&`.

### Passo D: O Código Corrigido (Empréstimo)
Ajuste a função e a chamada no `src/main.rs`:

```rust
// Raciocínio: Agora a função recebe um 'empréstimo' (&String)
fn processar_nome_melhorado(nome: &String) {
    println!("Processando o empréstimo: {nome}");
}

fn main() {
    let usuario = String::from("Alexandre");

    // Passamos apenas a referência (&). O Ownership continua com 'usuario'.
    processar_nome_melhorado(&usuario);

    // Agora funciona! O dono ainda tem o livro.
    println!("O dono ainda pode usar: {usuario}");
}
```

---

## Resumo para o Quadro Branco

| Situação | Ação | Consequência |
| :--- | :--- | :--- |
| `let x = s;` | **Move** | `s` morre, `x` vira o novo dono. |
| `let x = &s;` | **Borrow** | `s` continua dono, `x` apenas olha o valor. |
| **No Tauri** | `Command` | Ao receber dados do JS, o Rust vira o dono para garantir que o dado seja válido durante todo o processo. |

**Dica de Ouro:** Se os alunos perguntarem "Por que não clonar tudo?", explique que `.clone()` copia os dados na memória, o que é lento. Referências (`&`) são quase instantâneas.

---

Se prepare, porque este é o ponto onde muitos alunos de C++ ou Java começam a questionar a própria sanidade. Mas o raciocínio é lógico: **memória custa caro e o Rust quer que você saiba exatamente onde cada byte está.**

No JavaScript, uma string é só uma string. No Rust, precisamos diferenciar entre **quem é o dono do dado** e **quem está apenas olhando para ele**.

---

## 1. Preparação do Projeto

Vamos isolar esse "terror" em um projeto limpo.

* **Onde escrever:** No terminal.
* **Comando:**
    ```bash
    cargo new aula_strings
    cd aula_strings
    ```
* **Arquivo:** Abra o `src/main.rs`.

---

## 2. O Raciocínio: O Dono da Caixa vs. Quem olha pela Janela

Explique assim para os alunos:
* **`String` (A Caixa):** É um objeto que você possui. Você pode colocar mais coisas dentro, aumentar o tamanho ou destruir a caixa. Ela vive na memória *Heap*.
* **`&str` (A Janela):** É apenas uma visão (um *slice*). Você não é o dono da paisagem, você só está olhando para ela. É imutável e muito rápida.



---

## 3. Passo a Passo do Código

### Passo A: A String Estática (`&str`)
Vamos começar com o tipo mais simples, que é o texto "chumbado" no código.

No arquivo `src/main.rs`:

```rust
fn main() {
    // Raciocínio: Isso é um &str. O texto vive dentro do próprio binário.
    // É uma 'janela' para um texto que nunca muda.
    let mensagem_estatica: &str = "Olá, alunos!";
    
    println!("Estatica: {}", mensagem_estatica);
}
```

### Passo B: A String Dinâmica (`String`)
Agora, vamos criar algo que pode crescer, como se estivéssemos recebendo um input do usuário.

```rust
fn main() {
    let mensagem_estatica: &str = "Olá, alunos!";

    // Raciocínio: Transformamos a 'janela' em uma 'caixa' (String).
    // Agora a memória foi alocada e nós somos os donos.
    let mut mensagem_dinamica = mensagem_estatica.to_string();

    // Podemos modificar a String (precisa do 'mut' que vimos na aula 1)
    mensagem_dinamica.push_str(" Bem-vindos ao Rust.");

    println!("Dinamica: {}", mensagem_dinamica);
}
```

### Passo C: O Cenário do Tauri
No Tauri, o frontend (JS) envia dados via JSON. O Rust precisa **possuir** esses dados para garantir que eles não sumam enquanto o programa roda. Por isso, usamos `String`.

```rust
// Simulando um comando do Tauri
fn processar_input_do_js(nome: String) {
    // Raciocínio: Recebemos a 'caixa' inteira do JS.
    println!("Recebi do frontend: {}", nome);
}

fn main() {
    // ... código anterior ...

    let input_js = String::from("Alexandre");
    processar_input_do_js(input_js);
}
```

---

## 4. Testando o Código

* **Comando:** No terminal, dentro da pasta do projeto:
    ```bash
    cargo run
    ```

O Rust vai compilar e mostrar as duas versões. Se você tentar dar um `push_str` na `mensagem_estatica`, o compilador vai te dar um erro épico, explicando que `&str` não pode ser modificada.

---

## 5. Tabela Comparativa (Para o quadro)

| Característica | `String` (A Caixa) | `&str` (A Janela) |
| :--- | :--- | :--- |
| **Dono?** | Sim | Não (é um empréstimo) |
| **Pode crescer?** | Sim | Não |
| **Onde vive?** | Heap (Dinâmico) | Binário ou dentro de uma String |
| **Uso no Tauri** | Receber dados do JS | Apenas leitura rápida |

---

### Por que isso importa para você?
Se você tentar passar um `&str` para uma função que espera uma `String`, o Rust vai reclamar porque ele não quer "olhar pela janela" de algo que pode desaparecer; ele quer a "caixa" garantida.

---

O **Shadowing** (Sombreamento) é uma das "mordomias" que o Rust oferece para manter seu código limpo. Em linguagens como JavaScript ou Java, se você recebe um dado "sujo" e quer limpá-lo, acaba criando nomes como `nome`, `nomeFormatado`, `nomeTrimmed`. No Rust, você simplesmente reaproveita o nome.

Aqui está o roteiro para explicar isso aos seus alunos:

---

## 1. Preparação do Projeto

Vamos criar um espaço novo para testar esse conceito.

* **Onde escrever:** No terminal.
* **Comando:**
    ```bash
    cargo new aula_shadowing
    cd aula_shadowing
    ```
* **Arquivo:** Abra o `src/main.rs`.

---

## 2. O Raciocínio: "O Nome é o Mesmo, mas a Caixa é Nova"

Explique aos alunos: "Shadowing não é mudar o valor de uma variável (como o `mut`). É como se você jogasse a caixa antiga fora e colocasse uma nova no lugar, usando a mesma etiqueta."

### Passo A: O problema dos nomes feios
No arquivo `src/main.rs`, vamos escrever como faríamos em outras linguagens:

```rust
fn main() {
    // Raciocínio: Recebemos um input com espaços extras.
    let nome_com_espacos = "   João Silva   ";
    
    // Em outras linguagens, criaríamos um novo nome:
    let nome_limpo = nome_com_espacos.trim();

    println!("Nome: '{}'", nome_limpo);
}
```

### Passo B: Aplicando o Shadowing
Agora, vamos transformar isso no estilo Rust. O segredo é usar a palavra-chave `let` novamente.

```rust
fn main() {
    let nome = "   João Silva   ";

    // Raciocínio: "Não preciso mais da versão com espaços. 
    // Vou 'sombrear' a variável antiga com uma versão nova e limpa."
    let nome = nome.trim(); 

    println!("Nome final: '{}'", nome);
}
```

---

## 3. O "Superpoder": Mudança de Tipo

O Shadowing permite algo que o `mut` não permite: **mudar o tipo da variável**. Isso é o que mais explode a cabeça dos iniciantes.

### Passo C: De Texto para Número
Adicione isso ao seu `main.rs`:

```rust
fn main() {
    let entrada = "42"; // Isso é uma String (&str)

    // Raciocínio: Quero converter para número. 
    // Em vez de 'entrada_int', uso o Shadowing:
    let entrada: u32 = entrada.parse().expect("Não é um número!");

    println!("O número processado é: {}", entrada + 8); 
    // Note que agora podemos fazer contas, o que seria impossível com o texto "42".
}
```

---

## 4. Testando o Código

* **Comando:** No terminal:
    ```bash
    cargo run
    ```



---

## 5. Shadowing vs. Mutabilidade (Tabela de Aula)

Essa é a pergunta que os alunos certamente farão: "Qual a diferença?"

| Característica | `let mut x = ...` (Mutabilidade) | `let x = ...; let x = ...;` (Shadowing) |
| :--- | :--- | :--- |
| **O que muda?** | O valor dentro da caixa. | A própria caixa (é uma nova). |
| **Muda o tipo?** | **Não.** Se era String, continua String. | **Sim.** Pode virar número, booleano, etc. |
| **Imutabilidade** | A variável é mutável. | Ambas as instâncias continuam imutáveis. |

> **Dica para o Professor:** Use o Shadowing para transformações de dados (limpeza, conversão). Use o `mut` para contadores ou valores que precisam mudar de estado constantemente em um loop.

---

Chegamos ao "pulo do gato" para quem quer construir interfaces modernas com o **Tauri**. Aqui, o Rust deixa de ser apenas uma linguagem de sistema e passa a ser o "cérebro" por trás de uma interface em JavaScript/TypeScript.

O grande desafio aqui é entender como um dado viaja de um mundo (o navegador/frontend) para o outro (o binário Rust).

---

## 1. Localização: Onde a mágica acontece

Diferente dos exemplos anteriores onde usamos apenas o `src/main.rs` de um projeto comum, no Tauri o código do backend fica dentro de uma pasta específica.

* **Pasta:** `src-tauri/src/`
* **Arquivo:** `main.rs` (ou `lib.rs` em versões mais recentes do Tauri).

---

## 2. O Raciocínio: A "Ponte" entre JS e Rust

Imagine que o Frontend é o cliente de um restaurante (faz o pedido em JS) e o Rust é a cozinha. O `#[tauri::command]` é o **garçom**. Ele recebe o pedido, garante que os ingredientes (os tipos de dados) estão certos e leva para a cozinha.

### Passo A: Criando o Comando
No arquivo `src-tauri/src/main.rs`, vamos escrever o comando. Note que usamos um "atributo" (`#[tauri::command]`) para avisar ao Tauri que essa função deve ser visível para o JavaScript.

```rust
// 1. O atributo diz ao Tauri: "Exponha esta função para o Frontend"
#[tauri::command]
fn saudar(nome: String) -> String {
    // Raciocínio: O nome chega como uma 'String' completa. 
    // O Rust tomou posse desse dado que veio do JSON do navegador.
    
    // 2. Usamos a macro format! para criar uma NOVA String.
    // Perceba que não alteramos 'nome', nós criamos algo NOVO.
    let saudacao = format!("Olá, {}! Bem-vindo ao app.", nome);
    
    // 3. Retornamos a nova String para o Frontend
    saudacao
}
```

### Passo B: Registrando o Comando
Não basta criar a função; você precisa dizer ao sistema do Tauri que ela existe dentro da "central de controle" (`tauri::Builder`).

```rust
fn main() {
    tauri::Builder::default()
        // Raciocínio: Aqui listamos todos os comandos que o JS pode chamar.
        .invoke_handler(tauri::generate_handler![saudar])
        .run(tauri::generate_context!())
        .expect("erro ao rodar a aplicação Tauri");
}
```

---

## 3. Por que `String` e não `&str` aqui?

Esta é uma dúvida comum dos alunos. Por que o parâmetro é `String`?
* **O Raciocínio:** O dado vem de uma rede (uma chamada IPC - Inter-Process Communication). O Rust precisa alocar memória para guardar esse texto que veio do JSON. Como ele precisa garantir que esse dado exista enquanto a função rodar, ele cria uma `String` (dona do dado) em vez de apenas uma referência.



---

## 4. A Dica de Ouro: Imutabilidade no Desktop

Ao contrário de um script simples, um app desktop (Tauri) lida com **estado**. Se você tem uma variável global que muda toda hora (`mut`), e o usuário clica em três botões ao mesmo tempo, você pode gerar uma "condição de corrida" (race condition).

**O conselho para os alunos:**
> "Se você puder resolver o problema criando uma variável nova com **Shadowing** ou apenas retornando um valor novo (como fizemos no `format!`), faça isso. Deixe o `mut` apenas para quando for estritamente necessário, como um contador ou um buffer de dados pesado."

---

## 5. Como Testar

Para testar a integração, os alunos precisam rodar o ambiente de desenvolvimento do Tauri.

* **Comando no Terminal:**
    ```bash
    npm run tauri dev
    # ou
    cargo tauri dev
    ```

Para chamar esse código no **Frontend (App.js ou main.ts)**, eles usariam:
```javascript
import { invoke } from '@tauri-apps/api/core';

// Chamando o comando Rust
invoke('saudar', { nome: 'Alexandre' })
  .then((response) => console.log(response)); // "Olá, Alexandre!..."
```

---

### Resumo para a Aula

1.  **`#[tauri::command]`**: É a porta de entrada.
2.  **Ownership**: O Rust vira dono do dado enviado pelo JS para garantir segurança.
3.  **Imutabilidade**: Preferimos criar dados novos (`format!`) do que alterar os antigos, evitando bugs de interface travada ou dados corrompidos.

----

Essa é a parte onde o Rust "conversa" com o mundo exterior. Para um aluno que vem do JavaScript, a sintaxe de funções do Rust parece familiar, mas o **retorno implícito** e as **anotações (macros)** são onde eles costumam tropeçar.

Aqui está o roteiro detalhado para sua aula:

---

## 1. Localização do Código

Diferente de um script Rust puro, no Tauri o "cérebro" vive em uma pasta específica criada pelo CLI do Tauri.

* **Pasta:** `src-tauri/src/`
* **Arquivo:** `main.rs` (ou `lib.rs` dependendo da versão do Tauri).
* **Comando para Testar:** `npm run tauri dev` ou `cargo tauri dev`.

---

## 2. O Raciocínio: Anatomia de uma Função

Antes de transformar em um comando Tauri, os alunos precisam entender como o Rust declara funções. No Rust, somos **explícitos** sobre o que entra e o que sai.

### Passo A: A Função Básica
Vamos escrever uma função que apenas soma dois números.

```rust
// Raciocínio:
// 'fn' define a função.
// Parâmetros PRECISAM de tipo (ex: i32 para números inteiros).
// '-> i32' indica que a função promete retornar um inteiro.
fn somar(a: i32, b: i32) -> i32 {
    return a + b;
}
```

---

## 3. O "Pulo do Gato": Retorno Implícito

Aqui é onde você ganha os alunos pela elegância do Rust. No Rust, quase tudo é uma **expressão** (algo que resulta em um valor).

### Passo B: Removendo o `return` e o `;`
Explique que a última linha de uma função, se não tiver ponto e vírgula, é o que a função retorna automaticamente.

```rust
fn somar_elegante(a: i32, b: i32) -> i32 {
    // Raciocínio: Como não tem ';' no final, o Rust entende:
    // "O resultado desta conta é o que deve sair desta função".
    a + b 
}
```

> **Dica para os alunos:** Se você colocar um `;` no final de `a + b`, o Rust vai achar que você quer apenas executar a conta e não retornar nada, resultando em um erro de compilação!

---

## 4. Transformando em um Command do Tauri

Agora, como o JavaScript do frontend vai "enxergar" essa função? Usamos o atributo `#[tauri::command]`.

### Passo C: Preparando a Ponte


No seu `main.rs`, adicione a macro e registre a função:

```rust
// 1. A anotação que "abre a porta" para o frontend
#[tauri::command]
fn calcular_total(preco: f64, quantidade: i32) -> String {
    let total = preco * (quantidade as f64);
    
    // Raciocínio: Usamos format! para criar uma String. 
    // Sem o ponto e vírgula, ela é retornada para o JS como uma Promise.
    format!("O total da compra é: R$ {:.2}", total)
}

fn main() {
    tauri::Builder::default()
        // 2. REGISTRO: Você precisa dizer ao Tauri que essa função é um comando
        .invoke_handler(tauri::generate_handler![calcular_total])
        .run(tauri::generate_context!())
        .expect("erro ao rodar o app");
}
```

---

## 5. Como o Aluno testa no Frontend?

Para que eles vejam o resultado, mostre como o JavaScript chama isso. É uma ponte direta.

* **Arquivo:** `src/App.jsx` (ou similar no frontend).
* **Código JS:**
    ```javascript
    import { invoke } from '@tauri-apps/api/core';

    // Raciocínio: O nome da função no Rust vira o nome do comando no JS.
    // Os argumentos devem ser passados como um objeto.
    invoke('calcular_total', { preco: 29.90, quantidade: 3 })
      .then((mensagem) => console.log(mensagem)); 
      // Saída: "O total da compra é: R$ 89.70"
    ```

---

## Resumo para o Quadro

| Elemento | O que faz? | Exemplo |
| :--- | :--- | :--- |
| **`fn`** | Declara a função | `fn minha_funcao() { ... }` |
| **`-> Tipo`** | Define o retorno | `-> String` |
| **Sem `;`** | Retorno implícito | `total` (na última linha) |
| **`#[tauri::command]`** | Expõe ao JS | Colocado acima da `fn` |
| **`invoke_handler`** | Registra o comando | Dentro do `main` |

---

### Pergunta para a turma:
"Se eu colocar um ponto e vírgula na última linha de um `#[tauri::command]` que deveria retornar uma String, o que o JavaScript vai receber?" 
*(Resposta: Ele receberá `null` ou um erro, porque a função retornou `()` - vazio - em vez de um valor).*

---
Esta é a parte onde o Rust e o JavaScript finalmente começam a falar a mesma língua. Se as funções são os "correios", as **Structs** são as "caixas padronizadas" e o **Serde** é a "fita isolante" que garante que tudo chegue inteiro do outro lado.

Aqui está o roteiro para sua aula sobre como estruturar e trafegar dados complexos:

---

## 1. O Raciocínio: O Problema da Tradução

Explique aos alunos: "O JavaScript adora objetos literais `{ nome: "Alex", idade: 30 }`. O Rust é rigoroso e precisa de um molde fixo (**Struct**). O **Serde** (abreviação de **Ser**ialization/**De**serialization) é o tradutor que transforma o objeto JS em Rust e vice-versa."



---

## 2. Preparação: As Ferramentas (Cargo.toml)

Para usar o Serde no Tauri, precisamos avisar ao Rust que queremos os "superpoderes" dessa biblioteca.

* **Arquivo:** `src-tauri/Cargo.toml`
* **O que adicionar:** Procure a seção `[dependencies]` e garanta que estas linhas existam:

```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tauri = { version = "2.0", features = [] } # ou a versão que você estiver usando
```

---

## 3. Passo a Passo: Construindo a Estrutura

### Passo A: Definindo a Struct básica
No arquivo `src-tauri/src/main.rs`, vamos criar o nosso modelo de dados.

```rust
// Raciocínio: Criamos um molde para um 'Usuario'. 
// No Rust, usamos CamelCase para o nome da Struct.
struct Usuario {
    id: u32,
    nome: String,
    email: String,
    ativo: bool,
}
```

### Passo B: Adicionando o "Poder" de Serialização
Se você tentar enviar essa `struct` para o frontend agora, o Rust vai reclamar que não sabe como transformar isso em texto (JSON). Precisamos do `derive`.

```rust
use serde::{Serialize, Deserialize}; // Importamos os tradutores

// Raciocínio: O 'derive' gera automaticamente o código que 
// transforma essa struct em JSON (Serialize) e o JSON nela (Deserialize).
#[derive(Serialize, Deserialize)]
struct Usuario {
    id: u32,
    nome: String,
    email: String,
    ativo: bool,
}
```

---

## 4. O Comando Tauri com Objetos

Agora, vamos criar um comando que recebe um usuário e retorna um "perfil" formatado.

### Passo C: Implementando o Comando
Ainda no `src-tauri/src/main.rs`:

```rust
#[tauri::command]
fn criar_perfil(dados: Usuario) -> String {
    // Raciocínio: 'dados' já chega como uma instância da Struct Usuario.
    // O Tauri/Serde já fez todo o trabalho sujo de converter o JSON do JS para nós.
    
    if dados.ativo {
        format!("Perfil criado para {}: <{}> (ID: {})", dados.nome, dados.email, dados.id)
    } else {
        format!("O usuário {} está inativo.", dados.nome)
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![criar_perfil])
        .run(tauri::generate_context!())
        .expect("erro ao rodar o app");
}
```

---

## 5. Como Testar (Frontend e Backend)

* **No Terminal:** 

```bash
    npm run tauri dev
```

* **No Console do Navegador (Frontend):**
    Peça para os alunos rodarem este comando no console do DevTools para ver a mágica:

```javascript
// Raciocínio: O objeto JS deve ter as MESMAS chaves da Struct Rust.
const meuUsuario = {
  id: 101,
  nome: "Alexandre",
  email: "alex@email.com",
  ativo: true
};

window.__TAURI__.core.invoke('criar_perfil', { dados: meuUsuario })
  .then((res) => console.log("Resposta do Rust:", res));
```

---

## Resumo para o Quadro

| Termo | Função | Analogia |
| :--- | :--- | :--- |
| **Struct** | Define o formato do dado | O projeto de engenharia da caixa |
| **Serialize** | Transforma Rust → JSON | Desmontar a caixa para caber na carta |
| **Deserialize** | Transforma JSON → Rust | Montar a caixa ao receber a carta |
| **`#[derive(...)]`**| Gera o código automaticamente | Um robô que escreve o código chato por você |

---

### Dica para o Professor:
Mostre aos alunos o que acontece se o JavaScript enviar um campo errado (ex: `idade` em vez de `id`). O Rust vai recusar a chamada e o `invoke` vai cair direto no `.catch()`, provando que o Rust protege o backend de dados mal formatados vindo do frontend.

---

Para ensinar **Tipagem e Segurança de Memória**, você está entrando no "Santo Graal" do Rust. É aqui que explicamos por que o Rust não trava com o famoso *NullPointerException* ou erros de segmentação que assombram o C++.

Vamos construir o raciocínio de uma aplicação que gerencia o saldo de um usuário, aplicando cada conceito.

---

## 1. Preparação do Terreno

Peça aos alunos para resetarem o ambiente para um novo exemplo focado em segurança.

* **Onde escrever:** No terminal.
* **Comando:**
    ```bash
    cargo new aula_seguranca
    cd aula_seguranca
    ```
* **Arquivo:** `src/main.rs`.

---

## 2. O Raciocínio: O Contrato com o Compilador

Explique: "No JavaScript, o tipo é decidido na hora que o código roda. No Rust, o tipo é um **contrato assinado** antes de começar. Se você disse que é um número, o compilador garante que nunca será um texto por acidente."



### Passo A: Tipos Primitivos e Mutabilidade
Vamos começar definindo o saldo e o nome do usuário. No `src/main.rs`:

```rust
fn main() {
    // Raciocínio: 'nome' é String (texto complexo). 
    // 'saldo' é i32 (inteiro de 32 bits, o padrão para números).
    let nome: String = String::from("Alexandre");
    let mut saldo: i32 = 100; // Precisa ser 'mut' porque o saldo vai mudar!

    println!("Usuário: {} | Saldo Inicial: {}", nome, saldo);

    // Alterando o valor (Mutabilidade em ação)
    saldo = saldo + 50;
    println!("Novo Saldo: {}", saldo);
}
```

---

## 3. O Raciocínio: O "Dono" da Memória (Ownership)

Agora, o conceito que faz o Rust não precisar de um *Garbage Collector*. 

**A analogia da caneta:** "Se eu te empresto minha caneta, eu ainda sou o dono. Se eu te **dou** a caneta, eu não posso mais escrever com ela. O Rust rastreia quem é o dono de cada dado para saber quando 'jogar fora' a memória com segurança."



### Passo B: O Erro de Posse (Move)
Vamos criar uma função que "valida" o usuário, mas que acaba "roubando" a posse da variável.

```rust
fn validar_usuario(n: String) {
    println!("Validando o acesso de: {}", n);
} // <-- Aqui a variável 'n' é destruída da memória!

fn main() {
    let nome = String::from("Alexandre");
    let mut saldo = 100;

    // Raciocínio: Ao passar 'nome', a função validar_usuario se torna a DONA.
    validar_usuario(nome);

    // ERRO DE COMPILAÇÃO! 
    // println!("O usuário {} ainda está logado.", nome);
}
```

---

## 4. A Solução: Empréstimos (References)

Para resolver o erro acima sem gastar memória duplicando dados, usamos o **Empréstimo (&)**.

### Passo C: O Código Final e Seguro
Substitua o conteúdo de `src/main.rs` por este código completo, que mostra o raciocínio de segurança:

```rust
// Raciocínio: Recebemos um '&String' (uma referência), 
// ou seja, apenas pegamos emprestado para ler.
fn mostrar_perfil(n: &String, s: i32) {
    println!("--- RELATÓRIO ---");
    println!("Dono da conta: {}", n);
    println!("Saldo disponível: R${}", s);
}

fn main() {
    // 1. Tipagem e Mutabilidade
    let nome = String::from("Alexandre");
    let mut saldo = 500;

    // 2. Operação mutável
    saldo -= 100;

    // 3. Empréstimo (Borrowing)
    // Usamos '&nome' para que a função apenas olhe, mas não 'roube' a variável.
    mostrar_perfil(&nome, saldo);

    // 4. Como o nome foi apenas emprestado, ainda podemos usá-lo aqui!
    println!("Sessão encerrada para: {}", nome);
}
```

---

## 5. Como Testar e Validar

1.  **Comando:** No terminal, digite `cargo run`.
2.  **O que observar:** O código deve compilar instantaneamente.
3.  **Desafio para os alunos:** Peça para eles removerem o `&` de `&nome` na chamada da função e tentarem rodar de novo. O compilador vai dar uma aula de segurança de memória no terminal.

---

## Resumo para o Quadro

| Conceito | Regra de Ouro | Por que importa? |
| :--- | :--- | :--- |
| **Tipagem Estrita** | O tipo não muda após declarado. | Evita bugs de "NaN" ou operações inválidas. |
| **Mutabilidade** | Tudo é imutável por padrão (`let`). | Evita que dados mudem sem você perceber. |
| **Ownership** | Um valor, um único dono. | Garante que não haja vazamento de memória. |
| **Borrowing (&)** | Pegar emprestado em vez de tomar posse. | Permite reutilizar dados de forma ultra veloz. |

---

### Dica para o Professor:
Sempre reforce que o Rust não é "chato" por proibir coisas; ele é "protetor". Cada erro de compilação que o aluno vê agora é um erro que ele **não** verá o usuário final enfrentar com o app travando.

---

Esta é a aula que transforma um "codificador" em um **Engenheiro de Software**. No JavaScript, um erro pode "explodir" e travar a aplicação se você esquecer um `try/catch`. No Rust, o erro é um **valor** que você é obrigado a tratar.

No Tauri, isso é maravilhoso: se o Rust retorna um erro, o JavaScript já cai direto no `.catch()`.

---

## 1. O Raciocínio: O Fim das "Surpresas"

Explique aos alunos: "No Rust, não existem exceções invisíveis. Se uma função pode falhar (como ler um arquivo ou processar um pagamento), ela não retorna apenas um número ou texto; ela retorna um **envelope** chamado `Result`."

* **`Ok(valor)`**: O envelope contém o presente esperado.
* **`Err(erro)`**: O envelope contém uma nota explicativa do que deu errado.



---

## 2. Preparação e o "Bônus" de Tipagem Numérica

Como prometido, vamos usar `f64` (decimal) para dinheiro, mas com cuidado.

* **Onde escrever:** `src-tauri/src/main.rs`
* **Comando para testar:** `npm run tauri dev` ou `cargo tauri dev`

---

## 3. Passo a Passo do Código

### Passo A: Criando a Lógica de Erro
Primeiro, vamos definir o que pode dar errado. No Rust, o tipo de erro pode ser apenas uma `String`.

```rust
// Raciocínio: Esta função simula um saque. 
// Ela retorna um 'Result'. 
// Se der certo, retorna o novo saldo (f64). 
// Se der errado, retorna uma mensagem (String).
fn calcular_saque(saldo_atual: f64, valor_saque: f64) -> Result<f64, String> {
    if valor_saque <= 0.0 {
        // Raciocínio: Erro de lógica. Retornamos o 'envelope' de erro.
        return Err(String::from("O valor do saque deve ser maior que zero."));
    }

    if valor_saque > saldo_atual {
        // Raciocínio: Regra de negócio. Retornamos erro.
        return Err(String::from("Saldo insuficiente para esta operação."));
    }

    // Raciocínio: Tudo certo! Retornamos o 'envelope' de sucesso.
    Ok(saldo_atual - valor_saque)
}
```

### Passo B: Integrando com o Tauri Command
Agora, vamos expor isso para o frontend. O Tauri entende perfeitamente o `Result`.

```rust
#[tauri::command]
fn realizar_pagamento(saldo: f64, valor: f64) -> Result<String, String> {
    // Raciocínio: Chamamos nossa lógica e usamos o 'match' para decidir o que enviar ao JS.
    match calcular_saque(saldo, valor) {
        Ok(novo_saldo) => {
            // Se for Ok, o Tauri enviará um sinal de SUCESSO para a Promise do JS.
            Ok(format!("Sucesso! Seu novo saldo é R$ {:.2}", novo_saldo))
        },
        Err(mensagem_erro) => {
            // Se for Err, o Tauri enviará um sinal de ERRO para o .catch() do JS.
            Err(mensagem_erro)
        }
    }
}
```

---

## 4. Como testar no Frontend (A Ponte)

É aqui que os alunos veem o valor prático. Mostre este código JavaScript:

```javascript
import { invoke } from '@tauri-apps/api/core';

async function sacar() {
  try {
    // Raciocínio: O invoke tenta rodar o comando.
    const resposta = await invoke('realizar_pagamento', { saldo: 100.0, valor: 150.0 });
    console.log("Sucesso:", resposta); // Não vai chegar aqui se o saldo for baixo
  } catch (erro) {
    // Raciocínio: O 'Err' do Rust cai automaticamente aqui!
    console.error("Erro vindo do Rust:", erro); 
    alert("Operação negada: " + erro);
  }
}
```



---

## 5. Tabela de Comparação para o Quadro

| Característica | JavaScript (`try/catch`) | Rust (`Result`) |
| :--- | :--- | :--- |
| **Estilo** | Reativo (trata depois que explode) | Proativo (o tipo avisa que pode falhar) |
| **Segurança** | Você pode esquecer o `catch` | O compilador te obriga a tratar o `Result` |
| **Performance** | Lançar exceções é pesado | Retornar `Result` é tão rápido quanto um `if` |
| **No Tauri** | `Promise.reject` | `Err(mensagem)` |

---

## 6. Dica sobre Tipos Numéricos (`i32` vs `f64`)

Ao dar essa aula, mencione aos alunos:
* Use **`i32`** para contagens inteiras (quantidade de itens, IDs).
* Use **`f64`** para medidas e dinheiro simples (como fizemos aqui).
* **Atenção:** Em sistemas financeiros reais e críticos, o Rust tem *crates* (bibliotecas) específicas para decimais de alta precisão (como o `rust_decimal`), para evitar que `0.1 + 0.2` vire `0.30000000004`.

---

Pronto para a aula? Vamos fechar o ciclo do conhecimento básico de Rust para Tauri com chave de ouro. Agora, vamos unir a precisão dos números, o rigor dos erros com $Result<T, E>$ e a força do **Cargo**, que é quem mantém tudo isso de pé.

---

## 1. O Raciocínio: Números e Precisão ($i32$ vs $f64$)

Explique aos alunos: "No JavaScript, todo número é um `Number` (um float de 64 bits por baixo dos panos). No Rust, você escolhe a ferramenta certa para o trabalho. Se vai contar itens, use inteiros ($i32$); se vai medir algo ou lidar com moedas simples, use decimais ($f64$)."

### Passo A: O Problema da Imprecisão
No arquivo `src-tauri/src/main.rs`, vamos mostrar por que o Rust nos faz pensar sobre tipos:

```rust
fn main() {
    // Raciocínio: f64 é ótimo para cálculos científicos, 
    // mas tem "sujeira" binária em somas simples.
    let a: f64 = 0.1;
    let b: f64 = 0.2;
    
    // Isso pode resultar em 0.30000000000000004
    println!("Soma imprecisa: {}", a + b); 
}
```

---

## 2. Tratamento de Erros com $Result<T, E>$

Aqui está a maior diferença para o desenvolvedor Web: **Rust não "joga" exceções, ele as entrega na mão.**

### Passo B: Construindo um Comando Seguro
Vamos criar um comando que valida um pagamento. Ele deve retornar um erro se o saldo for insuficiente.

**Onde escrever:** `src-tauri/src/main.rs`

```rust
// 1. Definimos o que a função retorna:
// Se der certo: Uma String de sucesso.
// Se der errado: Uma String com a mensagem de erro.
#[tauri::command]
fn processar_pagamento(valor: f64, saldo: f64) -> Result<String, String> {
    // Raciocínio: Antes de fazer a conta, validamos a regra de negócio.
    if valor > saldo {
        // Retornamos o 'envelope' de erro
        return Err(String::from("Saldo insuficiente para esta transação!"));
    }

    if valor <= 0.0 {
        return Err(String::from("O valor deve ser positivo."));
    }

    // Se passou pelas guardas, retornamos o 'envelope' de sucesso
    let novo_saldo = saldo - valor;
    Ok(format!("Pagamento de R$ {:.2} realizado! Novo saldo: R$ {:.2}", valor, novo_saldo))
}
```

**Por que isso é incrível no Tauri?**
Quando o JavaScript chama esse comando usando o `invoke`, o Tauri olha para o $Result$. Se for `Ok`, ele resolve a **Promise** (`.then()`). Se for `Err`, ele rejeita a **Promise** (`.catch()`).

---

## 3. O Ecossistema Cargo: A Central de Comando

Explique que o **Cargo** é o equivalente ao `npm`, mas muito mais poderoso, pois ele compila o código além de gerenciar pacotes.

### Passo C: Adicionando uma Biblioteca (Crate)
Se os alunos quiserem resolver o problema da imprecisão do $f64$ em sistemas financeiros, eles usariam uma "Crate".

* **Onde escrever:** No terminal, dentro da pasta `src-tauri`.
* **Comando:** ```bash
    cargo add rust_decimal
    ```
    * *Raciocínio:* O Cargo vai baixar a biblioteca, verificar a segurança e atualizar o seu arquivo `Cargo.toml` automaticamente.

### Passo D: Os Comandos Essenciais de Aula
Ensine esses três pilares para os alunos:

1.  **Desenvolvimento:** `cargo tauri dev` (Inicia o backend em Rust e o frontend em JS com Hot Reload).
2.  **Checagem Rápida:** `cargo check` (Verifica se o código tem erros sem perder tempo compilando tudo).
3.  **Produção:** `cargo tauri build` (O comando final que gera o `.exe`, `.dmg` ou `.deb` para o usuário).

---

## 4. Resumo para os Alunos (Tabela de Estudo)

| Ferramenta / Conceito | Função | Equivalente no JS |
| :--- | :--- | :--- |
| **Cargo** | Gerenciador e Compilador | `npm` + `webpack/vite` |
| **$Result<T, E>$** | Tratamento de Erro Explícito | `try { ... } catch { ... }` |
| **$i32$ vs $f64$** | Inteiro vs Decimal | Tudo é `Number` |
| **`Cargo.toml`** | Manifesto do Projeto | `package.json` |



---

## 5. Como Testar Tudo

Para garantir que a integração com o frontend está funcionando:

1.  No terminal: `npm run tauri dev`.
2.  No console do navegador (DevTools), peça para eles testarem o erro:

```javascript
// Testando o erro (Saldo insuficiente)
window.__TAURI__.core.invoke('processar_pagamento', { valor: 500, saldo: 100 })
  .then(msg => console.log(msg))
  .catch(err => console.error("O Rust barrou:", err)); // Vai cair aqui!
```

---

Excelente. Este exercício foi desenhado para ser feito em **60 minutos**. Ele força o aluno a usar todos os pilares que vimos: **Structs**, **Ownership**, **Strings**, **Result** e a **Integração com o Frontend**.

---

## 🚀 Desafio: "O Filtro de Tarefas Seguro"

**Objetivo:** Criar um comando no Rust que recebe uma tarefa do frontend, limpa o texto (Shadowing/Strings), valida se ela não está vazia (Result) e retorna um objeto formatado (Struct/Serde).

---

### 1. Preparação (Cargo)

Peça aos alunos para abrirem o terminal na pasta do projeto Tauri.

* **Onde:** Terminal.
* **Comando:**
    ```bash
    # Garanta que as dependências de serialização estão lá
    cargo add serde --features derive
    ```

---

### 2. O Molde dos Dados (Structs)

Abra o arquivo `src-tauri/src/main.rs`. Vamos definir como é uma "Tarefa" no nosso sistema.

**Raciocínio:** No JS, uma tarefa é qualquer objeto. No Rust, precisamos de um contrato fixo para o compilador garantir que não faltará nenhum campo.

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Tarefa {
    id: i32,
    titulo: String,
    concluida: bool,
}
```


---

### 3. A Lógica de Negócio (Result & Strings)

Ainda no `main.rs`, vamos criar a função que processa essa tarefa.

**Raciocínio:** Queremos evitar que o usuário salve tarefas apenas com espaços em branco. Vamos usar **Shadowing** para limpar o texto e **Result** para avisar o erro.

```rust
fn processar_texto_tarefa(input: String) -> Result<String, String> {
    // 1. Shadowing para limpar espaços (Strings & Shadowing)
    let input = input.trim();

    // 2. Validação (Result)
    if input.is_empty() {
        return Err(String::from("O título da tarefa não pode estar vazio!"));
    }

    if input.len() < 3 {
        return Err(String::from("Tarefa muito curta! Mínimo 3 caracteres."));
    }

    // 3. Sucesso
    Ok(input.to_string())
}
```

---

### 4. A Ponte com o Frontend (Tauri Command)

Agora, vamos unir tudo em um comando que o JavaScript possa chamar.

**Raciocínio:** O comando recebe a `Tarefa` inteira, usa nossa lógica de validação e devolve a tarefa "limpa" para o frontend.

```rust
#[tauri::command]
fn salvar_tarefa(mut tarefa: Tarefa) -> Result<Tarefa, String> {
    // Raciocínio: Chamamos a validação usando o título que veio na struct
    // O '?' é um atalho do Rust: se der erro, ele retorna o erro na hora.
    let titulo_limpo = processar_texto_tarefa(tarefa.titulo)?;

    // Atualizamos a struct com o texto limpo
    tarefa.titulo = titulo_limpo;

    // Retornamos a struct inteira de volta para o JS
    Ok(tarefa)
}
```


---

### 5. Registro e Teste Final

Não esqueça de registrar o comando no `main`:

```rust
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![salvar_tarefa])
        .run(tauri::generate_context!())
        .expect("erro ao rodar o app");
}
```

---

### Como os alunos testam?

Peça para eles abrirem o console do navegador no app rodando (`npm run tauri dev`) e testarem os dois cenários:

**Cenário A: Erro (Título vazio)**
```javascript
window.__TAURI__.core.invoke('salvar_tarefa', { 
    tarefa: { id: 1, titulo: "   ", concluida: false } 
})
.catch(err => console.error("Barreado pelo Rust:", err));
```

**Cenário B: Sucesso (Título com espaços)**
```javascript
window.__TAURI__.core.invoke('salvar_tarefa', { 
    tarefa: { id: 1, titulo: "   Estudar Rust   ", concluida: false } 
})
.then(res => console.log("Tarefa Salva e Limpa:", res));
```

---

### Checklist para o Professor (O que observar na sala):

1.  **O ponto e vírgula:** Alunos vindo do JS vão colocar `;` no final do `Ok(...)` ou `Err(...)` dentro das funções. Lembre-os que o retorno é implícito!
2.  **O Atributo Derive:** Verifique se eles não esqueceram o `#[derive(Serialize, Deserialize)]` em cima da Struct, ou o `invoke` vai falhar silenciosamente.
3.  **Snake_case vs CamelCase:** No Rust usamos `salvar_tarefa`. No JS, ao chamar o `invoke`, o nome deve ser exatamente igual ao do Rust (a menos que use renomeação).




