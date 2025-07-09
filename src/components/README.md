# React Components (`src/components/`)

Este diretório é o coração da interface do usuário (UI) da aplicação. Ele contém todos os componentes [React](https://react.dev/) que formam as telas, os controles e os elementos visuais do **openGUIcode (Claudia)**.

## Estrutura e Filosofia

A estrutura de componentes segue uma abordagem modular e reutilizável.

- **`ui/`**: Este subdiretório contém componentes de UI genéricos e de baixo nível, como `Button`, `Input`, `Dialog`, `Tooltip`, etc. Eles são construídos com base na biblioteca [shadcn/ui](https://ui.shadcn.com/), que fornece blocos de construção de UI acessíveis e estilizáveis. Esses componentes são agnósticos em relação à lógica de negócios da aplicação.

- **Componentes de Funcionalidade (Raiz)**: No diretório `src/components/`, você encontrará componentes de nível superior que representam funcionalidades ou telas inteiras da aplicação. Eles compõem a UI usando os componentes básicos de `ui/` e implementam a lógica de negócios específica da aplicação, geralmente utilizando hooks de `../hooks/` e a camada de API de `../lib/api.ts`.

## Componentes Notáveis

- **`OpenCodeSession.tsx`**: Um componente central que renderiza a interface de chat para uma sessão interativa com o OpenCode Server. Ele gerencia a exibição de mensagens, o status do servidor e a entrada do usuário.

- **`AgentExecution.tsx`**: A tela para configurar e executar um [Claudia Agent](../cc_agents/README.md). Permite ao usuário selecionar um projeto, definir uma tarefa e iniciar a execução do agente.

- **`CCAgents.tsx`**: A tela principal para gerenciar os Claudia Agents, permitindo criar, listar e executar agentes.

- **`TimelineNavigator.tsx`**: O componente de UI para interagir com o sistema de [Checkpointing](../src-tauri/src/checkpoint/README.md). Ele exibe o histórico de checkpoints e permite ao usuário criar, restaurar ou "forkar" estados do projeto.

- **`FloatingPromptInput.tsx`**: Um componente de entrada de prompt sofisticado que inclui seleção de modelo, menção de arquivos (`@`), comandos de barra (`/`) e suporte para arrastar e soltar imagens.

- **`StreamMessage.tsx`**: Um componente especializado em renderizar as mensagens que chegam em tempo real (streaming) do backend. Ele sabe como exibir texto, invocações de ferramentas e resultados de ferramentas de forma clara.

## Fluxo de Trabalho Típico

1.  Um **componente de tela** (ex: `OpenCodeSession.tsx`) é renderizado.
2.  Ele usa um **hook customizado** (ex: `useOpenCode` de `../hooks/`) para buscar dados ou se inscrever em eventos do backend.
3.  Ele renderiza **componentes de UI** de `ui/` (como `Button`, `Input`) para construir a interface.
4.  Quando o usuário interage com um componente de UI, o manipulador de eventos no componente de tela chama uma função da **camada de API** (`../lib/api.ts`).
5.  A API invoca um comando no backend, e o resultado é usado para atualizar o estado no hook, o que causa uma nova renderização do componente com os dados atualizados.
