name: "PRP - Integração OpenCode no Claudia"
description: |
  Template otimizado para agentes de IA implementarem recursos com contexto suficiente e capacidades de auto-validação para alcançar código funcional através de refinamento iterativo.

#### Purpose
  Este PRP tem como objetivo guiar a integração do OpenCode como o novo backend de inteligência artificial (IA) para o Claudia, substituindo a comunicação direta do Claudia com o Claude Code CLI para as operações de IA e execução de ferramentas. O objetivo é alavancar a modularidade e flexibilidade do OpenCode, mantendo a experiência de usuário visual do Claudia.

#### Core Principles
1.  **Context is King**: Incluir TODA a documentação necessária, exemplos e ressalvas [1-3].
2.  **Validation Loops**: Fornecer testes/linters executáveis que a IA pode rodar e corrigir [1, 2, 4].
3.  **Information Dense**: Usar palavras-chave e padrões da base de código [1, 2].
4.  **Progressive Success**: Começar simples, validar, depois aprimorar [1, 2].

--------------------------------------------------------------------------------

#### Goal
Integrar o OpenCode como o **motor de IA principal** do Claudia, roteando as interações da UI do Claudia (e.g., chat, execução de agentes) através do servidor local do OpenCode, em vez de diretamente para o Claude Code CLI. Isso visa aprimorar a flexibilidade e modularidade do sistema de IA subjacente ao Claudia [5, 6].

#### Why
*   **Maior Controle e Flexibilidade da IA:** O OpenCode oferece um sistema de `Providers` e `Tools` mais transparente e programável, permitindo maior controle sobre quais LLMs usar e quais capacidades de ferramentas o agente pode ter, em contraste com a interação direta com o `claude CLI` [5-7].
*   **Modularidade e Separação de Preocupações:** Reforça a separação entre a camada de UI (Claudia, em Tauri/React/Rust) e a camada de lógica central de IA (OpenCode Server, em Bun/TypeScript), comunicando-se via uma API bem definida (HTTP/SSE) [5, 8].
*   **Base para Evoluções Futuras:** A arquitetura do OpenCode é projetada para extensibilidade, o que é crucial para futuras capacidades de IA e integração com sistemas de orquestração mais avançados como o `Mastra` (embora fora do escopo deste PRP, é uma consideração estratégica) [5, 9].
*   **Centralização da Lógica de IA:** Unifica a gestão de sessões, mensagens e ferramentas sob o controle do OpenCode, permitindo uma experiência de IA mais coesa [10, 11].

#### What
Implementar a comunicação entre o backend Rust do Claudia e o servidor local do OpenCode (Bun/TypeScript), permitindo que as principais funcionalidades de interação com a IA do Claudia sejam processadas pelo OpenCode.

##### Success Criteria
*   **Conexão Estabelecida:** O Claudia consegue iniciar (ou se conectar a um já existente) e se comunicar com o servidor OpenCode localmente via HTTP [5, 12].
*   **Chat Básico Funcional:** Mensagens de texto enviadas pelo usuário na interface do Claudia são processadas pelo OpenCode (`/session_chat` endpoint) e as respostas do AI (via `Providers` do OpenCode) são recebidas e exibidas em tempo real na GUI do Claudia (via stream SSE do `/event` endpoint) [5, 13-15].
*   **Visualização de Ferramentas:** O output de ferramentas básicas do OpenCode (e.g., `ReadTool`, `BashTool`) orquestradas pelo OpenCode é corretamente exibido na GUI do Claudia [5, 16, 17].
*   **Persistência de Sessões:** As sessões e mensagens criadas via Claudia são persistidas pelo `Storage` do OpenCode e o Claudia pode visualizá-las no histórico (`Session History`) [5, 18-20].
*   **Gerenciamento da Dependência no `claude CLI`:** A integração deve substituir explicitamente as chamadas de IA do `claude CLI` pelo OpenCode. A existência do `claude CLI` como um pré-requisito de instalação para o Claudia será documentada, mas a lógica de interação com LLMs será via OpenCode.

