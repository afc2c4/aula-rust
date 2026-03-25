Fala, mestre! Preparar uma aula de **Tauri** é excelente porque ele resolve a maior "dor" do Electron: o consumo excessivo de memória. No ambiente de Codespaces (que é baseado em Linux), o setup exige alguns cuidados específicos com dependências de sistema.

Aqui está o roteiro e a execução da sua aula, focada no raciocínio arquitetural e na prática inicial do **To-Do List**.

---

## 1. O Raciocínio: Por que Tauri?

O primeiro ponto que seus alunos precisam entender é a **Arquitetura de WebView Nativa**.

* **Electron:** Empacota um navegador inteiro (Chromium) e o Node.js dentro de cada app. É como se, para entregar uma pizza, você entregasse o pizzaiolo e o forno junto.
* **Tauri:** Ele usa o que o sistema já tem. No Windows ele usa o WebView2 (Edge), no macOS o WebKit (Safari) e no Linux o WebKitGTK. Ele substitui o Node.js pelo **Rust**, o que garante muito mais segurança e performance.

### O Modelo de Processos
Imagine o app como um banco:
1.  **Processo Core (Rust):** É o "Gerente" e o "Cofre". Ele tem acesso total ao sistema de arquivos, rede e hardware.
2.  **Processo WebView (Frontend):** É o "Guichê de Atendimento". Ele só mostra a interface e não pode fazer nada perigoso sem pedir permissão ao Gerente (via comandos IPC).



---

## 2. Configurando o Codespace (Setup)

Como o Codespace roda em um container Linux, precisamos instalar as bibliotecas de desenvolvimento que o Rust usará para "conversar" com a interface gráfica.

**No terminal do seu Codespace, execute:**

```bash
sudo apt-get update
sudo apt-get install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf \
    build-essential \
    curl \
    wget \
    file \
    libssl-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev
```

> **Aviso de Reinicialização:** Você **não** precisa reiniciar a sandbox inteira após esses comandos, mas se o Rust não reconhecer o comando `cargo`, basta abrir um novo terminal.

---

## 3. Criando o Projeto: O Nascimento do "Tauri-Todo"

O raciocínio aqui é usar o `create-tauri-app`, que funciona como um "scaffolding" (andaime). Ele já prepara as duas metades do cérebro da nossa aplicação.

**Comando:**
```bash
npx create-tauri-app@latest
```

**Escolhas sugeridas para a aula:**
* Project name: `tauri-todo`
* Frontend language: `TypeScript / JavaScript`
* Package manager: `npm`
* UI Template: `Vanilla` (Para focar no Tauri e não se perder em frameworks agora).
* UI Flavor: `TypeScript`

### Entendendo a Estrutura de Pastas
Após a criação, entre na pasta: `cd tauri-todo`.

* **`/src`**: Aqui mora o seu HTML/CSS/TS. É um site comum.
* **`/src-tauri`**: Aqui está o "motor".
    * `Cargo.toml`: Gerenciador de pacotes do Rust.
    * `tauri.conf.json`: Onde configuramos permissões e janelas.
    * `src/main.rs`: O ponto de entrada do backend.

---

## 4. Mão na Massa: O To-Do List (Início)

Vamos começar configurando a interface mínima e o comando inicial.

### Passo 1: O Frontend (Interface)
Abra o arquivo `index.html` na raiz da pasta `src` e substitua o conteúdo para o nosso To-Do:

**Local:** `src/index.html`
```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Tauri Todo</title>
    <style>
      body { font-family: sans-serif; display: flex; flex-direction: column; align-items: center; }
      #todo-list { list-style: none; padding: 0; }
      .input-container { margin-bottom: 20px; }
    </style>
  </head>
  <body>
    <h1>Minha Lista de Tarefas</h1>
    <div class="input-container">
      <input type="text" id="todo-input" placeholder="Nova tarefa..." />
      <button id="add-btn">Adicionar</button>
    </div>
    <ul id="todo-list"></ul>

    <script type="module" src="/main.ts"></script>
  </body>
</html>
```

### Passo 2: O Raciocínio do Backend (Rust)
Muitas vezes, queremos que o Rust processe algo (ex: salvar a tarefa num banco de dados). Vamos criar um comando simples no Rust que apenas confirma o recebimento da tarefa.

**Local:** `src-tauri/src/main.rs`
Substitua o conteúdo para:

```rust
// Prevê que a janela do console não apareça no Windows em modo release
#![cfg_寛all(not(debug_assertions), target_os = "windows")]

// Esse é o comando que o Frontend vai chamar
#[tauri::command]
fn confirmar_tarefa(name: &str) -> String {
    format!("Tarefa '{}' recebida pelo Rust com sucesso!", name)
}

fn main() {
    tauri::Builder::default()
        // Registramos o comando aqui para o Tauri saber que ele existe
        .invoke_handler(tauri::generate_handler![confirmar_tarefa])
        .run(tauri::generate_context!())
        .expect("erro ao rodar a aplicação");
}
```

### Passo 3: Conectando os dois mundos
Agora, no `src/main.ts`, vamos fazer o TypeScript chamar o Rust.

**Local:** `src/main.ts`
```typescript
import { invoke } from "@tauri-apps/api/tauri";

const inputEl = document.querySelector("#todo-input") as HTMLInputElement;
const btnEl = document.querySelector("#add-btn");
const listEl = document.querySelector("#todo-list");

btnEl?.addEventListener("click", async () => {
  const task = inputEl.value;
  
  // Chamando o comando 'confirmar_tarefa' do Rust
  const response = await invoke<string>("confirmar_tarefa", { name: task });
  
  // Adicionando na tela
  const li = document.createElement("li");
  li.textContent = `${task} - (${response})`;
  listEl?.appendChild(li);
  
  inputEl.value = "";
});
```

---

## 5. Testando a Aplicação

Para rodar, usamos o CLI do Tauri. No terminal:

```bash
npm run tauri dev
```

**Nota sobre o Codespace:** O Tauri tentará abrir uma janela GUI. Como o Codespace é via browser, ele não vai conseguir "desenhar" a janela na sua tela local a menos que você esteja usando a extensão **"Remote Desktop"** ou tenha um servidor X11. No entanto, o comando acima irá compilar o backend Rust e o frontend Vite, verificando se toda a sua estrutura está correta.

---
Vamos dar o próximo passo nessa jornada. O **Módulo 2** é fundamental porque é aqui que os alunos percebem que todo o conhecimento prévio que possuem de **Web** (HTML, CSS, React, Vue, etc.) é 100% reaproveitável no Tauri.

---~

# Módulo 2

## 1. O Raciocínio: A "Casca" e o "Motor"

Imagine que o Tauri é uma moldura de quadro inteligente. 
* A **tela (Frontend)** pode ser pintada com qualquer técnica: tinta a óleo (React), aquarela (Svelte) ou apenas lápis (Vanilla JS). O Tauri não se importa, ele apenas exibe o que você colocar na pasta `src`.
* O **HMR (Hot Module Replacement)** é o que torna a experiência mágica: você altera uma cor no CSS e, instantaneamente, a janela do app (que é um processo nativo) se atualiza sem você precisar recompilar o código Rust, que é a parte mais pesada.



---

## 2. Preparando o Ambiente (Codespace)

Se você fechou e abriu o Codespace, certifique-se de que as dependências do sistema ainda estão lá. **Não precisa reiniciar a sandbox**, mas execute o comando abaixo caso sinta falta de alguma biblioteca:

```bash
# Apenas se as dependências do Módulo 1 não estiverem instaladas
sudo apt-get update && sudo apt-get install -y libgtk-3-dev libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf build-essential curl wget file libssl-dev libayatana-appindicator3-dev
```

Certifique-se de estar na pasta do projeto:
```bash
cd tauri-todo
```

---

## 3. Evoluindo o To-Do List: Frontend e Debugging

Vamos transformar nossa lista simples em algo mais profissional e aprender a debugar o que acontece no "meio do caminho" entre o JS e o Rust.

### Passo 1: Estilização (O "Look and Feel" de App)
O raciocínio aqui é fazer com que a Web pareça um software nativo.

**Local:** `src/style.css` (Crie este arquivo se não existir ou limpe o atual)
```css
:root {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background-color: #f4f4f9;
  color: #333;
}

h1 { color: #2d3748; }

.container {
  padding: 2rem;
  max-width: 400px;
  margin: 0 auto;
}

input {
  padding: 10px;
  border: 1px solid #cbd5e0;
  border-radius: 4px;
  width: 70%;
}

button {
  padding: 10px 15px;
  background-color: #4a90e2;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

button:hover { background-color: #357abd; }

ul { list-style: none; padding: 0; margin-top: 20px; }

li {
  background: white;
  margin-bottom: 8px;
  padding: 10px;
  border-radius: 4px;
  display: flex;
  justify-content: space-between;
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
}

.delete-btn {
  background-color: #e53e3e;
  padding: 5px 10px;
  font-size: 12px;
}
```

### Passo 2: Estrutura Agnóstica
Note que o HTML não tem nada "especial" do Tauri. É puramente Web.

**Local:** `src/index.html`
```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8" />
  <link rel="stylesheet" href="./style.css">
  <title>Tauri Todo Pro</title>
</head>
<body>
  <div class="container">
    <h1>Tauri Todo</h1>
    <div class="input-group">
      <input type="text" id="todo-input" placeholder="O que precisa ser feito?" />
      <button id="add-btn">Add</button>
    </div>
    <ul id="todo-list"></ul>
  </div>
  <script type="module" src="/main.ts"></script>
</body>
</html>
```

### Passo 3: Debugging e Lógica de Eventos
Aqui vamos aplicar o **debugging**. No Tauri, podemos ver mensagens no console do navegador (Frontend) e no terminal (Backend/Rust).

**Local:** `src/main.ts`
```typescript
import { invoke } from "@tauri-apps/api/tauri";

const inputEl = document.querySelector("#todo-input") as HTMLInputElement;
const btnEl = document.querySelector("#add-btn");
const listEl = document.querySelector("#todo-list");

// Função para adicionar tarefa
async function addTask() {
  const task = inputEl.value;
  if (!task) return;

  // DEBUG FRONTEND: Ver no console do navegador (Inspect Element)
  console.log("Tentando adicionar tarefa:", task);

  try {
    // Chamamos o comando no Rust
    const response = await invoke<string>("log_tarefa", { msg: `Usuário criou: ${task}` });
    
    // DEBUG BACKEND: O Rust nos devolve uma confirmação
    console.log("Resposta do Rust:", response);

    const li = document.createElement("li");
    li.innerHTML = `
      <span>${task}</span>
      <button class="delete-btn">X</button>
    `;

    // Lógica de deleção
    li.querySelector(".delete-btn")?.addEventListener("click", () => li.remove());

    listEl?.appendChild(li);
    inputEl.value = "";
    inputEl.focus();
  } catch (err) {
    console.error("Erro ao comunicar com o Core:", err);
  }
}

btnEl?.addEventListener("click", addTask);

// Atalho de teclado (Enter)
inputEl.addEventListener("keypress", (e) => {
  if (e.key === "Enter") addTask();
});
```

### Passo 4: O Console do Rust (O lado escuro da lua)
Para depurar no Backend, usamos macros do Rust como `println!`. Isso aparecerá direto no seu terminal do Codespace.

