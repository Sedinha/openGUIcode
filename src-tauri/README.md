# Backend Source (`src-tauri/`)

Este diretório contém todo o código-fonte do backend da aplicação **openGUIcode (Claudia)**. Ele é escrito em [Rust](https://www.rust-lang.org/) e utiliza o framework [Tauri](https://tauri.app/) para criar uma aplicação de desktop multiplataforma, segura e performática.

## Visão Geral

O backend é o cérebro da aplicação. Ele é responsável por toda a lógica de negócios pesada, interações com o sistema de arquivos, gerenciamento de processos e comunicação com serviços externos, como o OpenCode Server. Por ser escrito em Rust, ele oferece segurança de memória e alto desempenho, o que é crucial para uma aplicação de desktop robusta.

## Estrutura de Diretórios

- **`src/`**: O diretório principal do código-fonte Rust.
  - **`main.rs`**: O ponto de entrada da aplicação Rust. É aqui que o builder do Tauri é configurado, os comandos são registrados e a aplicação é iniciada.
  - **`commands/`**: Contém os módulos que definem os **Comandos Tauri**. Cada função marcada com `#[tauri::command]` neste diretório pode ser chamada pelo frontend através da ponte IPC.
  - **`checkpoint/`**: Implementa a lógica de **Checkpointing e Timelines**. É responsável por salvar e restaurar snapshots do estado do projeto e do histórico de conversas.
  - **`opencode_integration.rs`**: O módulo central que gerencia a integração com o **OpenCode Server**. Ele lida com a inicialização do processo do servidor, o envio de requisições HTTP e a escuta de eventos em tempo real (SSE).
  - **`process/`**: Utilitários para gerenciamento de processos filhos, usados para executar agentes ou outras tarefas em segundo plano.

- **`capabilities/`**: Define as permissões da aplicação, controlando o que a API do Tauri pode acessar (ex: sistema de arquivos, shell, etc.).

- **`icons/`**: Ícones da aplicação para diferentes plataformas e resoluções.

- **`target/`**: (Gerado durante o build) Contém os artefatos de compilação, incluindo os executáveis finais.

- **`Cargo.toml`**: O arquivo de manifesto do projeto Rust, que define as dependências (crates), metadados e configurações de build.

- **`tauri.conf.json`**: O arquivo de configuração principal do Tauri. Ele define o identificador da aplicação, as configurações de build, as permissões dos plugins e a aparência da janela.

## Fluxo de Trabalho

1.  **Inicialização**: A aplicação é iniciada, e o `main.rs` configura e executa o builder do Tauri.
2.  **Registro de Comandos**: O `invoke_handler` em `main.rs` registra todas as funções de `src/commands/` que estarão disponíveis para o frontend.
3.  **Inicialização de Serviços**: Serviços de fundo, como o `OpenCodeState` (que gerencia a integração com o OpenCode Server), são inicializados e adicionados ao estado gerenciado do Tauri.
4.  **Espera por Invocação**: O backend fica ocioso, esperando que o frontend invoque um comando.
5.  **Execução de Comando**: Quando o frontend chama `invoke("meu_comando")`, o Tauri encontra a função Rust correspondente em `src/commands/` e a executa.
6.  **Retorno de Dados**: A função Rust executa sua lógica (ex: lê um arquivo, faz uma chamada de API) e retorna um `Result`, que o Tauri serializa para JSON e envia de volta para o frontend como uma Promise resolvida ou rejeitada.