#### All Needed Context
##### Documentation & References (list all context needed to implement the feature)
**PRPs-agentic-eng Framework (Referências detalhadas):**
*   `docfile: PRPs/ai_docs/01_claude_code_commands_.md` — Comandos customizados e arquitetura de comandos do Claude Code.
*   `docfile: PRPs/ai_docs/02_prp_execution__running_a_prp__.md` — Execução de PRPs: fluxo, etapas e automação.
*   `docfile: PRPs/ai_docs/03_prp__product_requirement_prompt__.md` — Conceito, estrutura e propósito do PRP.
*   `docfile: PRPs/ai_docs/04_validation_loops_.md.md` — Loops de Validação: auto-correção e validação iterativa.
*   `docfile: PRPs/ai_docs/05_prp_creation__generating_a_prp__.md` — Geração de PRPs via agentes de criação.
*   `docfile: PRPs/ai_docs/06_prp_templates_.md` — Templates padronizados de PRP.
*   `docfile: PRPs/ai_docs/07_codebase_context__ai_documentation__.md` — Engenharia de contexto: documentação e referências para IA.
*   `docfile: PRPs/ai_docs/08_claude_code_platform_.md` — Plataforma Claude Code: engine, ferramentas e integração.
*   `docfile: PRPs/ai_docs/09_parallel_agentic_execution_.md` — Execução paralela de agentes e estratégias avançadas.
 

**Claudia (Frontend/Rust Backend):**
*   `docfile: PRPs/ai_docs/Overview Claudia.md`
*   `docfile: PRPs/ai_docs/claudia-1.Session-Project Management.md`
*   `docfile: PRPs/ai_docs/claudia-2.Agents.md`
*   `docfile: PRPs/ai_docs/claudia-3.Frontend UI Components.md`
*   `docfile: PRPs/ai_docs/claudia-4.Tauri Commands.md`
*   `docfile: PRPs/ai_docs/claudia-5.Claude CLI interaction.md`
*   `docfile: PRPs/ai_docs/claudia-6.Sandboxing.md`
*   `docfile: PRPs/ai_docs/claudia-7.Streamed Output Processing.md`
*   `docfile: PRPs/ai_docs/claudia-8.Process Registry.md`
*   `docfile: PRPs/ai_docs/claudia-9.Checkpoiting.md`
*   `docfile: PRPs/ai_docs/claudia-10.MCP.md`
    *   docfile: PRPs/ai_docs/claude_code_features.md (Features específicas do Claude Code relevantes para o Claudia)
**OpenCode (Backend/TypeScript):**
*   `docfile: PRPs/ai_docs/Overview OpenCode.md`
*   `docfile: PRPs/ai_docs/opencode-1.TUI.md`
*   `docfile: PRPs/ai_docs/opencode-2.Message.md`
*   `docfile: PRPs/ai_docs/opencode-3.Session.md`
*   `docfile: PRPs/ai_docs/opencode-4.Config.md`
*   `docfile: PRPs/ai_docs/opencode-5.Provider.md`
*   `docfile: PRPs/ai_docs/opencode-6.Tool.md`
*   `docfile: PRPs/ai_docs/opencode-7.Storage.md`
*   `docfile: PRPs/ai_docs/opencode-8.Server.md`
*   `docfile: PRPs/ai_docs/opencode-8.2 API Server.md`
*   `docfile: PRPs/ai_docs/opencode-9.Bus.md`
*   `docfile: PRPs/ai_docs/opencode-10.App Context.md`
*   `docfile: PRPs/ai_docs/opencode-11.Share Feature.md`
*   `docfile: PRPs/ai_docs/opencode-12.LSP Integration.md`
    *   OpenCode repository: [OpenCode](../opencode/)(on this root directory exists opencode repository)  
*   `docfile: PRPs/ai_docs/01_claude_code_commands_.md`
**Tauri 2.0 (Rust):**
*   `docfile: PRPs/ai_docs/tauri-index.md`
*   `docfile: PRPs/ai_docs/tauri-01_tauri_command_line_interface__cli__.md`
*   `docfile: PRPs/ai_docs/tauri-02_configuration_system__tauri_conf_json__.md`
*   `docfile: PRPs/ai_docs/tauri-03_inter_process_communication__ipc____commands_.md`
*   `docfile: PRPs/ai_docs/tauri-04_javascript_api___tauri_apps_api__.md`
*   `docfile: PRPs/ai_docs/tauri-05_application_builder_.md`
*   `docfile: PRPs/ai_docs/tauri-06_access_control_list__acl____capabilities_.md`
*   `docfile: PRPs/ai_docs/tauri-07_application_bundler_.md`
*   `docfile: PRPs/ai_docs/tauri-08_runtime_abstraction_.md`



##### Current Codebase tree (run `ls -R` in the root of the project) to get an overview of the codebase
`!git ls-files`
*   *Analogia:* O agente deve considerar a estrutura de arquivos e módulos existentes em ambos os projetos (Claudia em Rust/Tauri e OpenCode em TypeScript/Bun) para planejar a integração.