**Local:** `src-tauri/src/main.rs`
```rust
#![cfg_attr(not(debug_assertions), target_os = "windows")]

#[tauri::command]
fn log_tarefa(msg: String) -> String {
    // Esse print aparece no seu TERMINAL (Console do Rust)
    println!("LOG DO SISTEMA: {}", msg);
    
    // Retornamos algo para o Frontend saber que funcionou
    format!("Rust recebeu seu log às!")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![log_tarefa])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## 4. Testando e Debugando no Codespace

Para ver o HMR e o Debugging em ação, execute:

```bash
npm run tauri dev
```

### Como seus alunos devem debugar:
1.  **No Terminal:** Eles verão a mensagem `LOG DO SISTEMA: Usuário criou: [Tarefa]`. Isso prova que o Frontend conversou com o binário Rust.
2.  **Na Interface (Webview):** Se você estiver usando um ambiente com interface gráfica no Codespace (ou rodando localmente), clique com o botão direito em qualquer lugar do app e selecione **"Inspecionar Elemento"**.
    * Vá na aba **Console**.
    * Lá você verá as mensagens do `console.log` que colocamos no `main.ts`.

> **Dica de Aula:** Mostre aos alunos que, se eles mudarem a cor do botão no `style.css` e salvarem, o app se atualiza sem que o processo do Rust (que aparece no terminal) precise reiniciar. Isso é o **HMR**.

---

# Módulo 3

O **Módulo 3** é o coração da segurança do Tauri. Se o Electron é um "vidro aberto", o Tauri é um "cofre com um guichê".

---

## 1. O Raciocínio: O Guichê de Segurança (IPC)

No desenvolvimento desktop moderno, você nunca quer que sua interface (o JavaScript) tenha acesso direto ao seu disco rígido ou à sua rede. Por quê? Porque se um hacker conseguir injetar um script malicioso no seu frontend, ele teria controle total sobre o computador do usuário.

O **IPC (Inter-Process Communication)** do Tauri funciona como um guichê de banco:
1.  **O Frontend (Cliente):** Preenche um formulário (JSON) pedindo: "Ei, salve esta tarefa para mim".
2.  **A Ponte (Tauri):** Leva esse formulário até o Rust.
3.  **O Backend (Gerente em Rust):** Lê o formulário, verifica se os dados são seguros e executa a ação no sistema operacional.



---

## 2. Preparando o Ambiente

Como estamos no **Codespace**, as ferramentas já foram instaladas nos módulos anteriores. Caso você tenha acabado de abrir a sandbox agora, certifique-se de estar na pasta do projeto e que o Node.js e o Rust estão prontos:

```bash
cd tauri-todo
# Verifique se o Rust está ok
rustc --version
# Verifique se o Node está ok
node -v
```
*(Não é necessário reiniciar a sandbox se as ferramentas já foram instaladas anteriormente).*

---

## 3. Construindo a Comunicação Segura

Vamos evoluir nosso To-Do List para que o Rust não apenas "receba um log", mas gerencie a criação de objetos de tarefa com IDs únicos.

### Passo 1: Definindo o Modelo no Rust (O "Contrato")
Para que o Rust e o JS falem a mesma língua, usamos uma biblioteca chamada `serde` (Serialization/Deserialization). Ela transforma objetos JSON em estruturas do Rust e vice-versa.

**Local:** `src-tauri/src/main.rs`
Substitua o código para refletir essa estrutura:

```rust
// Importamos o Serde para que a estrutura possa ser "traduzida" para JSON
use serde::{Serialize, Deserialize};

// Definimos como é uma Tarefa no nosso sistema
#[derive(Serialize, Deserialize)]
struct Tarefa {
    id: i32,
    texto: String,
    concluida: bool,
}

// O comando IPC: Ele recebe o texto e retorna um objeto Tarefa completo
#[tauri::command]
fn criar_tarefa_backend(texto: String) -> Tarefa {
    println!("Backend: Criando tarefa para o texto: {}", texto);
    
    // Raciocínio: Aqui o Rust poderia salvar em um banco de dados SQL ou JSON.
    // Por enquanto, vamos simular a criação de um ID único.
    Tarefa {
        id: rand::random::<i32>().abs(), // Gera um ID aleatório positivo
        texto: texto,
        concluida: false,
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![criar_tarefa_backend])
        .run(tauri::generate_context!())
        .expect("erro ao rodar a aplicação");
}
```

> **Nota:** Se o compilador reclamar do `rand`, adicione `rand = "0.8"` no arquivo `src-tauri/Cargo.toml` em `[dependencies]`.

### Passo 2: O Raciocínio do Frontend (Chamada Assíncrona)
O JavaScript não sabe quanto tempo o Rust vai levar para processar a informação. Por isso, toda chamada `invoke` é uma **Promise**. Precisamos lidar com isso usando `async/await`.

**Local:** `src/main.ts`
Vamos atualizar a lógica para receber o objeto tarefa do Rust:

```typescript
import { invoke } from "@tauri-apps/api/tauri";

// Definimos uma interface para o TypeScript entender o que o Rust devolve
interface Tarefa {
  id: number;
  texto: string;
  concluida: boolean;
}

const inputEl = document.querySelector("#todo-input") as HTMLInputElement;
const btnEl = document.querySelector("#add-btn");
const listEl = document.querySelector("#todo-list");

async function handleAddTask() {
  const texto = inputEl.value;
  if (!texto) return;

  try {
    // Raciocínio: Enviamos apenas o texto. 
    // O Rust decide o ID e o estado inicial (Segurança!).
    const novaTarefa = await invoke<Tarefa>("criar_tarefa_backend", { texto });

    renderizarTarefa(novaTarefa);
    
    inputEl.value = "";
    inputEl.focus();
  } catch (error) {
    console.error("Falha na comunicação IPC:", error);
  }
}

function renderizarTarefa(tarefa: Tarefa) {
  const li = document.createElement("li");
  li.id = `task-${tarefa.id}`;
  li.innerHTML = `
    <span>${tarefa.texto}</span>
    <small>ID: ${tarefa.id}</small>
    <button class="delete-btn">Excluir</button>
  `;
  
  listEl?.appendChild(li);
}

btnEl?.addEventListener("click", handleAddTask);
```

---

## 4. Testando a Comunicação

Agora, vamos rodar o comando para ver a mágica acontecer. O Rust vai compilar o novo comando e o TypeScript vai se conectar a ele.

**Comando no terminal:**
```bash
npm run tauri dev
```

### O que observar durante o teste:
1.  **No Terminal do Codespace:** Toda vez que você clicar em "Add", o Rust imprimirá: `Backend: Criando tarefa para o texto: ...`. Isso confirma que a mensagem atravessou a ponte IPC.
2.  **Na Interface:** Você verá que o ID exibido ao lado da tarefa é um número aleatório gerado pelo Rust. Isso prova que o Frontend recebeu dados estruturados de volta do backend.

---

## Resumo da Aula para seus Alunos:
* **Isolamento:** O Frontend só conhece o nome do comando (`criar_tarefa_backend`). Ele não sabe como o Rust cria o ID ou onde ele salva.
* **Tipagem:** Usamos `struct` no Rust e `interface` no TS para garantir que os dois lados falem a mesma língua sem erros de campo faltando.
* **Assincronismo:** A interface nunca trava enquanto o Rust processa, graças ao modelo de Promises do `invoke`.

---
Perfeito! Vamos focar agora na "mágica" que faz o Tauri ser tão poderoso: o **sistema de comandos**. No Módulo 3, o objetivo é transformar uma função comum do Rust em um "ponto de extremidade" (endpoint) que o seu JavaScript consegue chamar como se fosse uma API.

---

## 1. O Raciocínio: Transformando Rust em uma "API Interna"

Por padrão, o Rust é um mundo isolado do navegador. Para que eles se falem, o Tauri usa **Macros**. 

* **`#[tauri::command]`**: Essa anotação é um sinalizador. Ela diz ao compilador: "Ei, prepare esta função para que ela possa ser serializada e enviada através da ponte IPC (Inter-Process Communication)".
* **O Registro**: Não basta criar a função; o "Coração" do app (o Builder) precisa saber que ela existe para permitir o acesso. É como dar permissão de firewall para uma porta específica.
* **A Invocação**: No JavaScript, usamos o `invoke`. Ele sempre retorna uma **Promise**, pois a comunicação entre processos é inerentemente assíncrona.



---

## 2. Passo a Passo: Criando e Invocando o Comando

Vamos evoluir nosso To-Do List criando uma funcionalidade de "Validação de Tarefa" no lado do Rust.

### Passo 1: O Lado do Rust (O Backend)
Vamos escrever a função que valida se uma tarefa é muito curta e retorna uma mensagem personalizada.

**Arquivo:** `src-tauri/src/main.rs`
**Pasta:** `src-tauri/src/`

```rust
// 1. Criamos a função com a anotação necessária
#[tauri::command]
fn validar_tarefa(texto: String) -> Result<String, String> {
    // Raciocínio: O Rust é excelente para lógica de regras de negócio.
    // Aqui verificamos se o texto é vazio ou muito curto.
    if texto.trim().is_empty() {
        return Err("A tarefa não pode estar vazia!".into());
    }

    if texto.len() < 3 {
        return Err("Tarefa muito curta! Use pelo menos 3 caracteres.".into());
    }

    // Se estiver tudo ok, retornamos um Ok
    Ok(format!("Tarefa '{}' validada com sucesso pelo Rust!", texto))
}

fn main() {
    tauri::Builder::default()
        // 2. O REGISTRO: Aqui "abrimos a porta" para o comando
        .invoke_handler(tauri::generate_handler![validar_tarefa])
        .run(tauri::generate_context!())
        .expect("erro ao rodar a aplicação");
}
```

### Passo 2: O Lado do JavaScript (O Frontend)
Agora, vamos modificar o nosso arquivo TypeScript para chamar essa validação antes de adicionar a tarefa na lista.

**Arquivo:** `src/main.ts`
**Pasta:** `src/`

```typescript
import { invoke } from "@tauri-apps/api/tauri";

const inputEl = document.querySelector("#todo-input") as HTMLInputElement;
const btnEl = document.querySelector("#add-btn");
const listEl = document.querySelector("#todo-list");

async function adicionarComValidacao() {
  const texto = inputEl.value;

  try {
    // 3. A INVOCAÇÃO: Chamamos o comando pelo nome exato definido no Rust
    // O segundo parâmetro é um objeto onde a chave deve ser IGUAL ao nome do argumento no Rust (texto)
    const mensagemSucesso = await invoke<string>("validar_tarefa", { texto: texto });
    
    // Se chegou aqui, o Rust retornou 'Ok'
    console.log(mensagemSucesso);
    
    const li = document.createElement("li");
    li.textContent = texto;
    listEl?.appendChild(li);
    inputEl.value = "";

  } catch (erro) {
    // Se o Rust retornou 'Err', ele cai no catch do JavaScript
    alert("Erro de Validação: " + erro);
  }
}

btnEl?.addEventListener("click", adicionarComValidacao);
```

---

## 3. Testando no Codespace

Como estamos em um ambiente de desenvolvimento (Codespace), o comando para compilar o Rust e abrir o servidor de frontend continua sendo:

```bash
npm run tauri dev
```

### O que observar no teste:
1.  Tente adicionar uma tarefa vazia: Você verá o `alert` disparado pelo `catch`, com a mensagem vinda direto do Rust.
2.  Tente adicionar apenas "Oi": O Rust rejeitará por ser menor que 3 caracteres.
3.  Tente uma tarefa válida: O código seguirá o fluxo normal e adicionará o item.

