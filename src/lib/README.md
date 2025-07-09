# Libraries & Utilities (`src/lib/`)

Este diretório contém o código de suporte essencial para o frontend do **openGUIcode (Claudia)**. Ele abriga funções utilitárias, a camada de abstração da API que se comunica com o backend e outras lógicas de negócios do lado do cliente que não são específicas de um componente ou hook.

## Propósito

O objetivo do diretório `lib` é fornecer um local centralizado para código reutilizável que forma a base sobre a qual os componentes e hooks são construídos. Isso ajuda a manter o código organizado, promove a reutilização e separa as preocupações de forma clara.

## Arquivos e Módulos Notáveis

### `api.ts`

Este é um dos arquivos mais críticos do frontend. Ele atua como a **camada de cliente da API**, criando uma ponte limpa e fortemente tipada entre o frontend (TypeScript) e o backend (Rust).

**Responsabilidades:**

-   **Abstração de Comandos Tauri**: Ele envolve as chamadas `invoke` do Tauri em funções JavaScript/TypeScript assíncronas e fáceis de usar. Em vez de espalhar `invoke("meu_comando", { arg: 1 })` por todo o código, os componentes chamam `api.meuComando(1)`.
-   **Tipagem**: Ele usa as definições de tipo (muitas vezes compartilhadas ou espelhadas do backend) para garantir que os dados enviados e recebidos do backend sejam consistentes, pegando erros em tempo de compilação.
-   **Ponto Único de Contato**: Centraliza todas as interações com o backend em um único lugar, facilitando a manutenção e a depuração.

**Exemplo:**

```typescript
// Em api.ts
import { invoke } from "@tauri-apps/api/core";
import type { OpenCodeSession } from "./types"; // Supondo que os tipos estão definidos

export const api = {
  async listOpenCodeSessions(): Promise<OpenCodeSession[]> {
    return await invoke("list_opencode_sessions");
  },
  // ... outras funções de API
};

// Em um componente React
import { api } from "@/lib/api";

function SessionList() {
  useEffect(() => {
    api.listOpenCodeSessions().then(sessions => {
      // ... fazer algo com as sessões
    });
  }, []);
  // ...
}
```

### `utils.ts`

Este arquivo contém funções utilitárias genéricas que podem ser usadas em qualquer parte da aplicação. Exemplos incluem:

-   Formatação de datas e horas.
-   Funções de manipulação de strings.
-   Cálculos ou transformações de dados comuns.
-   A função `cn` (de `clsx` e `tailwind-merge`) para construir nomes de classes CSS de forma condicional e inteligente.

### `claudeSyntaxTheme.ts`

Define um tema de cores customizado para o realce de sintaxe em blocos de código, garantindo uma aparência consistente com a identidade visual da aplicação.

### `linkDetector.tsx`

Um utilitário que pode analisar texto e detectar URLs, transformando-as em links clicáveis, o que é útil para renderizar as saídas do modelo de IA.

## Conclusão

O diretório `src/lib` é a espinha dorsal do frontend. Ele fornece as ferramentas, abstrações e utilitários necessários para que os [componentes](../components/README.md) e [hooks](../hooks/README.md) possam focar em suas responsabilidades principais sem se preocupar com os detalhes de baixo nível da comunicação com o backend ou com a lógica de negócios repetitiva.