##### Desired Codebase tree with files to be added and responsibility of file
*   `src-tauri/src/opencode_integration.rs`: Novo módulo Rust para gerenciar o processo do OpenCode Server e a comunicação HTTP/SSE.
*   `src-tauri/src/commands/opencode.rs`: Comandos Tauri para o frontend Claudia interagir com o novo módulo `opencode_integration.rs`.
*   `src-tauri/src/main.rs`: Atualizar `tauri::generate_handler!` para incluir novos comandos.
*   `src-tauri/src/app_config.rs` (se aplicável): Configurações para a porta do OpenCode Server ou caminhos.
*   `packages/opencode/src/server/server.ts`: Assegurar que o servidor OpenCode pode ser iniciado programaticamente e que seus endpoints são acessíveis.

##### Known Gotchas of our codebase & Library Quirks
*   **Orquestração de Processos:** O Claudia atualmente spawna o `claude CLI` como um processo filho [21, 22]. A nova abordagem exigirá que o backend Rust do Claudia inicie e monitore o servidor OpenCode (um processo Node.js/Bun) [5, 23]. A gestão do ciclo de vida (início, parada, monitoramento) do servidor OpenCode deve ser robusta.
*   **Comunicação Inter-Processos:** A tradução entre a comunicação de processos nativa do Rust e as requisições HTTP/SSE do OpenCode Server [5]. A desserialização/serialização de dados JSON entre Rust e TypeScript (garantir compatibilidade `serde` em Rust com schemas Zod em TypeScript) [24, 25].
*   **Gerenciamento de Fluxo de Output:** O Claudia processa `streamed output` do `claude CLI` em formato JSONL [26, 27]. O OpenCode Server também envia streams de eventos (SSE) [15]. A adaptação da UI do Claudia para processar e exibir esses novos streams de eventos do OpenCode é crucial.
*   **Persistência de Dados de Sessão:** O Claudia tem um sistema próprio para `Session/Project Management` e `Checkpointing` baseado em arquivos JSONL no diretório `~/.claude` [19, 28, 29]. O OpenCode também tem seu próprio `Storage` de sessões e mensagens (`session/info` e `session/message` como arquivos JSON) [18, 20]. Será necessário decidir se o Claudia adaptará sua lógica de UI para ler diretamente o `Storage` do OpenCode, ou se o OpenCode fornecerá endpoints para que o Claudia continue usando sua própria estrutura de gerenciamento de sessões com dados "proxy". Para este PRP, focaremos na adaptação do Claudia para interagir com o `Storage` do OpenCode.
*   **Transição de Funcionalidades do `claude CLI`:** O `claude CLI` oferece funcionalidades como `Custom Commands` [30, 31], `Agents` com `System Prompts` [32], e `MCP` [33]. Como essas funcionalidades serão espelhadas ou substituídas pela flexibilidade do OpenCode (`Tools`, `Providers`, `Config`) [6, 7, 34]?

#### Implementation Blueprint
##### Data models and structure
*   **Rust (Claudia):** Adaptar estruturas de dados existentes ou criar novas para representar a comunicação com o OpenCode (e.g., structs para requests/responses HTTP/SSE que mapeiem para `Session.Info` e `Message.Info` do OpenCode).
*   **TypeScript (OpenCode):** Os modelos existentes do OpenCode para `Message`, `Session`, `ToolInvocationPart` [25, 35, 36] serão a fonte da verdade para o backend do Claudia.

##### list of tasks to be completed to fullfill the PRP in the order they should be completed
1.  **Estudo e Adaptação do Módulo de Interação CLI do Claudia:**
    *   Analisar `src-tauri/src/commands/claude.rs` e `claudia-5.Claude CLI interaction.md` para entender como o Claudia spawna e interage com o `claude CLI` [22, 37].
    *   Identificar os pontos de entrada onde as chamadas ao `claude CLI` ocorrem (`execute_agent`, `execute_claude_code`, `continue_claude_code`, `resume_claude_code`) [38].