**Lembre-se:** No Codespace, se o Rust não compilar, verifique se você não esqueceu de fechar algum parêntese ou se o nome da função no `invoke` está idêntico ao do `main.rs`.

---

## Destaques para sua aula:
* **Segurança:** Mostre que o JavaScript não decide se a tarefa é válida, quem decide é o binário nativo (Rust).
* **Tipagem:** O `invoke<string>` ajuda o TypeScript a saber que a resposta será um texto, evitando erros de "undefined".
* **Payloads:** O objeto `{ texto: texto }` é transformado em JSON automaticamente pelo Tauri e o Rust o transforma em uma `String` de forma transparente.

---

Avançando no **Módulo 3**, entramos agora na parte que separa os amadores dos profissionais: a **robustez**. 

No mundo real, as coisas falham. O servidor cai, o disco enche ou o usuário digita algo absurdo. O Rust é famoso por sua segurança justamente porque ele te obriga a tratar esses casos usando o tipo `Result`. Além disso, vamos aprender a não "travar" a interface do usuário enquanto o Rust faz um trabalho pesado.

---

## 1. O Raciocínio: Sincronismo vs. Assincronismo

Imagine que o seu To-Do List agora precisa "criptografar" a tarefa antes de salvar, e isso leva 2 segundos.
* **Sincronismo (Ruim):** O usuário clica em "Adicionar", a interface congela, o mouse não mexe e ele acha que o app travou.
* **Assincronismo (Bom):** O Rust inicia a tarefa em uma "thread" separada. O JavaScript recebe uma promessa (Promise) e pode mostrar um ícone de carregamento enquanto espera.

No Rust, usamos `async fn` para isso. O Tauri gerencia automaticamente o conjunto de threads (thread pool) para você.



---

## 2. O Lado do Rust: Processamento Pesado e Respostas Seguras

Vamos criar um comando que simula um salvamento demorado e que pode falhar propositalmente se o texto for "proibido".

**Arquivo:** `src-tauri/src/main.rs`
**Pasta:** `src-tauri/src/`

```rust
use std::time::Duration;
use tauri::async_runtime::spawn;

// O comando agora é 'async' para não bloquear a thread principal do SO
#[tauri::command]
async fn salvar_tarefa_remota(texto: String) -> Result<String, String> {
    println!("Backend: Iniciando processo de salvamento pesado para: {}", texto);

    // Simulando um delay de 2 segundos (ex: chamada de rede ou criptografia)
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Raciocínio de Erro: Se a palavra for 'erro', simulamos uma falha
    if texto.to_lowercase() == "erro" {
        return Err("Falha crítica: O servidor de tarefas está offline!".into());
    }

    Ok(format!("Tarefa '{}' salva com sucesso no banco de dados!", texto))
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![salvar_tarefa_remota])
        .run(tauri::generate_context!())
        .expect("erro ao rodar a aplicação");
}
```

> **Nota para a Aula:** Explique que o `Result<String, String>` funciona como uma caixa. Ou ela contém o sucesso (`Ok`), ou contém a mensagem de erro (`Err`). O JavaScript saberá distinguir isso automaticamente.

---

## 3. O Lado do JavaScript: Feedback para o Usuário

No Frontend, precisamos gerenciar o estado de "carregando". Sem isso, o usuário clicará no botão várias vezes achando que nada aconteceu.

**Arquivo:** `src/main.ts`
**Pasta:** `src/`

```typescript
import { invoke } from "@tauri-apps/api/tauri";

const inputEl = document.querySelector("#todo-input") as HTMLInputElement;
const btnEl = document.querySelector("#add-btn") as HTMLButtonElement;
const listEl = document.querySelector("#todo-list");

async function salvarComFeedback() {
  const texto = inputEl.value;
  
  // 1. Início do Estado de Carregamento
  btnEl.disabled = true;
  btnEl.textContent = "Salvando...";

  try {
    // 2. Chamada Assíncrona
    // O JS espera o 'await' sem travar o restante do navegador
    const resposta = await invoke<string>("salvar_tarefa_remota", { texto });
    
    // Se o Rust retornou 'Ok', cai aqui
    const li = document.createElement("li");
    li.textContent = texto;
    listEl?.appendChild(li);
    
    console.log("Sucesso:", resposta);
    inputEl.value = "";

  } catch (erro) {
    // 3. Tratamento de Erro
    // Se o Rust retornou 'Err', o invoke lança uma exceção que o catch captura
    alert("Ops! Algo deu errado: " + erro);
    console.error("Erro vindo do Rust:", erro);
  } finally {
    // 4. Fim do Estado de Carregamento (Sempre executa, dando erro ou não)
    btnEl.disabled = false;
    btnEl.textContent = "Add";
  }
}

btnEl?.addEventListener("click", salvarComFeedback);
```

---

## 4. Testando o Comportamento

No terminal do seu Codespace:

```bash
npm run tauri dev
```

### O que mostrar para os alunos:
1.  **O Delay:** Digite uma tarefa e clique em Add. Mostre que o botão fica desabilitado por 2 segundos e depois a tarefa aparece. Isso mostra o `async/await` em ação.
2.  **O Erro Controlado:** Digite a palavra **"erro"** e clique em Add. O `alert` será exibido com a mensagem exata que escrevemos no Rust. 
3.  **A Segurança:** Explique que, ao usar `Result`, o Rust garante que você não esqueça de tratar o erro no frontend (via `catch`), evitando que o app feche inesperadamente (o famoso "crash").

---

## Resumo dos Conceitos-Chave:
* **`async fn`**: Permite que o Rust processe dados em segundo plano.
* **`Result<T, E>`**: A forma elegante do Rust de dizer "Isso pode dar certo ou errado".
* **`try/catch/finally`**: Como o JavaScript consome a segurança do Rust e mantém o usuário informado.

---

Fala, mestre! Chegamos a um ponto sensacional. Até agora, o JavaScript era o "chefe" que pedia coisas e o Rust apenas obedecia (`invoke`). 

No **Módulo 3: Eventos**, a dinâmica muda. Imagine que o Rust está fazendo uma tarefa pesada em segundo plano e precisa avisar a interface: *"Ei, já fiz 50%!"*. Isso é feito via **Events (Emit/Listen)**. É como uma rádio: o Rust transmite em uma frequência (o nome do evento) e qualquer parte do Frontend que estiver "sintonizada" vai reagir.

---

## 1. O Raciocínio: Inversão de Controle

Se usarmos apenas o `invoke`, o Frontend fica preso esperando a resposta. Com **Eventos**, o fluxo é livre:
* **Unidirecional Push:** O Rust empurra informação para o Frontend sem que o Frontend tenha perguntado nada naquele momento exato.
* **Global ou Específico:** Você pode gritar para todas as janelas (`emit_all`) ou sussurrar para uma específica.



---

## 2. Preparando a Interface (O Receptor)

Primeiro, vamos preparar nosso HTML para mostrar o progresso de uma "sincronização" de tarefas que o Rust fará.

**Local:** `src/index.html`
Adicione uma barra de progresso abaixo da lista:

```html
<div id="sync-container" style="margin-top: 20px; display: none;">
  <label>Sincronizando com a Nuvem...</label>
  <progress id="sync-progress" value="0" max="100" style="width: 100%;"></progress>
  <span id="sync-status">0%</span>
</div>
```

---

## 3. O Lado do Rust: O "Emissor" (Backend)

Vamos criar um comando que simula um processo longo e dispara eventos a cada passo.

**Local:** `src-tauri/src/main.rs`
**Pasta:** `src-tauri/src/`

```rust
use std::time::Duration;
use tauri::Manager; // Importante para ter acesso ao emit_all

#[tauri::command]
async fn sincronizar_tarefas(window: tauri::Window) {
    // Raciocínio: Recebemos a 'window' automaticamente pelo Tauri 
    // para podermos emitir eventos especificamente para ela.
    
    let mut progresso = 0;
    
    while progresso <= 100 {
        // Simulando trabalho...
        tokio::time::sleep(Duration::from_millis(500)).await;
        
        progresso += 20;

        // EMIT: O Rust "grita" o evento 'progresso-sync'
        // O segundo parâmetro é o payload (os dados que o JS vai receber)
        window.emit("progresso-sync", progresso).unwrap();
        
        println!("Rust disparou evento de progresso: {}%", progresso);
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![sincronizar_tarefas])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

---

## 4. O Lado do JavaScript: O "Ouvinte" (Frontend)

Agora vamos usar a função `listen` para capturar esse grito do Rust.

**Local:** `src/main.ts`
**Pasta:** `src/`

```typescript
import { invoke } from "@tauri-apps/api/tauri";
import { listen } from "@tauri-apps/api/event"; // Importamos o listen

const syncContainer = document.querySelector("#sync-container") as HTMLElement;
const progressBar = document.querySelector("#sync-progress") as HTMLProgressElement;
const statusText = document.querySelector("#sync-status") as HTMLElement;

// 1. O Raciocínio: Escutar assim que o app carregar
// O listen retorna uma função para "parar de ouvir" (unlisten), útil para evitar vazamento de memória
async function iniciarEscutaDeEventos() {
  await listen<number>("progresso-sync", (event) => {
    // O payload do Rust vem dentro de event.payload
    const valor = event.payload;
    
    syncContainer.style.display = "block";
    progressBar.value = valor;
    statusText.textContent = `${valor}%`;

    if (valor === 100) {
      setTimeout(() => {
        syncContainer.style.display = "none";
        alert("Sincronização concluída!");
      }, 1000);
    }
  });
}

// Chamamos a escuta
iniciarEscutaDeEventos();

// Botão para testar (você pode adicionar um botão no HTML para isso)
// btnSync.addEventListener("click", () => invoke("sincronizar_tarefas"));
```

---

## 5. Testando no Codespace

Se você seguiu os módulos anteriores, suas dependências estão prontas. Rode o comando:

```bash
npm run tauri dev
```

### Como validar a aula:
1.  **Disparo:** Assim que o comando `sincronizar_tarefas` for executado (via console ou botão), a barra de progresso no HTML começará a encher sozinha.
2.  **Assincronismo:** Mostre aos alunos que você ainda pode interagir com o resto do To-Do List enquanto a barra enche. O evento não bloqueia a UI!
3.  **Logs:** Olhe o terminal do Codespace. Você verá o Rust logando o progresso enquanto o Frontend reage visualmente.

---

### Por que isso é importante?
Imagine um app de música: quando a música muda, o Rust (que gerencia o player nativo) avisa o JS: *"A música agora é X"*. O JS não precisa ficar perguntando a cada segundo. **Isso economiza bateria e processamento.**

---

Fala, mestre! Chegamos a um dos pontos mais críticos e interessantes do Tauri. Se o **IPC** é a ponte, o **State Management** é o "Cérebro" ou o "Cofre" do lado do Rust.

Até agora, nossas tarefas existiam apenas na memória do navegador (JavaScript). Se o usuário desse um F5, tudo sumia. Para resolver isso de forma profissional, precisamos que o **Rust** seja o dono da "Verdade" (o Estado da aplicação).

---

## 1. O Raciocínio: Por que Mutex e State?

No Rust, a segurança é levada muito a sério. Imagine que você tem duas janelas do seu app abertas e ambas tentam adicionar uma tarefa ao mesmo tempo. No JavaScript, isso poderia causar confusão. No Rust, o compilador nem deixa você fazer isso sem proteção.

* **`app.manage()`**: É como uma mochila que o Tauri carrega. Você coloca lá dentro conexões com banco de dados, configurações ou, no nosso caso, a lista de tarefas.
* **`Mutex` (Mutual Exclusion)**: Imagine uma cabine telefônica (antiga, eu sei!). Apenas **uma** pessoa (thread) pode entrar e usar por vez. Enquanto uma thread está escrevendo na lista de tarefas, o Mutex "tranca" a porta para que ninguém mais estrague os dados.
* **`State`**: É como o comando pede ao Tauri: "Ei, pegue aquele item que está na sua mochila e me empreste o acesso".



---

## 2. O Lado do Rust: Configurando o "Cofre"

Vamos criar uma estrutura para segurar nossas tarefas e registrá-la no ciclo de vida do app.

**Arquivo:** `src-tauri/src/main.rs`  
**Pasta:** `src-tauri/src/`

```rust
use std::sync::Mutex;
use tauri::State;

