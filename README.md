# openGUIcode

**openGUIcode** é um ambiente de desenvolvimento integrado (IDE) de desktop, construído com Tauri, React e Rust, projetado para facilitar o desenvolvimento de software assistido por IA. Ele combina uma interface de usuário moderna e reativa com um poderoso backend de IA, permitindo que os desenvolvedores interajam com agentes de IA especializados para realizar tarefas complexas de codificação, análise e automação.

O projeto é estruturado em torno de um **Product Requirement Prompt (PRP)**, uma metodologia que guia os agentes de IA através de "Validation Loops" para garantir que o trabalho seja concluído de forma correta e iterativa.

## Estrutura do Projeto

O repositório está organizado da seguinte forma:

- **`/` (Raiz)**: Contém a configuração principal do projeto, incluindo `package.json` para dependências do Node.js, `vite.config.ts` para o build do frontend e `tsconfig.json` para a configuração do TypeScript.
- **`src/`**: O código-fonte do frontend da aplicação, escrito em React e TypeScript.
  - **`src/components/`**: Componentes React reutilizáveis que compõem a interface do usuário.
  - **`src/hooks/`**: Hooks React customizados que encapsulam a lógica de estado e efeitos.
  - **`src/lib/`**: Funções utilitárias, definições de API e lógica de negócios do lado do cliente.
- **`src-tauri/`**: O código-fonte do backend da aplicação, escrito em Rust.
  - **`src-tauri/src/commands/`**: Comandos Tauri que servem como a ponte (IPC) entre o frontend e o backend.
  - **`src-tauri/src/checkpoint/`**: Lógica para o sistema de checkpointing e timelines, permitindo salvar e restaurar o estado do projeto.
  - **`src-tauri/src/opencode_integration.rs`**: O módulo principal para a integração com o OpenCode Server.
- **`opencode/`**: O submódulo contendo o OpenCode Server, que gerencia os provedores de IA e as ferramentas.
- **`PRPs/`**: Contém os Product Requirement Prompts, que são os documentos que guiam a execução das tarefas de IA.
- **`scripts/`**: Scripts de automação para tarefas de build, fetch e outras operações de desenvolvimento.
- **`cc_agents/`**: Definições para os "Claudia Agents", os agentes de IA pré-configurados.

## Conceitos Fundamentais

- **Product Requirement Prompt (PRP)**: Um documento detalhado que serve como uma "ordem de serviço" para um agente de IA, descrevendo o objetivo, o contexto e o plano de implementação.
- **Validation Loops**: Um conjunto de comandos executáveis (testes, linters) dentro de um PRP que a IA usa para validar seu próprio trabalho de forma iterativa.
- **Claudia Agents**: "Personalidades" de IA pré-configuradas para tarefas específicas (ex: "Security Scanner", "Unit Test Writer").
- **OpenCode Server**: Um servidor local que atua como o motor da IA, gerenciando a comunicação com diferentes modelos de IA (Providers) e as ações que eles podem executar (Tools).
- **Tauri Commands**: A ponte de comunicação segura entre o frontend (JavaScript) e o backend (Rust).
- **Checkpointing**: Um sistema de "versionamento" para as sessões de IA, que salva o estado dos arquivos e do histórico de conversas, permitindo restaurar ou "forkar" o trabalho.

## Como Começar

1.  **Instale as dependências:**
    ```bash
    bun install
    ```
2.  **Inicie o ambiente de desenvolvimento:**
    ```bash
    bun run tauri dev
    ```

Isso iniciará o frontend em Vite com hot-reload e o backend em Rust, permitindo o desenvolvimento em tempo real.