2.  **Desenvolvimento do Módulo de Integração OpenCode no Claudia (Rust):**
    *   Criar um novo módulo Rust (`src-tauri/src/opencode_integration.rs`) responsável por:
        *   **Iniciar o OpenCode Server:** Lançar o processo Node.js/Bun do OpenCode Server (`packages/opencode/src/index.ts` ou `packages/opencode/src/server/server.ts`) como um processo filho [23]. Capturar a porta em que o servidor está escutando [39].
        *   **Requisições HTTP para OpenCode:** Implementar chamadas HTTP assíncronas (e.g., usando `reqwest` crate) para os endpoints REST do OpenCode Server (e.g., `/session_chat`, `/session_list`) [5, 14, 40].
        *   **Consumo de Stream SSE:** Implementar um cliente SSE (Server-Sent Events) em Rust para se conectar ao endpoint `/event` do OpenCode [5, 15].
        *   **Re-emissão de Eventos Tauri:** Receber os eventos do OpenCode via SSE e re-emitir para o frontend React do Claudia via `Tauri Events` (e.g., `agent-output`, `claude-output`, `agent-error`) [5, 27].
3.  **Adaptação dos Comandos Tauri Existentes:**
    *   Modificar as funções `execute_agent` e `execute_claude_code` no backend Rust do Claudia (`src-tauri/src/commands/claude.rs` ou um novo arquivo `opencode.rs` se a separação for maior) para que, em vez de spawnar o `claude CLI`, elas usem as novas funcionalidades do `opencode_integration.rs` para interagir com o OpenCode Server.
4.  **Adaptação da UI do Claudia (React/TypeScript):**
    *   Ajustar os componentes de frontend (`AgentExecution.tsx`, `ClaudeCodeSession.tsx`, `StreamMessage.tsx`) para processar os novos formatos de eventos (`Message.Info` e `Message.Part`) do OpenCode recebidos via `Tauri Events` [25, 41, 42].
    *   Atualizar a lógica de `Session/Project Management` para usar as chamadas do OpenCode Server para listar e obter dados de sessões (`/session_list`, `/session_messages`) [40].
5.  **Mapeamento de Funcionalidades de Agentes:**
    *   Para o `System Prompt` dos `Agents` do Claudia [32], mapear para a entrada do `Session.chat` do OpenCode.
    *   Para a seleção de `Model` nos `Agents` do Claudia [43], mapear para a configuração de `Provider`/`Model` no `Config` do OpenCode [44, 45].
6.  **Revisão e Adaptação de `Sandboxing`:**
    *   O `Sandboxing` do Claudia [46] é projetado para o `claude CLI`. Reavaliar como aplicar restrições de segurança ao processo do servidor OpenCode (e.g., limitações de rede ou filesystem para o processo `bun` ou `node`) [47].

##### Per task pseudocode as needed added to each task
*   **Iniciar OpenCode Server (Rust `opencode_integration.rs`):**
    ```rust
    fn start_opencode_server(app_handle: AppHandle) -> Result<OpenCodeServerInfo, String> {
        // Encontrar o executável do Bun/Node
        // Construir o comando para `bun run packages/opencode/src/index.ts`
        // Configurar stdout/stderr pipes
        // Spawnar processo filho (OpenCode Server)
        // Ler stdout para capturar a porta do servidor
        // Emitir evento Tauri para o frontend com a porta e status
        // Registrar PID no Process Registry do Claudia (se relevante)
    }
    ```
*   **Enviar Mensagem (Rust `commands/opencode.rs` via `opencode_integration.rs`):**
    ```rust
    #[tauri::command]
    async fn send_message_to_opencode(session_id: String, message_text: String) -> Result<(), String> {
        // Obter URL do OpenCode Server do estado da aplicação
        // Construir payload JSON para POST para `/{session_id}/chat` com message_text
        // Fazer requisição HTTP POST assíncrona
        // Lidar com resposta (sucesso/erro)
    }
    ```
*   **Consumir Eventos (Rust `opencode_integration.rs`):**
    ```rust
    async fn listen_to_opencode_events(server_url: String, app_handle: AppHandle) {
        // Conectar via HTTP/2 (para SSE) ao `server_url/event`
        // Loop: ler linha por linha do stream SSE
        // Para cada linha:
            // Parsear JSON do evento OpenCode (e.g., Message.Event.PartUpdated)
            // Mapear para um tipo de evento Claudia/Tauri
            // app_handle.emit("opencode-output", payload_mapeado)
    }
    ```
*   **Processar Mensagens na UI (TypeScript `AgentExecution.tsx`):**
    ```typescript
    useEffect(() => {
        const unlisten = listen<string>("opencode-output", (event) => {
            const opencodePayload = JSON.parse(event.payload);
            // Lógica para mapear opencodePayload para Message.Info do Claudia/UI
            // Atualizar estado `setMessages` com a nova mensagem/parte
        });
        return () => { unlisten(); };
    }, []);
    ```