// 1. Definimos a estrutura que será guardada no Estado Global
struct AppState {
    // Usamos Mutex para que o Rust nos permita alterar o Vec com segurança entre threads
    tarefas: Mutex<Vec<String>>,
}

// 2. Comando para adicionar uma tarefa ao estado do Rust
#[tauri::command]
fn adicionar_ao_estado(nova_tarefa: String, state: State<'_, AppState>) -> Result<usize, String> {
    // .lock() tenta pegar a "chave" do Mutex. Se outra thread estiver usando, ele espera.
    let mut lista = state.tarefas.lock().map_err(|_| "Erro ao trancar o cofre de tarefas")?;
    
    lista.push(nova_tarefa);
    
    println!("Estado atualizado! Total de tarefas: {}", lista.len());
    
    // Retornamos o novo tamanho da lista
    Ok(lista.len())
}

// 3. Comando para recuperar todas as tarefas guardadas no Rust
#[tauri::command]
fn obter_todas_tarefas(state: State<'_, AppState>) -> Vec<String> {
    let lista = state.tarefas.lock().expect("Falha ao ler estado");
    // Retornamos uma cópia da lista para o Frontend
    lista.clone()
}

fn main() {
    tauri::Builder::default()
        // 4. O REGISTRO: Aqui "entregamos a mochila" para o Tauri
        .manage(AppState {
            tarefas: Mutex::new(Vec::new()),
        })
        .invoke_handler(tauri::generate_handler![adicionar_ao_estado, obter_todas_tarefas])
        .run(tauri::generate_context!())
        .expect("erro ao rodar a aplicação");
}
```

---

## 3. O Lado do JavaScript: Sincronizando com o Cérebro

Agora, toda vez que o app carregar ou uma tarefa for adicionada, vamos conversar com o Estado Global do Rust.

**Arquivo:** `src/main.ts`  
**Pasta:** `src/`

```typescript
import { invoke } from "@tauri-apps/api/tauri";

const inputEl = document.querySelector("#todo-input") as HTMLInputElement;
const btnEl = document.querySelector("#add-btn");
const listEl = document.querySelector("#todo-list");

// Raciocínio: Ao carregar a página, perguntamos ao Rust: "O que você tem guardado?"
window.addEventListener("DOMContentLoaded", async () => {
  const tarefasSalvas = await invoke<string[]>("obter_todas_tarefas");
  tarefasSalvas.forEach(t => renderizarNaTela(t));
});

async function addTarefa() {
  const texto = inputEl.value;
  if (!texto) return;

  try {
    // Enviamos para o Rust guardar no Mutex
    await invoke("adicionar_ao_estado", { novaTarefa: texto });
    
    renderizarNaTela(texto);
    inputEl.value = "";
  } catch (err) {
    console.error(err);
  }
}

function renderizarNaTela(texto: string) {
  const li = document.createElement("li");
  li.textContent = texto;
  listEl?.appendChild(li);
}

btnEl?.addEventListener("click", addTarefa);
```

---

## 4. Testando no Codespace

Para validar essa aula, siga os comandos:

```bash
# Entre na pasta do projeto
cd tauri-todo

# Rode o ambiente de desenvolvimento
npm run tauri dev
```

### O que explicar durante o teste:
1.  **Persistência na Sessão:** Note que, se você tiver uma lógica de recarregamento de página (como o F5 do navegador), as tarefas agora podem ser recuperadas do Rust no `DOMContentLoaded` porque elas não estão mais apenas em uma variável volátil do JS, mas no processo Core (Rust).
2.  **O Lock do Mutex:** Se você tentar adicionar tarefas muito rápido, o Rust gerencia as filas de acesso ao `Vec` sem que você precise se preocupar com travamentos de memória.
3.  **Logs do Terminal:** Observe o terminal do Codespace. Você verá a mensagem: `Estado atualizado! Total de tarefas: X`. Isso confirma que o Rust está ativamente gerenciando a contagem.

---

### Dica de Ouro para o Codespace:
Se o Rust demorar para compilar, lembre seus alunos que o Codespace usa processadores compartilhados. O Rust faz muitas verificações de segurança no momento da compilação para que o app nunca trave em produção.

# Módulo 4

Chegamos ao **Módulo 4**, o pilar que diferencia o Tauri de quase qualquer outro framework de desktop: a **Segurança**.

No desenvolvimento de software, existe um conceito chamado **Princípio do Menor Privilégio**. O Tauri leva isso ao extremo com o que chamamos de "Secure by Design" (Seguro por Projeto).

---

## 1. O Raciocínio: O "Zero Trust" (Confiança Zero)

No Electron, se um atacante conseguir injetar um script malicioso no seu frontend (via XSS, por exemplo), ele pode ter acesso total ao Node.js e, consequentemente, apagar todos os arquivos do computador do usuário.

No Tauri, a interface (WebView) vive em uma "ilha isolada". Ela não tem permissão para fazer **nada** no sistema operacional, a menos que você, o desenvolvedor, assine uma autorização explícita.
* **Permissions (Permissões):** São as chaves individuais (ex: "pode ler arquivos", "pode abrir janelas").
* **Scopes (Escopos):** É o limite da chave (ex: "pode ler arquivos, mas **apenas** dentro da pasta `/documentos/meu-app`").
* **Capabilities (Capacidades):** É o conjunto de permissões que você dá para uma janela específica.



---

## 2. Preparando o Terreno no Codespace

Para demonstrar a segurança, vamos tentar salvar nossa lista de tarefas em um arquivo de texto. Primeiro, precisamos adicionar o plugin de Sistema de Arquivos (FileSystem) ao projeto.

**No terminal do Codespace:**
```bash
# Adiciona o plugin de filesystem (Backend Rust)
cd src-tauri
cargo add tauri-plugin-fs
# Adiciona a biblioteca de suporte do lado do JS
cd ..
npm install @tauri-apps/plugin-fs
```

---

## 3. Configurando as "Chaves do Cofre" (Permissions)

O Tauri 2.0 organiza as permissões dentro de uma pasta dedicada. Vamos dizer ao sistema que nosso Frontend tem permissão para escrever e ler arquivos, mas com um **Escopo** restrito.

**Local:** `src-tauri/capabilities/default.json`
*(Se o arquivo não existir, crie-o. Se existir, adicione estas permissões ao array `permissions`)*

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capacidades padrão para o To-Do List",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "fs:allow-write-text-file",
    "fs:allow-read-text-file",
    {
      "identifier": "fs:allow-app-cache-write",
      "comment": "Permitir escrever apenas na pasta de cache do app"
    }
  ]
}
```

**Raciocínio:** Note que não demos permissão para "fs:allow-all". Se o código JavaScript tentar acessar a pasta `C:/Windows` ou `/etc/shadow`, o Tauri bloqueará a execução imediatamente, pois não está no nosso escopo autorizado.

---

## 4. Implementando o Salvamento Seguro

Agora vamos escrever o código no Frontend que utiliza essa permissão. O objetivo é salvar a lista de tarefas em um arquivo chamado `tarefas.txt` dentro da pasta de dados da aplicação.

**Local:** `src/main.ts`
**Pasta:** `src/`

```typescript
import { writeTextFile, BaseDirectory } from '@tauri-apps/plugin-fs';

const btnSalvar = document.querySelector("#save-btn");

async function salvarNoDisco() {
  const tarefas = document.querySelectorAll("#todo-list li span");
  const listaTexto = Array.from(tarefas).map(t => t.textContent).join("\n");

  try {
    // Raciocínio: Usamos o BaseDirectory.AppData para garantir que o arquivo
    // seja salvo em um local seguro e isolado, gerenciado pelo SO.
    await writeTextFile('tarefas.txt', listaTexto, { 
      baseDir: BaseDirectory.AppData 
    });

    alert("Segurança Tauri: Arquivo gravado com sucesso no escopo permitido!");
  } catch (error) {
    // Se você esquecer de adicionar a permissão no default.json, 
    // o erro cairá aqui como "Permission Denied".
    console.error("Bloqueio de Segurança:", error);
    alert("Erro de permissão: A WebView tentou acessar o disco sem autorização.");
  }
}

// Lembre-se de adicionar um botão <button id="save-btn"> no seu index.html
btnSalvar?.addEventListener("click", salvarNoDisco);
```

---

## 5. Testando a Segurança

Para validar, execute o comando:

```bash
npm run tauri dev
```

### O Experimento da Aula:
1.  **Teste Positivo:** Clique no botão de salvar. O arquivo será criado silenciosamente na pasta de dados do app no Linux do Codespace.
2.  **Teste de Auditoria:** Vá ao arquivo `src-tauri/capabilities/default.json` e remova a linha `"fs:allow-write-text-file"`. Salve e tente clicar no botão novamente. 
    * **Resultado:** O Tauri impedirá a ação e você verá o erro no console. Isso prova que o código JS não tem poder soberano; ele é um súdito do arquivo de configurações de segurança.

---

## Por que isso é vital para seus alunos?
A maioria dos desenvolvedores ignora segurança até ser tarde demais. Ensinar Capabilities e Scopes no Módulo 4 mostra que, com Tauri, eles estão construindo softwares que protegem a privacidade e os dados do usuário final por padrão.

---

Fala, dev\! Se as **Capabilities** que vimos antes eram os guardas no portão do sistema operacional, o **CSP (Content Security Policy)** é o segurança infiltrado dentro da "festa" da sua interface (WebView).

O CSP é uma camada de segurança que informa ao navegador (ou à WebView do Tauri) quais fontes de conteúdo — como scripts, estilos e imagens — são confiáveis. Se um invasor tentar injetar um script de um servidor malicioso no seu To-Do List, o CSP olha para a lista de convidados e diz: *"Você não está na lista, não entra"*.

-----

## 1. O Raciocínio: Bloqueando o "Inimigo Externo"

No desenvolvimento Web tradicional, estamos acostumados a puxar bibliotecas de CDNs (como o Google Fonts ou scripts do Firebase). No entanto, em uma aplicação desktop nativa, cada conexão externa é um risco potencial.

O CSP do Tauri atua como uma **lista branca (allowlist)**. Por padrão, o Tauri sugere uma política altamente restritiva: **nada de fora entra, e nada de dentro sai sem permissão**.

  * **`default-src 'self'`**: Só aceita conteúdo que venha do próprio pacote da aplicação.
  * **`script-src 'self'`**: Proíbe a execução de scripts de domínios externos ou scripts "inline" (escritos diretamente no HTML) que não tenham uma assinatura digital (nonce).

