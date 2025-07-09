# Frontend Source (`src/`)

Este diretório contém todo o código-fonte do frontend da aplicação **openGUIcode (Claudia)**. Ele é construído com [React](https://react.dev/) e [TypeScript](https://www.typescriptlang.org/), utilizando [Vite](https://vitejs.dev/) como ferramenta de build.

## Visão Geral

O frontend é responsável por toda a interface do usuário com a qual o usuário interage. Ele se comunica com o backend Rust através da ponte IPC do Tauri para executar ações, gerenciar o estado da aplicação e exibir dados.

## Estrutura de Diretórios

- **`components/`**: Contém todos os componentes React reutilizáveis. Estes são os blocos de construção da UI, como botões, inputs, caixas de diálogo e visualizadores de dados.
  - `ui/`: Componentes de UI genéricos, baseados no `shadcn/ui`.
  - Componentes específicos da aplicação (ex: `AgentExecution.tsx`, `OpenCodeSession.tsx`).

- **`hooks/`**: Contém hooks React customizados que encapsulam lógica de estado complexa e interações com o backend. Um exemplo chave é o `useOpenCode.ts`, que gerencia a conexão e o estado do OpenCode Server.

- **`lib/`**: Contém bibliotecas de suporte, utilitários e a camada de abstração da API.
  - `api.ts`: Define uma camada de cliente para interagir com os [Comandos Tauri](../src-tauri/src/commands/README.md), fornecendo uma maneira limpa e tipada para o frontend chamar o backend.
  - `utils.ts`: Funções utilitárias gerais usadas em toda a aplicação.

- **`assets/`**: Arquivos estáticos como imagens, ícones e folhas de estilo CSS.

## Arquivos Principais

- **`main.tsx`**: O ponto de entrada da aplicação React. Ele é responsável por renderizar o componente raiz (`App.tsx`) no DOM.

- **`App.tsx`**: O componente principal da aplicação. Ele gerencia o roteamento, o layout geral e a renderização dos diferentes "cards" ou seções da aplicação, como `CCAgents` e `OpenCodeSession`.

- **`styles.css`**: Folha de estilo global, principalmente para configurar o [Tailwind CSS](https://tailwindcss.com/).

## Fluxo de Dados

1.  **Renderização da UI**: O React renderiza os componentes definidos em `src/components/`.
2.  **Ação do Usuário**: O usuário clica em um botão ou interage com um elemento da UI.
3.  **Chamada de API**: O manipulador de eventos no componente chama uma função do `src/lib/api.ts`.
4.  **Invocação do Comando**: A função da API usa o `invoke` do Tauri para chamar um comando no backend Rust.
5.  **Atualização de Estado**: O backend retorna dados, que são usados para atualizar o estado da aplicação (geralmente através de um hook de `src/hooks/`) e re-renderizar a UI com as novas informações.
