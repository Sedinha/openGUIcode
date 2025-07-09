# Scripts de Automação (`scripts/`)

Este diretório contém scripts de automação usados para facilitar o processo de desenvolvimento, build e empacotamento da aplicação **openGUIcode (Claudia)**. Esses scripts são escritos em JavaScript/TypeScript e executados com [Bun](https://bun.sh/).

## Propósito

O objetivo principal deste diretório é automatizar tarefas repetitivas e complexas, garantindo consistência e reduzindo a chance de erro humano. Eles são invocados através dos comandos definidos na seção `scripts` do arquivo `package.json`.

## Scripts Notáveis

### `fetch-and-build.js`

Este é o script mais importante do diretório. Ele é responsável por baixar uma versão específica do `claude-code` (a CLI original) da npm, extrair os binários necessários (como `ripgrep`) e, em seguida, compilar o código do `claude-code` em um executável nativo para a plataforma de destino.

**Funcionalidades:**
-   Baixa e descompacta pacotes npm.
-   Copia seletivamente os binários e arquivos necessários.
-   Usa o `bun build --compile` para criar executáveis nativos para diferentes plataformas (Linux, macOS, Windows).
-   É usado nos hooks `predev` e `prebuild` do `package.json` para garantir que os binários necessários estejam disponíveis antes de rodar ou construir a aplicação Tauri.

### `build-executables.js`

Um script auxiliar, geralmente chamado por `fetch-and-build.js`, que contém a lógica específica para compilar os executáveis para diferentes alvos (ex: `x86_64-unknown-linux-gnu`, `aarch64-apple-darwin`).

### `prepare-bundle-native.js`

Este script prepara o código-fonte do `claude-code` para ser empacotado em um executável nativo. Ele pode realizar tarefas como:
-   Modificar caminhos de arquivos para usar assets embutidos.
-   Injetar código para carregar binários (como `ripgrep`) que foram empacotados dentro do executável principal.

## Como Usar

Os scripts não são projetados para serem chamados diretamente. Em vez disso, eles são executados através dos comandos do `package.json`.

**Exemplos de Comandos:**

-   `bun run build`: Este comando, antes de executar o `tsc && vite build`, primeiro roda o `prebuild`, que por sua vez chama `bun run build:executables:current`. Isso garante que os binários corretos para a sua plataforma estejam prontos antes do build final do Tauri.
-   `bun run dev`: Da mesma forma, o `predev` garante que os binários estejam prontos antes de iniciar o servidor de desenvolvimento.

Esses scripts são uma parte crucial da pipeline de build, abstraindo a complexidade de empacotar uma aplicação que depende de binários externos e de um runtime de JavaScript como o Bun.