-----

## 2\. Configurando o Escudo no Tauri

Diferente de um site comum onde você coloca uma tag `<meta>`, no Tauri nós configuramos o CSP diretamente no "cérebro" das configurações, garantindo que ele seja aplicado antes mesmo da interface carregar.

**Local:** `src-tauri/tauri.conf.json`
**Pasta:** `src-tauri/`

Abra o arquivo e localize a seção `tauri` \> `security`. Vamos definir uma política que permite apenas scripts locais.

```json
{
  "tauri": {
    "security": {
      "csp": "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline';"
    },
    "bundle": {
      "identifier": "com.tauri.todo.seguro"
    }
    // ... restante das configurações
  }
}
```

**O que acabamos de fazer:**

1.  **`default-src 'self'`**: Bloqueamos qualquer recurso (imagens, fontes) que não esteja dentro da nossa pasta `src`.
2.  **`script-src 'self'`**: Proibimos o carregamento de arquivos `.js` externos. Se alguém tentar colocar um `<script src="https://hacker.com/malice.js">`, o Tauri vai travar.
3.  **`style-src 'self' 'unsafe-inline'`**: Permitimos estilos locais e estilos escritos dentro de tags `<style>` (comum em frameworks modernos), mas ainda bloqueamos CSS de domínios estranhos.

-----

## 3\. Testando a Proteção (Simulando um Ataque)

Para provar aos seus alunos que isso funciona, vamos tentar burlar nossa própria segurança.

**Local:** `src/index.html`
**Pasta:** `src/`

Tente adicionar um script "malicioso" de mentira que aponta para um servidor externo:

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <title>Tauri Todo</title>
    <script src="https://code.jquery.com/jquery-3.7.1.min.js"></script>
  </head>
  <body>
    <div id="app">
        <h1>Meu To-Do Seguro</h1>
        </div>
    <script type="module" src="/main.ts"></script>
  </body>
</html>
```

-----

## 4\. Verificando o Bloqueio no Codespace

Agora, execute a aplicação para ver o segurança em ação.

**No terminal do seu Codespace:**

```bash
npm run tauri dev
```

### O que observar:

1.  O app vai abrir normalmente (pois o seu `main.ts` é local e permitido).
2.  Abra o **Console de Desenvolvedor** (Botão direito na tela \> *Inspect Element* \> aba *Console*).
3.  Você verá um erro vermelho berrante:
    > **Refused to load the script '[https://code.jquery.com/jquery-3.7.1.min.js](https://www.google.com/url?sa=E&source=gmail&q=https://code.jquery.com/jquery-3.7.1.min.js)' because it violates the following Content Security Policy directive: "script-src 'self'".**

**Isso é o CSP salvando o dia\!** Mesmo que um hacker conseguisse injetar esse link no seu HTML, o motor da WebView se recusaria a baixar o arquivo porque ele não é `'self'` (ele mesmo/local).

-----

## Destaques para sua aula:

  * **Segurança Silenciosa:** O CSP não avisa o usuário, ele apenas impede o ataque silenciosamente no nível do motor de renderização.
  * **Performance:** Ao proibir recursos externos não autorizados, você também evita que o app tente baixar lixo da rede, tornando-o mais rápido.
  * **Dica de Debug:** Se um dia um ícone ou fonte do Google não carregar no seu app, a primeira coisa a verificar é se o domínio deles está na sua `csp` dentro do `tauri.conf.json`.

-----

# Módulo 5

Fala, mestre! Chegamos ao **Módulo 5**, onde a produtividade decola. Até aqui, nós escrevemos código Rust na mão para criar comandos e gerenciar estado. Mas, e se você precisar de algo complexo, como abrir uma janela nativa do Windows ou conectar a um banco de dados SQLite? 

Não precisamos reinventar a roda. O Tauri possui um **Ecossistema de Plugins Oficiais**. Eles são pacotes de código Rust já prontos e auditados, que expõem funções limpas para o nosso JavaScript.



Nesta aula, vamos implementar dois recursos no nosso To-Do List:
1.  **Store Plugin:** Para salvar a preferência de "Tema Escuro/Claro" do usuário.
2.  **Dialog + FS Plugins:** Para permitir que o usuário exporte sua lista de tarefas escolhendo o local exato no computador com uma janela nativa.

---

## 1. O Raciocínio: Por que usar Plugins?

* **Store vs FS:** Ler e escrever arquivos de texto bruto (FS) é ótimo para exportar dados. Mas para salvar configurações do app (como volume, tema, idioma), gerenciar um arquivo JSON na mão é trabalhoso. O **Store** faz isso automaticamente, criando um banco de dados chave-valor leve.
* **Dialog:** É a ponte para a interface do sistema operacional. Em vez de forçar o salvamento em uma pasta obscura, o Dialog abre aquela janela clássica de "Salvar Como...", dando controle ao usuário.

---

## 2. Instalando as Ferramentas (Backend e Frontend)

No terminal do seu Codespace, precisamos instalar os plugins tanto no Rust (Cargo) quanto no JavaScript (NPM).

**No terminal, execute:**
```bash
# Entra na pasta do Rust e instala os plugins do backend
cd src-tauri
cargo add tauri-plugin-store
cargo add tauri-plugin-dialog
cargo add tauri-plugin-fs

# Volta para a raiz e instala as bibliotecas do frontend
cd ..
npm install @tauri-apps/plugin-store @tauri-apps/plugin-dialog @tauri-apps/plugin-fs
```

---

## 3. O Lado do Rust: Inicializando os Plugins

Para que o Tauri reconheça os plugins, precisamos "plugá-los" no Builder do nosso `main.rs`.

**Arquivo:** `src-tauri/src/main.rs`
**Pasta:** `src-tauri/src/`

Adicione as inicializações logo no começo da função `main`:

```rust
fn main() {
    tauri::Builder::default()
        // Inicializa o plugin de Dialog (Janelas nativas)
        .plugin(tauri_plugin_dialog::init())
        // Inicializa o plugin de FS (Sistema de arquivos)
        .plugin(tauri_plugin_fs::init())
        // Inicializa o plugin de Store (Preferências chave-valor)
        .plugin(tauri_plugin_store::Builder::new().build())
        
        // ... (mantenha os invoke_handlers e setup que já tínhamos)
        .run(tauri::generate_context!())
        .expect("erro ao rodar a aplicação");
}
```

---

## 4. Configurando as Permissões (Obrigatório no Tauri 2)

Lembra do Módulo 4? O Tauri não deixa os plugins agirem sem permissão explícita. Precisamos atualizar nossas capacidades.

**Arquivo:** `src-tauri/capabilities/default.json`
**Pasta:** `src-tauri/capabilities/`

Adicione as permissões dos plugins na lista `"permissions"`:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capacidades do App",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "store:default",
    "dialog:default",
    "fs:allow-write-text-file",
    "fs:allow-read-text-file",
    {
      "identifier": "fs:allow-app-write",
      "comment": "Permite salvar em locais escolhidos pelo Dialog"
    }
  ]
}
```

---

## 5. Escrevendo a Lógica no JavaScript

Agora, vamos usar as APIs dos plugins na nossa interface.

### Passo 1: O Botão de Tema (Usando o Store)

O **Store** vai criar um arquivo oculto chamado `settings.json` para guardar se o usuário prefere o tema escuro ou não.

**Arquivo:** `src/main.ts`
**Pasta:** `src/`

Adicione este código no seu arquivo TypeScript:

```typescript
import { Store } from '@tauri-apps/plugin-store';

// 1. Conecta ou cria o arquivo de configurações
const store = new Store('settings.json');

async function configurarTema() {
  // Busca a chave 'temaEscuro' no Store. Se não existir, retorna false.
  let isDarkMode = await store.get<{ temaEscuro: boolean }>('temaEscuro');

  // Aplica o tema visualmente
  if (isDarkMode?.temaEscuro) {
    document.body.style.backgroundColor = '#333';
    document.body.style.color = '#fff';
  }

  // Lógica para o botão de alternar tema (crie um <button id="theme-btn"> no HTML)
  document.querySelector('#theme-btn')?.addEventListener('click', async () => {
    const atual = await store.get<{ temaEscuro: boolean }>('temaEscuro');
    const novoStatus = !(atual?.temaEscuro || false);
    
    // Salva no disco imediatamente
    await store.set('temaEscuro', { temaEscuro: novoStatus });
    await store.save(); // Essencial para persistir no arquivo físico
    
    // Atualiza a tela
    document.body.style.backgroundColor = novoStatus ? '#333' : '#f4f4f9';
    document.body.style.color = novoStatus ? '#fff' : '#333';
  });
}

configurarTema();
```

### Passo 2: Exportar Tarefas (Dialog + FS)

Aqui mostramos a união de dois plugins. O usuário clica em "Exportar", o `Dialog` abre a janela nativa para ele escolher a pasta e o nome do arquivo, e o `FS` faz a gravação.

Ainda no **Arquivo:** `src/main.ts`:

```typescript
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';

async function exportarTarefas() {
  // 1. DIALOG: Pede ao sistema operacional para abrir a janela "Salvar Como..."
  const caminhoArquivo = await save({
    filters: [{
      name: 'Arquivo de Texto',
      extensions: ['txt']
    }],
    defaultPath: 'minhas_tarefas.txt'
  });

  // Se o usuário fechar a janela sem escolher nada, retorna null
  if (!caminhoArquivo) {
    console.log("Exportação cancelada pelo usuário.");
    return;
  }

  // 2. Coleta as tarefas da tela
  const tarefas = document.querySelectorAll("#todo-list li");
  const textoParaSalvar = Array.from(tarefas).map(t => t.textContent).join("\n");

  // 3. FS: Grava o arquivo no caminho exato que o Dialog devolveu
  try {
    await writeTextFile(caminhoArquivo, textoParaSalvar);
    alert(`Sucesso! Tarefas salvas em:\n${caminhoArquivo}`);
  } catch (err) {
    alert("Erro ao salvar arquivo: " + err);
  }
}

// Crie um <button id="export-btn">Exportar Lista</button> no HTML
document.querySelector('#export-btn')?.addEventListener('click', exportarTarefas);
```

---

## 6. Testando no Codespace

Chegou a hora de ver o ecossistema funcionando em conjunto.

**No terminal:**
```bash
npm run tauri dev
```

### Pontos de atenção durante o teste da aula:
1.  **O Store:** Clique no botão de tema, atualize a página da webview (F5) e mostre que o fundo escuro permaneceu. O plugin gravou isso permanentemente sem você precisar escrever rotinas complexas de I/O em Rust.
2.  **O Dialog:** Clique em "Exportar Lista". O Codespace (por ser Linux via navegador) abrirá a janela de seleção de arquivos nativa do GTK/Linux. Isso prova que o HTML puro agora tem poderes completos de desktop.

---

Excelente decisão! Integrar o **Plugin SQL** eleva a nossa aplicação de um simples bloco de notas para um software robusto. Vamos usar o **SQLite**, que é um motor de base de dados relacional completo, mas que funciona localmente num único ficheiro, sem precisar de instalar servidores (como o MySQL ou PostgreSQL).

---

## 1. O Raciocínio: Porquê o SQLite e o Plugin SQL?

