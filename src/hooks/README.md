# React Hooks (`src/hooks/`)

Este diretório abriga os hooks [React](https://react.dev/reference/react/hooks) customizados, que são uma parte fundamental da arquitetura de frontend do **openGUIcode (Claudia)**. Os hooks são usados para extrair e reutilizar a lógica de estado e de ciclo de vida dos componentes, tornando o código dos componentes mais limpo, declarativo e fácil de manter.

## Propósito

O principal objetivo dos hooks neste diretório é gerenciar o estado complexo e as interações com o backend de forma isolada. Em vez de cada componente ter que lidar diretamente com chamadas de API, gerenciamento de estado de carregamento, tratamento de erros e escuta de eventos, essa lógica é encapsulada dentro de um hook.

## Hooks Notáveis

### `useOpenCode.ts`

Este é o hook mais importante e complexo do projeto. Ele serve como o principal ponto de contato para qualquer componente que precise interagir com o **OpenCode Server**.

**Responsabilidades:**

1.  **Gerenciamento do Servidor**: Inicia, para e monitora o estado do processo do OpenCode Server no backend.
2.  **Gerenciamento de Sessão**: Cria novas sessões de chat, carrega sessões existentes e busca o histórico de mensagens.
3.  **Comunicação em Tempo Real**:
    - Envia mensagens do usuário para o servidor.
    - Escuta eventos em tempo real (Server-Sent Events - SSE) que vêm do backend para receber atualizações de mensagens, invocações de ferramentas e resultados.
4.  **Gerenciamento de Estado**: Mantém o estado da aplicação relacionado ao OpenCode, incluindo:
    - `isServerRunning`: Se o servidor está ativo.
    - `serverInfo`: Informações sobre o servidor (como a porta).
    - `currentSession`: A sessão de chat ativa no momento.
    - `messages`: A lista de mensagens na sessão atual.
    - `isLoading`: Um booleano que indica se uma resposta está sendo aguardada.
    - `error`: Mensagens de erro do servidor ou da sessão.

Ao usar este hook, um componente como `OpenCodeSession.tsx` pode simplesmente desestruturar os valores e funções de que precisa, sem se preocupar com os detalhes da implementação da comunicação com o backend.

**Exemplo de Uso:**

```tsx
import { useOpenCode } from '@/hooks/useOpenCode';

const MyChatComponent = () => {
  const { messages, isLoading, sendMessage } = useOpenCode({ autoStart: true });

  const handleSend = (prompt: string) => {
    sendMessage(prompt, "claude-3-5-sonnet-20241022", "anthropic");
  };

  return (
    <div>
      {messages.map(msg => <div key={msg.id}>{/* Render message */}</div>)}
      {isLoading && <div>Loading...</div>}
      <button onClick={() => handleSend("Olá, mundo!")}>Enviar</button>
    </div>
  );
};
```

Este padrão de design promove uma clara separação de preocupações:
-   **Componentes (`../components/`)**: Focam na apresentação (UI).
-   **Hooks (`src/hooks/`)**: Focam na lógica de estado e comunicação.
-   **API (`../lib/api.ts`)**: Foca na abstração das chamadas diretas ao backend.
