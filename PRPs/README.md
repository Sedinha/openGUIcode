# Product Requirement Prompts (`PRPs/`)

Este diretório é o coração da metodologia de **Engenharia de Contexto** e **Desenvolvimento Guiado por IA** do projeto **openGUIcode**. Ele contém os **Product Requirement Prompts (PRPs)**, que são documentos Markdown estruturados que servem como "ordens de serviço" ou "planos de missão" para os agentes de IA.

## Propósito

O objetivo dos PRPs é resolver o problema fundamental de como dar a um agente de IA uma tarefa complexa de forma que ele possa executá-la com sucesso e de forma autônoma. Em vez de dar instruções vagas, um PRP fornece:

1.  **Objetivo Claro**: O que precisa ser construído e por quê.
2.  **Contexto Completo**: Todas as referências de código, documentação e conhecimento prévio necessários para a tarefa.
3.  **Plano de Implementação**: Um plano passo a passo que o agente pode seguir.
4.  **Validação**: Comandos executáveis que o agente usa para testar e corrigir seu próprio trabalho.

## Estrutura de um PRP

Um PRP típico é um arquivo Markdown com seções padronizadas:

-   **`name` e `description`**: Metadados básicos sobre a tarefa.
-   **`Goal`**: Uma descrição de alto nível do objetivo final.
-   **`All Needed Context`**: Uma lista de referências cruciais. Isso pode incluir:
    -   `docfile:`: Links para outros documentos de design ou guias de estilo.
    -   `file:`: Links para arquivos de código-fonte existentes que servem como exemplo.
    -   `glob:`: Padrões de arquivos para dar ao agente uma ideia de onde procurar.
-   **`Implementation Blueprint`**: Uma lista de etapas de alto nível que o agente deve seguir (ex: `CREATE`, `MODIFY`, `TEST`).
-   **`Validation Loop`**: A seção mais importante. Contém uma série de comandos que o agente executa para validar seu trabalho. É tipicamente dividida em níveis:
    -   **Level 1 (Syntax & Style)**: Comandos de linting e formatação (`eslint`, `prettier`).
    -   **Level 2 (Unit Tests)**: Comandos para rodar testes unitários relevantes.
    -   **Level 3 (Integration Tests)**: Comandos para testes de integração mais amplos.
    -   **Level 4 (Build)**: Comandos para garantir que a aplicação ainda compila e constrói corretamente.

O agente executa o código, roda o loop de validação, analisa os erros, corrige o código e repete o ciclo até que todas as validações passem.

## Conteúdo do Diretório

-   **`ai_docs/`**: Contém a documentação gerada pela IA sobre o próprio código-fonte do `openGUIcode`. Estes documentos são usados como contexto para futuras tarefas de IA, criando um ciclo de auto-documentação.
-   **`templates/`**: Contém templates de PRPs que podem ser usados como ponto de partida para criar novas tarefas.
-   **`unificacao-da-ide.md`**: Um exemplo de um PRP completo que guiou a integração do OpenCode Server na aplicação Claudia.
-   **`status.md` e `summary.md`**: Documentos que podem ser usados para rastrear o progresso e resumir os resultados de uma execução de PRP.

Este diretório e a metodologia de PRP são o que eleva o `openGUIcode` de um simples cliente de chat de IA para um verdadeiro ambiente de desenvolvimento assistido por IA, capaz de lidar com tarefas complexas de forma estruturada e autônoma.