* **Estrutura e Consultas:** Ao usar o *File System* (Módulo 4), se quisermos procurar apenas as tarefas "concluídas", temos de carregar todo o texto para a memória do JavaScript e filtrar. Com o SQL, delegamos esse trabalho pesado ao motor da base de dados usando um simples `SELECT * FROM tarefas WHERE concluida = 1`. É muito mais rápido e consome menos memória.
* **Segurança Concorrente:** O SQLite gere automaticamente os bloqueios (locks). Se duas partes da aplicação tentarem escrever ao mesmo tempo, ele não corrompe o ficheiro de dados.
* **O Papel do Tauri:** O JavaScript no frontend (WebView) não consegue falar diretamente com bases de dados nativas por questões de segurança. O Plugin SQL atua como um tradutor seguro: o JS envia a *query* em texto, o Rust valida, executa no SQLite e devolve os dados limpos ao JS.

---

## 2. Instalação das Dependências

No terminal do Codespace, precisamos instalar as ferramentas. Repare que no lado do Rust vamos ativar especificamente a funcionalidade (feature) do SQLite.

**No terminal:**
```bash
# Entrar na pasta do backend e instalar o plugin com suporte a SQLite
cd src-tauri
cargo add tauri-plugin-sql --features sqlite

# Voltar à raiz e instalar o pacote do frontend
cd ..
npm install @tauri-apps/plugin-sql
```

---

## 3. Inicializar o Plugin no Lado do Rust

Temos de informar o "motor" do Tauri que o SQL agora faz parte da aplicação.

**Ficheiro:** `src-tauri/src/main.rs`  
**Pasta:** `src-tauri/src/`

Adicione a inicialização do construtor SQL:

```rust
fn main() {
    tauri::Builder::default()
        // Adicionamos o plugin SQL. Por defeito, ele permite acesso ao SQLite
        .plugin(tauri_plugin_sql::Builder::default().build())
        
        // (Mantenha os outros plugins que já tínhamos, como fs, dialog e store)
        // .plugin(tauri_plugin_dialog::init())
        // ...
        
        .run(tauri::generate_context!())
        .expect("erro ao correr a aplicação");
}
```

---

## 4. Configurar as Permissões de Segurança

Como aprendemos no Módulo 4, o Tauri adota a "Confiança Zero". Temos de dar permissão explícita à WebView para executar comandos SQL.

**Ficheiro:** `src-tauri/capabilities/default.json`  
**Pasta:** `src-tauri/capabilities/`

Adicione a diretiva do SQL na lista de permissões:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capacidades do App",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "sql:default"
  ]
}
```

---

## 5. Escrever a Lógica no JavaScript

Agora, vamos ligar o frontend à nossa nova base de dados. Vamos fasear o código para que o raciocínio seja claro: primeiro ligamos e criamos a tabela, depois inserimos dados, e por fim lemos os dados.

**Ficheiro:** `src/main.ts`  
**Pasta:** `src/`

### Passo 5.1: Ligação e Criação da Tabela (Migração Inicial)

Comece por limpar o código antigo de gestão de estado ou de ficheiros de texto do `main.ts` e insira o seguinte:

```typescript
import { window } from '@tauri-apps/api';
import Database from '@tauri-apps/plugin-sql';

let db: Database;

// 1. Raciocínio de Arranque: Quando o ecrã carrega, ligamos à base de dados.
// O Tauri cria o ficheiro 'tarefas.db' automaticamente na pasta de dados da aplicação.
async function iniciarBaseDeDados() {
  try {
    db = await Database.load('sqlite:tarefas.db');
    console.log("Ligação à base de dados estabelecida!");

    // 2. Garantir que a tabela existe (Migration)
    // Usamos AUTOINCREMENT para o SQLite tratar dos IDs de forma segura.
    await db.execute(`
      CREATE TABLE IF NOT EXISTS tarefas (
        id INTEGER PRIMARY KEY AUTOINCREMENT,
        texto TEXT NOT NULL,
        concluida BOOLEAN DEFAULT 0
      )
    `);
    
    // Depois de garantir a tabela, carregamos as tarefas existentes
    carregarTarefas();
  } catch (erro) {
    console.error("Erro ao iniciar a base de dados:", erro);
  }
}

// Chamar a função mal o ficheiro seja executado
iniciarBaseDeDados();
```

### Passo 5.2: Inserir e Ler Dados (CRUD)

Ainda no `main.ts`, vamos adicionar as funções para lidar com a interação do utilizador:

```typescript
const inputEl = document.querySelector("#todo-input") as HTMLInputElement;
const btnEl = document.querySelector("#add-btn");
const listEl = document.querySelector("#todo-list");

// 3. Raciocínio de Inserção (CREATE)
async function adicionarTarefaSQL() {
  const texto = inputEl.value;
  if (!texto) return;

  try {
    // Utilizamos parâmetros ($1) para evitar ataques de SQL Injection.
    // O Rust e o SQLite tratam de escapar o texto de forma segura.
    const resultado = await db.execute(
      'INSERT INTO tarefas (texto, concluida) VALUES ($1, $2)',
      [texto, 0]
    );
    
    console.log("Tarefa inserida, ID:", resultado.lastInsertId);
    inputEl.value = "";
    carregarTarefas(); // Recarrega a lista para mostrar a nova entrada
  } catch (erro) {
    alert("Erro ao guardar na base de dados: " + erro);
  }
}

// 4. Raciocínio de Leitura (READ)
async function carregarTarefas() {
  try {
    // db.select devolve um array de objetos JSON mapeados a partir das colunas
    const tarefas: Array<{ id: number, texto: string, concluida: number }> = await db.select(
      'SELECT * FROM tarefas ORDER BY id DESC'
    );

    // Limpar o ecrã antes de desenhar a nova lista
    if (listEl) listEl.innerHTML = "";

    tarefas.forEach(t => {
      const li = document.createElement("li");
      li.textContent = `[${t.id}] ${t.texto}`;
      listEl?.appendChild(li);
    });
  } catch (erro) {
    console.error("Erro ao carregar tarefas:", erro);
  }
}

btnEl?.addEventListener("click", adicionarTarefaSQL);
```

---

## 6. Testar no Codespace

Está tudo pronto! Vamos testar a persistência real baseada em SQL.

**Comando no terminal:**
```bash
npm run tauri dev
```

### O que mostrar e validar com os alunos:
1.  **Sem SQL Injection:** Peça para introduzirem uma tarefa como `' DROP TABLE tarefas; --`. Como estamos a usar a sintaxe de parâmetros (`$1`), o SQLite vai tratar isso apenas como texto literal e guardar na base de dados, protegendo a aplicação contra o ataque mais clássico de bases de dados.
2.  **Persistência Absoluta:** Adicione várias tarefas. Em seguida, feche o terminal (Ctrl+C) e volte a executar o `npm run tauri dev`. As tarefas vão aparecer imediatamente no ecrã. Isto acontece porque o ficheiro `tarefas.db` foi criado de forma persistente no sistema de ficheiros nativo do contentor Linux.
3.  **Visualizar IDs:** Repare que as tarefas aparecem com `[1]`, `[2]`, etc. Esse número não foi gerado no JavaScript, foi o motor nativo do SQLite que atribuiu o `PRIMARY KEY AUTOINCREMENT` de forma segura.

---

Fala, mestre! Excelente escolha. Retornando ao nosso bom e velho português do Brasil ("arquivos" e "tela" no lugar de "ficheiros" e "ecrã"), vamos entrar em um dos recursos mais poderosos do Tauri: os **Sidecars**.

Às vezes, você já tem um script de inteligência artificial em Python, um processador de vídeo em C++ ou um motor de regras de negócio em Node.js. Reescrever tudo isso em Rust ou JavaScript seria um desperdício de tempo e dinheiro. 

O **Sidecar** é a solução: o Tauri "empacota" esse programa externo junto com a sua aplicação e o executa de forma invisível em segundo plano (background). A interface manda dados para ele e recebe a resposta, tudo sem o usuário perceber que há outro programa rodando.



Nesta aula, vamos criar um "Analisador de Tarefas" em Python. Ele vai receber o texto da tarefa, contar as palavras e devolver um JSON estruturado para o nosso JavaScript.

---

## 1. O Raciocínio: A Anatomia do Sidecar

Para que o Tauri reconheça um arquivo como Sidecar, ele exige uma regra estrita de nomenclatura. O arquivo deve ter o nome base que você escolher, seguido do "Target Triple" (a arquitetura do sistema operacional).

No Codespace (que roda um Linux padrão), a arquitetura geralmente é `x86_64-unknown-linux-gnu`.
Portanto, se o nosso script se chama `analisador`, o arquivo real **tem** que se chamar `analisador-x86_64-unknown-linux-gnu`.

---

## 2. Criando o Binário Externo (O Script Python)

No Codespace, o Python já vem instalado. Vamos usar um truque do Linux chamado *Shebang* (`#!/usr/bin/env python3`) para transformar um script Python em um executável autônomo.

**Local:** Crie uma nova pasta chamada `binaries` dentro de `src-tauri`.  
**Pasta:** `src-tauri/binaries/`  
**Arquivo:** Crie um arquivo (sem extensão) chamado `analisador-x86_64-unknown-linux-gnu`

Cole o código abaixo dentro deste arquivo:

```python
#!/usr/bin/env python3
import sys
import json

# 1. Raciocínio: O script recebe dados via linha de comando (sys.argv)
if len(sys.argv) > 1:
    texto = sys.argv[1]
    qtd_palavras = len(texto.split())
    
    # Montamos um dicionário que será convertido para JSON
    resultado = {
        "status": "sucesso",
        "mensagem": f"O Python analisou a tarefa. Ela contém {qtd_palavras} palavras.",
        "palavras": qtd_palavras
    }
    
    # 2. Raciocínio: O Tauri lê o que o script "imprime" (stdout)
    print(json.dumps(resultado))
else:
    print(json.dumps({"status": "erro", "mensagem": "Nenhum texto recebido"}))
```

**Permissão de Execução:** Como estamos no Linux (Codespace), precisamos dizer ao sistema que esse arquivo é um programa e não apenas um texto.
No terminal, execute:
```bash
chmod +x src-tauri/binaries/analisador-x86_64-unknown-linux-gnu
```

---

## 3. Configurando o Tauri para Empacotar o Sidecar

Agora precisamos avisar o compilador do Tauri: "Ei, quando for gerar o aplicativo final, leve a pasta `binaries` junto".

**Arquivo:** `src-tauri/tauri.conf.json`  
**Pasta:** `src-tauri/`

Encontre a seção `bundle` e adicione a propriedade `externalBin`:

```json
{
  "tauri": {
    "bundle": {
      "identifier": "com.tauri.todo.seguro",
      "externalBin": ["binaries/analisador"]
    }
    // ... resto do arquivo
  }
}
```
*Observação: Note que não colocamos o sufixo `x86_64...` no JSON. O Tauri é inteligente o suficiente para anexar o sufixo automaticamente de acordo com o sistema que está compilando.*

---

## 4. Segurança: Autorizando a Execução

A arquitetura *Secure by Design* do Módulo 4 entra em ação novamente. O frontend não pode executar programas no computador do usuário sem permissão explícita. Para o Sidecar funcionar, precisamos do plugin `shell`.

**No terminal, instale o plugin shell para o JS:**
```bash
npm install @tauri-apps/plugin-shell
```
*(No Rust, o plugin shell geralmente já vem inicializado no Tauri 2, mas o pacote JS é necessário).*

**Arquivo:** `src-tauri/capabilities/default.json`  
**Pasta:** `src-tauri/capabilities/`

