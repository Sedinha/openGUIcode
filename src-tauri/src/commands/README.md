# Tauri Commands (`src-tauri/src/commands/`)

Este diretório é a ponte de comunicação (IPC - Inter-Process Communication) entre o frontend e o backend da aplicação **openGUIcode (Claudia)**. Cada módulo Rust neste diretório define um conjunto de **Comandos Tauri**, que são funções Rust que podem ser chamadas diretamente pelo código JavaScript/TypeScript do frontend.

## Propósito

A principal função deste diretório é expor a lógica de negócios do backend de forma segura e controlada para a interface do usuário. Em vez de o frontend ter acesso direto ao sistema de arquivos ou a processos, ele deve "invocar" um comando pré-aprovado no backend para realizar essas ações.

Isso cria uma fronteira de segurança clara:
-   **Frontend**: Lida com a apresentação e a interação do usuário.
-   **Backend (Comandos)**: Lida com a lógica sensível e o acesso a recursos do sistema.

## Como Funciona

1.  **Definição do Comando**: Uma função Rust pública é definida e anotada com o macro `#[tauri::command]`. Isso a marca como um comando que pode ser exposto ao frontend.

    ```rust
    // Exemplo de um comando
    #[tauri::command]
    async fn get_user_name(name: String) -> Result<String, String> {
        if name.is_empty() {
            Err("Name cannot be empty".to_string())
        } else {
            Ok(format!("Hello, {}!", name))
        }
    }
    ```

2.  **Registro do Comando**: Para que o comando seja acessível, ele deve ser registrado no `invoke_handler` dentro do `main.rs`.

    ```rust
    // Em src-tauri/src/main.rs
    fn main() {
        tauri::Builder::default()
            .invoke_handler(tauri::generate_handler![
                // ... outros comandos
                get_user_name
            ])
            // ...
            .run(...)
    }
    ```

3.  **Invocação pelo Frontend**: O frontend usa a função `invoke` do pacote `@tauri-apps/api/core` para chamar o comando.

    ```typescript
    // Em um arquivo .ts ou .tsx
    import { invoke } from "@tauri-apps/api/core";

    async function greet() {
        try {
            const result = await invoke("get_user_name", { name: "World" });
            console.log(result); // "Hello, World!"
        } catch (error) {
            console.error(error);
        }
    }
    ```

## Módulos de Comandos

-   **`opencode.rs`**: Contém todos os comandos relacionados à interação com o **OpenCode Server**. Isso inclui iniciar/parar o servidor, criar sessões, enviar mensagens, etc.
-   **`claude.rs`**: Comandos legados ou relacionados à interação direta com o Claude CLI (se aplicável).
-   **`agents.rs`**: Comandos para gerenciar os **Claudia Agents**, como criar, listar, executar e deletar agentes.
-   **`checkpoint.rs`**: Comandos para interagir com o sistema de **Checkpointing**, como criar e restaurar checkpoints.
-   Outros módulos podem ser adicionados para agrupar comandos por funcionalidade (ex: `projects.rs`, `settings.rs`).

## Gerenciamento de Estado

Os comandos Tauri podem acessar o estado compartilhado da aplicação (definido em `main.rs`) através do parâmetro `state: State<'_, MyState>`. Isso permite que os comandos interajam com serviços de longa duração, como o `OpenCodeState`, que gerencia a conexão com o OpenCode Server.

```rust
#[tauri::command]
async fn start_server(state: State<'_, AppState>) -> Result<(), String> {
    // Acessa um serviço no estado gerenciado para iniciar o servidor
    state.opencode_service.start().await;
    Ok(())
}
```