##### Integration Points
*   **`src-tauri/src/commands/claude.rs` (ou novo `opencode.rs`):** Ponto de modificação principal para desviar chamadas do `claude CLI` para o OpenCode Server.
*   **`src-tauri/src/main.rs`:** Registro de novos comandos Tauri.
*   **`src-tauri/src/process/registry.rs`:** Adaptar o `Process Registry` para monitorar o processo do OpenCode Server em vez do `claude CLI` [48].
*   **`src/components/AgentExecution.tsx`, `src/components/ClaudeCodeSession.tsx`:** Atualizar `useEffect` hooks para escutar novos eventos Tauri do OpenCode [49].
*   **`src/lib/api.ts`:** Criar wrappers para chamadas HTTP ao OpenCode Server.
*   **OpenCode Server (`packages/opencode/src/server/server.ts`):** Assegurar que os endpoints necessários (`/session_chat`, `/event`, `/session_list`, `/session_messages`) estão configurados e funcionais [40].
*   **OpenCode `Storage` (`packages/opencode/src/storage/storage.ts`):** Entender como as sessões e mensagens são persistidas no disco para que o Claudia possa lê-las [50].

#### Validation Loop
##### Level 1: Syntax & Style
```bash
cargo check --workspace --verbose # Para o projeto Rust do Claudia
cargo clippy --workspace --all-targets --all-features # Análise estática do Rust
bun check # Para o projeto TypeScript do OpenCode
bun fmt # Formatação de código TypeScript
```
Level 2: Unit Tests each new feature/file/function use existing test patterns
# Para o novo módulo de integração OpenCode em Rust
cargo test --package claudia --test opencode_integration_tests
# Para os testes unitários de componentes específicos do OpenCode
bun test packages/opencode/src/session/
bun test packages/opencode/src/tool/
Level 3: Integration Test
# Iniciar o Claudia
npx tauri dev
# Abrir terminal separado e verificar se o processo do OpenCode Server está rodando
# Fazer teste manual na GUI do Claudia:
# 1. Clicar em "Nova Sessão" (ou similar)
# 2. Digitar "Olá, OpenCode!" e enviar
# 3. Verificar se a resposta da IA aparece na tela e se o output de Tools é exibido (e.g. ListTool, ReadTool)
# 4. Verificar se a sessão é salva e visível na Session History do Claudia após reiniciar o app.

*   **Context Engineering:**
    *   docfile: PRPs/ai_docs/context_engineering.md [671-696] (Importância da Engenharia de Contexto em projetos de IA)

# Final validation Checklist
•
[] O Claudia inicia o processo do OpenCode Server e se conecta a ele sem erros visuais.
•
[] O fluxo básico de chat (envio de texto do usuário, recebimento de resposta da IA) funciona fluidamente na GUI do Claudia, com a lógica de IA vindo do OpenCode.
•
[] O output de Tools do OpenCode (e.g., ReadTool, BashTool, GrepTool) é corretamente parseado e renderizado na interface do Claudia.
•
[] As sessões e mensagens criadas através do Claudia são persistidas pelo OpenCode Storage e são visíveis na funcionalidade de Session History do Claudia.
•
[] A interação com o claude CLI para operações de IA é completamente desativada ou refeita para usar o OpenCode, conforme o escopo.
•
[] Testes de integração (Tauri-OpenCode Server) passam consistentemente.
•
[] A experiência do usuário para as funcionalidades integradas é fluida e intuitiva.
•
[] O consumo de recursos (CPU/RAM) dos processos Claudia e OpenCode combinados é aceitável durante o uso.

---

# Anti-Patterns to Avoid
•
❌ Não usar funções síncronas em contexto assíncrono: Evitar bloqueios de thread, especialmente na comunicação entre Rust e TypeScript.
•
❌ Não ignorar testes falhos: Corrigir imediatamente quaisquer testes unitários ou de integração que falhem.
•
❌ Não hardcodar valores que deveriam ser configuráveis: Incluir portas de servidor, caminhos de arquivo ou chaves de API em arquivos de configuração (tauri.conf.json, opencode.json).
•
❌ Não permitir Context Poisoning, Distraction, Confusion, ou Clash: Embora seja mais relevante para o conteúdo do prompt, assegurar que o contexto passado à IA via OpenCode seja relevante e não sobrecarregado.
•
❌ Não reinventar a roda para funcionalidades já existentes: Utilizar os sistemas robustos de Storage, Session, Tool e Provider do OpenCode, em vez de recriá-los no Claudia.