Adicione a permissão de shell e a regra permitindo especificamente o nosso analisador:

```json
{
  "$schema": "../gen/schemas/desktop-schema.json",
  "identifier": "default",
  "description": "Capacidades do App",
  "windows": ["main"],
  "permissions": [
    "core:default",
    "sql:default",
    "shell:default",
    {
      "identifier": "shell:allow-execute",
      "allow": [
        {
          "name": "binaries/analisador",
          "sidecar": true
        }
      ]
    }
  ]
}
```

---

## 5. Chamando o Python através do JavaScript

Vamos integrar isso ao nosso To-Do List. Toda vez que inserirmos uma tarefa, o JavaScript vai acionar o Python para analisá-la de forma assíncrona.

**Arquivo:** `src/main.ts`  
**Pasta:** `src/`

Modifique a sua função de adicionar tarefa para incluir a chamada ao Sidecar usando a classe `Command`:

```typescript
import { Command } from '@tauri-apps/plugin-shell';
// ... outros imports do SQL que fizemos na aula anterior

async function adicionarTarefaSQL() {
  const inputEl = document.querySelector("#todo-input") as HTMLInputElement;
  const texto = inputEl.value;
  if (!texto) return;

  try {
    // 1. Invocando o Sidecar Python
    // Repare que chamamos o nome base ("binaries/analisador") e passamos o texto como argumento
    const comandoPython = Command.sidecar('binaries/analisador', [texto]);
    
    // Executa e aguarda o Python terminar o processamento
    const saidaPython = await comandoPython.execute();
    
    // 2. Lemos a saída (stdout) que o Python imprimiu e convertemos de volta para objeto
    const analise = JSON.parse(saidaPython.stdout);
    
    // Mostramos a análise no console para debugar
    console.log("Resposta do Python:", analise.mensagem);

    // 3. (Opcional) Guardamos no banco SQL avisando o usuário
    await db.execute(
      'INSERT INTO tarefas (texto, concluida) VALUES ($1, $2)',
      [`${texto} (Análise: ${analise.palavras} palavras)`, 0]
    );
    
    inputEl.value = "";
    carregarTarefas(); // Atualiza a tela
  } catch (erro) {
    alert("Erro na execução: " + erro);
  }
}
```

---

## 6. Testando no Codespace

Está tudo interligado! Frontend (TS) -> Backend (Rust/Shell) -> Sidecar (Python).

**Comando no terminal:**
```bash
npm run tauri dev
```

### O que validar com os alunos durante o teste:
1.  **Isolamento:** Mostre que você não precisou criar um servidor web no Python (com Flask ou FastAPI). O Python foi executado nativamente via linha de comando, economizando memória e portas de rede.
2.  **O Console do JS:** Quando adicionarem uma tarefa, abra o *Inspect Element* > *Console*. Eles verão a mensagem formatada pelo Python aparecendo diretamente no mundo JavaScript.
3.  **Cross-Platform:** Explique que, se eles fossem compilar para Windows, bastaria criar um arquivo chamado `analisador-x86_64-pc-windows-msvc.exe` (compilado via PyInstaller) na mesma pasta `binaries`, e o Tauri cuidaria de pegar o arquivo certo automaticamente na hora do Build.

---

# Módulo 6


Fala, mestre! Chegamos ao ápice do nosso projeto: o **Módulo 6**.

Se existe um grande argumento de vendas para o Tauri contra o Electron, é o tamanho final do arquivo. Um aplicativo "Olá Mundo" no Electron não costuma ter menos de 100 MB, pois ele carrega um navegador inteiro dentro dele. No Tauri, nós conseguimos entregar o nosso To-Do List completo com poucos megabytes!

Mas, por padrão, o compilador do Rust prioriza a velocidade de compilação durante o desenvolvimento. Para o instalador final, nós precisamos inverter essa lógica: queremos que ele demore o tempo que for necessário para compilar, desde que entregue o menor e mais rápido executável possível.



---

## 1. O Raciocínio: A Dieta do Compilador (Cargo.toml)

Para secar os "quilos a mais" do nosso binário, nós utilizamos o conceito de **Profiles** no Rust. O profile de `release` é ativado automaticamente quando pedimos ao Tauri para gerar o instalador final. 

Vamos aplicar cinco estratégias no arquivo de configuração do Rust:
1.  **`opt-level = "s"`**: Em vez de otimizar para velocidade de execução (nível 3), mandamos o compilador otimizar para tamanho (`s` de *size*).
2.  **`strip = true`**: Todo programa compilado guarda "símbolos de debug" (nomes de variáveis, caminhos de pastas) para o caso de dar erro e você precisar ler o log. Em produção, isso é peso morto. O `strip` arranca tudo isso fora.
3.  **`lto = true`**: *Link Time Optimization*. O compilador vai olhar para a nossa aplicação e para *todas* as bibliotecas que usamos (plugins, sqlite, serde). Se tiver alguma função lá que o nosso código nunca chamou, ele corta fora (o famoso *Dead Code Elimination*).
4.  **`codegen-units = 1`**: Por padrão, o Rust divide a compilação em vários pedaços e usa todos os núcleos do seu processador ao mesmo tempo para ir mais rápido. O problema é que um pedaço não consegue otimizar bem com o outro. Definindo como `1`, o Rust faz tudo em uma fila única, conseguindo ver o quadro geral e otimizando o tamanho ao máximo.
5.  **`panic = "abort"`** (Bônus): Se o Rust encontra um erro fatal, ele faz um "unwind" (desfazendo as ações para gerar um log bonitinho). Em produção, se der erro fatal, a gente só quer que o app feche na hora. Isso economiza muito espaço.

---

## 2. Configurando o Cargo.toml

Vamos colocar esse raciocínio no código.

**Arquivo:** `Cargo.toml`
**Pasta:** `src-tauri/`

Abra este arquivo. Vá rolando até o final dele (abaixo das `[dependencies]`) e adicione o bloco de configuração de *release*:

```toml
# ... (suas dependências acima, não apague nada)

# Aqui começa a nossa configuração de Otimização para o Build Final
[profile.release]
# 1. Otimiza para o menor tamanho possível
opt-level = "s"

# 2. Remove todos os símbolos de debug e caminhos internos do computador
strip = true

# 3. Ativa a Otimização em Tempo de Linkagem (Remove funções não utilizadas)
lto = true

# 4. Força o compilador a usar apenas 1 unidade de geração (Demora mais, mas otimiza melhor)
codegen-units = 1

# 5. Corta a biblioteca de rastreamento de erros críticos (Menos peso)
panic = "abort"
```

---

## 3. Testando a Geração do Binário (O Build Final)

Agora não vamos mais usar o modo "dev" (`tauri dev`), que abre a tela na hora. Vamos gerar o artefato final!

Como estamos num Codespace (Linux), o Tauri vai gerar um arquivo `.deb` (instalador do Ubuntu/Debian) e um AppImage (executável universal Linux).

**No terminal, execute o comando:**
```bash
npm run tauri build
```

### O que vai acontecer agora (e o que avisar aos alunos):
* **Paciência:** Como ativamos o `lto = true` e `codegen-units = 1`, o compilador do Rust vai fazer um trabalho cirúrgico. Num Codespace (que tem recursos limitados), isso pode levar de 3 a 10 minutos. É o momento perfeito para tirar dúvidas da turma!
* **Onde fica o arquivo?** Quando o terminal terminar de processar com uma mensagem de sucesso, o arquivo final otimizado estará escondido bem fundo na pasta. O Tauri vai imprimir o caminho exato na tela, que geralmente é:
    `src-tauri/target/release/bundle/deb/` ou `src-tauri/target/release/bundle/appimage/`.

Se você fosse gerar isso no Windows (na sua máquina local com o mesmo código), o arquivo sairia na pasta `src-tauri/target/release/bundle/nsis/` como um arquivo `.exe` tradicional de instalação.

---

Fala, mestre! Chegamos à grande linha de chegada do nosso projeto. Todo aquele código frontend fluido, o backend seguro em Rust, a proteção CSP, o banco de dados SQLite e o analisador em Python vão agora se transformar em um **produto real**, um instalador que o seu usuário final pode baixar e clicar duas vezes para usar.

Nesta aula, vamos entender como o Tauri empacota tudo isso e rodar o comando definitivo da nossa aplicação.

---

## 1. O Raciocínio: O que é o Build de Produção?

Durante o desenvolvimento, usamos o `tauri dev`. Esse comando cria um servidor local rápido (Vite) e compila o Rust sem otimizações pesadas para não atrasar a sua programação. 

O **Build de Produção** faz exatamente o oposto:
1. Ele congela o seu HTML, CSS e JS (minificando tudo para tirar espaços em branco e comentários).
2. Ele compila o Rust usando as otimizações pesadas que configuramos na aula passada (arrancando código morto e reduzindo o tamanho).
3. Ele junta o binário do Rust, os arquivos web, o banco de dados e o nosso script Python (Sidecar) dentro de um pacote compactado e fechado: o Instalador.



---

## 2. Formatos e Plataformas: O que estamos gerando?

O Tauri é uma ferramenta multiplataforma, mas por padrão, **ele compila para o sistema operacional onde está rodando**. 

* **No Windows:** Ele gera um `MSI` (muito usado em empresas para instalação via rede) e um `NSIS` (o clássico arquivo `.exe` de "Avançar, Avançar, Concluir").
* **No macOS:** Ele gera um `.app` (o formato padrão de aplicativos Mac) e um `.dmg` (aquela "imagem de disco" que você arrasta para a pasta Aplicativos).
* **No Linux (Onde estamos no Codespace):** Ele gera um `.deb` (instalador para Debian/Ubuntu/Mint) e um `AppImage` (um formato incrível que não precisa instalar, é só dar dois cliques e ele roda como se fosse um portátil).

---

## 3. Configurando o Pacote Final (Identidade da Aplicação)

Antes de mandarmos compilar, precisamos dizer ao Tauri qual é a "identidade" do nosso aplicativo. Se não fizermos isso, o sistema operacional pode recusar a instalação por achar que é um aplicativo genérico.

**Pasta:** `src-tauri/`
**Arquivo:** `tauri.conf.json`

Abra o arquivo de configuração e procure pela seção `bundle`. Vamos atualizar o `identifier` (uma exigência da Apple e do Windows para saber quem criou o app), o `publisher` e os `targets` (os formatos que queremos gerar).

Altere a seção `bundle` para ficar assim:

```json
{
  "tauri": {
    "bundle": {
      "identifier": "com.mestretauri.todolist",
      "publisher": "Meu Nome ou Minha Empresa",
      "shortDescription": "Um gerenciador de tarefas blindado e ultra-rápido.",
      "targets": "all", 
      "externalBin": [
        "binaries/analisador"
      ],
      "icon": [
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.icns",
        "icons/icon.ico"
      ]
    }
    // ... resto do arquivo
  }
}
```

**O Raciocínio aqui:** Ao colocar `"targets": "all"`, dizemos ao Tauri: "Gere todos os formatos possíveis para o sistema operacional em que estou rodando agora". No nosso caso (Codespace/Linux), ele vai gerar o `.deb` e o `AppImage`.

---

## 4. Executando o Build de Produção

Agora, no terminal do seu Codespace, execute o comando que faz a mágica acontecer:

```bash
npm run tauri build
```

### O que vai acontecer agora na sua tela (e o que falar para os alunos):

