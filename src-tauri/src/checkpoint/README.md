# Checkpointing & Timelines (`src-tauri/src/checkpoint/`)

Este diretório implementa uma das funcionalidades mais poderosas e de segurança do **openGUIcode (Claudia)**: o sistema de **Checkpointing e Timelines**. Ele funciona como um sistema de controle de versão leve e integrado, projetado especificamente para o desenvolvimento assistido por IA.

## Propósito

O objetivo principal deste sistema é fornecer uma **rede de segurança** robusta. Ao permitir que os usuários salvem e restaurem o estado completo de um projeto e de uma sessão de IA, ele encoraja a experimentação e mitiga o risco de alterações destrutivas feitas por um agente de IA.

Funcionalidades chave:
-   **Salvar Estado**: Cria um "checkpoint" que é um snapshot do estado atual de todos os arquivos do projeto e do histórico de conversas da IA.
-   **Restaurar Estado**: Reverte todos os arquivos e o histórico de conversas para um checkpoint anterior.
-   **Histórico Visual**: Mantém uma "timeline" de todos os checkpoints, permitindo ao usuário navegar e entender a evolução do projeto.
-   **Forking**: Permite criar um novo ramo de desenvolvimento a partir de um checkpoint existente, ideal para explorar diferentes abordagens para um problema.

## Estrutura e Componentes

-   **`mod.rs`**: O módulo principal que define as estruturas de dados centrais, como `Checkpoint`, `Timeline` e `FileSnapshot`.
    -   `Checkpoint`: Representa um único ponto de salvamento no tempo, com metadados como ID, timestamp e descrição.
    -   `FileSnapshot`: Representa o conteúdo de um único arquivo em um determinado checkpoint.

-   **`manager.rs`**: O `CheckpointManager` é a orquestra do sistema. Ele lida com a lógica de alto nível:
    -   Determina quais arquivos foram modificados desde o último checkpoint.
    -   Coleta o histórico de conversas atual.
    -   Coordena com o `storage` para salvar ou carregar os dados.

-   **`storage.rs`**: O `CheckpointStorage` é responsável pela interação direta com o sistema de arquivos.
    -   Lê e escreve os dados dos checkpoints em um diretório `.timelines/` dentro do projeto.
    -   Implementa otimizações para economizar espaço, como não duplicar arquivos inalterados entre checkpoints.

-   **`commands.rs`**: (Dentro de `src-tauri/src/commands/`) Expõe a funcionalidade do checkpointing para o frontend através de Comandos Tauri, como `create_checkpoint` e `restore_checkpoint`.

## Fluxo de Trabalho de um "Create Checkpoint"

1.  **Invocação**: O usuário clica no botão "Checkpoint" na UI. O frontend invoca o comando `create_checkpoint`.
2.  **Gerenciamento**: O `CheckpointManager` é ativado.
3.  **Análise de Diff**: O `manager` compara o estado atual dos arquivos do projeto com o último checkpoint para identificar quais arquivos foram adicionados, modificados ou excluídos.
4.  **Criação de Snapshots**: Para cada arquivo alterado, um `FileSnapshot` é criado com o novo conteúdo.
5.  **Coleta de Histórico**: O `manager` obtém o histórico de conversas atual da sessão de IA.
6.  **Persistência**: O `manager` passa o novo `Checkpoint`, a lista de `FileSnapshot`s e o histórico de conversas para o `CheckpointStorage`.
7.  **Escrita em Disco**: O `storage` escreve os dados de forma eficiente no diretório `.timelines/`, criando uma nova entrada na timeline.
8.  **Retorno**: O comando retorna um resultado de sucesso para o frontend, que atualiza a UI para exibir o novo checkpoint na timeline.

Este sistema robusto garante que os usuários possam experimentar com confiança, sabendo que qualquer alteração pode ser desfeita com um único clique, tornando o desenvolvimento com IA mais seguro e produtivo.