1. **Vite Build:** Primeiro, você verá o Vite empacotando o JavaScript. Ele vai criar uma pasta invisível chamada `dist` com os arquivos minificados.
2. **Cargo Build:** O compilador do Rust (Cargo) vai assumir o controle. Como configuramos a otimização máxima na aula anterior (`codegen-units = 1` e `lto = true`), esse passo vai demorar um pouco. Ele está espremendo cada byte do aplicativo.
3. **Bundling:** Por fim, o Tauri vai pegar tudo isso e zipar nos formatos finais.

Quando terminar, você verá uma mensagem verde de sucesso indicando onde os arquivos foram salvos. Geralmente, eles estarão nas seguintes pastas dentro do seu projeto:

* **O arquivo portátil:** `src-tauri/target/release/bundle/appimage/todolist_1.0.0_amd64.AppImage`
* **O instalador Ubuntu:** `src-tauri/target/release/bundle/deb/todolist_1.0.0_amd64.deb`

Você pode clicar com o botão direito nesses arquivos no explorador do Codespace e selecionar **Download** para baixá--los para o seu computador físico!

---

Fala, mestre! Que jornada incrível até aqui. Agora que já sabemos gerar o nosso instalador, precisamos olhar para o sistema operacional que, estatisticamente, a maioria dos seus clientes finais vai utilizar: o **Windows**.

No Linux e no macOS, o motor que renderiza a nossa interface web geralmente já faz parte do núcleo do sistema. Mas no Windows, o Tauri depende do **Microsoft Edge WebView2**. O Windows 11 já vem com ele de fábrica, mas muitas máquinas com Windows 10 (ou ambientes corporativos restritos) não têm. Se o seu usuário não tiver o WebView2, o aplicativo simplesmente não abre.

Aqui entra a mágica do Tauri: nós podemos configurar o nosso instalador `.exe` (NSIS) para resolver esse problema sozinho!



---

## 1. O Raciocínio: As Estratégias de Instalação do WebView2

Nós temos basicamente três caminhos para configurar no instalador do Windows:

1.  **`downloadBootstrapper` (Padrão e Recomendado):** O instalador do seu To-Do List fica minúsculo (uns 4MB). Quando o usuário instala, o instalador verifica se o WebView2 existe. Se não existir, ele baixa da internet na hora e instala. *Pró: Tamanho mínimo. Contra: Exige internet na hora de instalar.*
2.  **`embedBootstrapper`**: Embutimos um pequeno "baixador" da Microsoft dentro do nosso instalador. Fica um pouquinho maior, mas burla alguns bloqueios de firewall corporativo na hora de baixar.
3.  **`offlineInstaller`**: Nós colocamos o instalador COMPLETO do WebView2 (cerca de 150MB) dentro do nosso `.exe`. *Pró: Instala em computadores sem internet (modo offline). Contra: O seu aplicativo de 4MB passa a pesar mais de 150MB!*

Para a nossa aula, vamos configurar o modo **`downloadBootstrapper`**, que é o mais inteligente para aplicativos modernos, mas vou deixar o código preparado para você mostrar as opções.

---

## 2. Configurando o Comportamento no Windows

Vamos mexer no nosso arquivo mestre de configurações para adicionar as regras específicas do Windows.

**Pasta:** `src-tauri/`  
**Arquivo:** `tauri.conf.json`

Abra o arquivo. Dentro da seção `"bundle"`, nós vamos criar um novo bloco chamado `"windows"`. Se o bloco já existir, apenas adicione a propriedade `webviewInstallMode`.

Deixe o seu arquivo com esta estrutura:

```json
{
  "tauri": {
    "bundle": {
      "identifier": "com.mestretauri.todolist",
      "publisher": "Sua Empresa",
      "targets": "all",
      
      "windows": {
        "certificateThumbprint": null,
        "digestAlgorithm": "sha256",
        "timestampUrl": "",
        "webviewInstallMode": {
          "type": "downloadBootstrapper"
        }
      }

    }
  }
}
```

### O que acontece se eu quiser o modo Offline?
Se você estiver construindo esse To-Do List para ser instalado em computadores de uma fábrica sem acesso à internet, você mudaria o bloco acima para:

```json
        "webviewInstallMode": {
          "type": "offlineInstaller"
        }
```
O Tauri é tão inteligente que, na hora do build, ele mesmo vai ao site da Microsoft, baixa o pacote de 150MB e "costura" dentro do seu `.exe`.

---

## 3. Como testar essa configuração estando no Codespace?

Aqui temos um detalhe técnico importante da vida real: **O Codespace é um ambiente Linux.** O compilador do Rust no Linux sabe gerar perfeitamente arquivos `.deb` e `AppImage`. Mas gerar um arquivo `.exe` do Windows a partir do Linux é um processo chamado *Cross-Compilation* (Compilação Cruzada), que exige instalar SDKs do Windows dentro do Linux — algo muito complexo e sujeito a erros.

Portanto, para validar se o nosso JSON não tem erros de sintaxe e se o Tauri aceita a configuração, rodamos o nosso comando padrão de build:

**No terminal, execute:**
```bash
npm run tauri build
```

*O Tauri vai ler o `tauri.conf.json`, validar a regra do Windows (não vai dar erro), mas como estamos no Linux, ele vai ignorá-la na hora de cuspir o arquivo final, gerando novamente os pacotes de Linux.*

---

Fala, mestre! Entramos agora na reta final, e este é o momento em que a sua aula sai do nível "tutorial de framework" e entra no nível "Engenharia de Software Profissional". 

Vamos falar sobre **CI/CD (Continuous Integration / Continuous Deployment)** usando o GitHub Actions.

---

## 1. O Raciocínio: Por que usar a Nuvem para compilar?

Como vimos na aula anterior, o Codespace é um ambiente Linux. Se tentarmos gerar um `.exe` do Windows a partir do Linux, teríamos que configurar ferramentas complexas de compilação cruzada (*cross-compilation*). Isso costuma dar muita dor de cabeça com dependências de C++ e bibliotecas visuais nativas.

A solução inteligente é: nós guardamos nosso código no GitHub. O GitHub possui milhares de computadores ociosos (Runners) rodando Windows, Linux e macOS reais. Nós vamos escrever um "roteiro" (um arquivo YAML) que diz ao GitHub:
*"Ei, toda vez que eu atualizar meu código, abra um computador com Windows 11 aí na sua sede, instale o Node.js, instale o Rust, compile o meu To-Do List e me devolva apenas o `.exe` pronto."*

Isso não custa nada para repositórios públicos e automatiza 100% do seu trabalho!

[A arquitetura mental aqui é: Seu Código (Linux) -> Push -> Servidor Windows do GitHub -> Build -> Instalador `.exe` disponível para download].

---

## 2. Preparando o Terreno (Aviso sobre o Sidecar)

Antes de criarmos o roteiro, um pequeno ajuste de realidade. Na aula do Sidecar (Python), nós criamos um executável exclusivo para Linux (`analisador-x86_64-unknown-linux-gnu`). Se mandarmos o GitHub compilar no Windows agora, o Tauri vai procurar o arquivo `analisador-x86_64-pc-windows-msvc.exe` na pasta `binaries` e, como não vai achar, o build vai falhar.

Para focar apenas na mágica do CI/CD nesta aula, abra o arquivo `src-tauri/tauri.conf.json` e **remova ou comente** a propriedade `"externalBin"` de dentro da seção `"bundle"`.

---

## 3. Criando o Roteiro do GitHub Actions

O GitHub reconhece automatizações apenas se elas estiverem em uma pasta com um nome muito específico.

**Ação:** Na raiz do seu projeto (fora da pasta `src`), crie uma nova pasta chamada `.github`. Dentro dela, crie outra pasta chamada `workflows`.
**Pasta:** `.github/workflows/`
**Arquivo:** Crie um arquivo chamado `release.yml`

Agora, vamos construir esse arquivo passo a passo para você entender a lógica.

### Parte 1: O Gatilho e a Máquina

Abra o arquivo `.github/workflows/release.yml` e comece escrevendo:

```yaml
name: Build Tauri Windows

# 1. Raciocínio (O Gatilho): Quando essa máquina deve ligar?
on:
  push:
    branches: [ main ] # Sempre que houver um push na branch main
  workflow_dispatch:   # Permite que você clique num botão para rodar manualmente

# 2. Raciocínio (A Máquina): Qual computador queremos alugar do GitHub?
jobs:
  build-windows:
    runs-on: windows-latest # Queremos a versão mais recente do Windows!
    
    steps:
      # O primeiro passo é o computador baixar o seu código
      - name: Baixar o código do repositório
        uses: actions/checkout@v4
```

### Parte 2: Preparando o Ambiente

Continuando no mesmo arquivo, logo abaixo, precisamos instalar as ferramentas que usamos no nosso Codespace, mas agora nesse Windows limpo do GitHub.

```yaml
      # 3. Raciocínio (Dependências): O Windows limpo não tem Node nem Rust
      - name: Instalar Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Instalar Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: x86_64-pc-windows-msvc # Alvo específico para Windows

      - name: Instalar dependências do Frontend (NPM)
        run: npm install
```

### Parte 3: A Mágica do Tauri Action

A equipe do Tauri criou uma automação oficial que já roda o build otimizado e pega o arquivo `.exe` gerado para criar uma página de "Release" (Lançamento) no seu GitHub.

Finalize o arquivo adicionando este último bloco:

```yaml
      # 4. Raciocínio (Build e Distribuição): A automação oficial do Tauri
      - name: Build do Instalador Tauri
        uses: tauri-apps/tauri-action@v0
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }} # Permissão para o robô criar o link de download
        with:
          tagName: app-v__VERSION__ # Pega a versão escrita no tauri.conf.json
          releaseName: 'To-Do List v__VERSION__'
          releaseBody: 'Nova versão compilada automaticamente nos servidores da Microsoft!'
          releaseDraft: true # Cria como rascunho para você revisar antes de publicar
          prerelease: false
```

---

## 4. Como testar essa automação?

Como o GitHub Actions roda nos servidores da nuvem, o comando de teste não é o `npm run`. O comando de teste é simplesmente enviar o seu código para o GitHub!

**No terminal do Codespace, digite os comandos do Git:**

```bash
git add .
git commit -m "Adiciona pipeline de CI/CD para compilar o executavel do Windows"
git push origin main
```

### O que mostrar para os seus alunos:

1.  Peça para eles abrirem a página do repositório deles no site do **GitHub**.
2.  Mandem clicar na aba superior chamada **"Actions"**.
3.  Eles verão uma bolinha amarela girando com o nome do commit: *"Adiciona pipeline de CI/CD..."*.
4.  Clique nela! Eles poderão ver o terminal do computador Windows, ao vivo, rodando o `npm install`, baixando o Rust e espremendo o binário.
5.  Quando ficar verde (sucesso), mande eles voltarem para a página inicial do repositório e olharem no canto direito, na seção **"Releases"**.
6.  Lá estará o rascunho (Draft) do lançamento. Ao abrir, o arquivo **`todolist_1.0.0_x64-setup.exe`** estará lá, pronto para ser baixado por qualquer pessoa no mundo e ser instalado nativamente, baixando o WebView2 automaticamente conforme configuramos antes!

---

Essa é a cereja do bolo. Você não apenas ensinou a criar um aplicativo desktop hiper-leve, seguro e com banco de dados embutido, como também entregou o fluxo profissional de distribuição que as grandes empresas (como Discord e 1Password, que usam Rust) utilizam! 




