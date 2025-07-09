import React, { useState, useEffect, useRef } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { 
  ArrowLeft,
  Terminal,
  FolderOpen,
  Copy,
  Settings,
  X,
  Command,
  Server,
  Loader2,
  AlertCircle,
  CheckCircle,
  StopCircle,
  Play
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { cn } from "@/lib/utils";
import { open } from "@tauri-apps/plugin-dialog";
import { FloatingPromptInput, type FloatingPromptInputRef } from "./FloatingPromptInput";
import { ErrorBoundary } from "./ErrorBoundary";
import { StreamMessage } from "./StreamMessage";
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from "@/components/ui/tooltip";
import { useOpenCode } from "@/hooks/useOpenCode";
import { type OpenCodeSession, type OpenCodeMessage } from "@/lib/api";

interface OpenCodeSessionProps {
  /**
   * Initial project path
   */
  initialProjectPath?: string;
  /**
   * Callback to go back
   */
  onBack: () => void;
  /**
   * Optional className for styling
   */
  className?: string;
  /**
   * Callback when streaming state changes
   */
  onStreamingChange?: (isStreaming: boolean, sessionId: string | null) => void;
}

/**
 * OpenCodeSession component for interactive OpenCode sessions
 */
export const OpenCodeSession: React.FC<OpenCodeSessionProps> = ({
  initialProjectPath = "",
  onBack,
  className,
  onStreamingChange,
}) => {
  // OpenCode integration
  const {
    serverInfo,
    isServerRunning,
    serverError,
    sessions,
    currentSession,
    messages,
    isLoading,
    startServer,
    stopServer,
    createSession,
    sendMessage,
    loadSession,
    abortSession,
    onEvent,
  } = useOpenCode({ autoStart: true, autoConnect: true });

  // Local state
  const [projectPath, setProjectPath] = useState(initialProjectPath);
  const [error, setError] = useState<string | null>(null);
  const [selectedModel, setSelectedModel] = useState<"sonnet" | "opus">("sonnet");
  const [selectedProvider, setSelectedProvider] = useState("anthropic");
  
  // Refs
  const promptInputRef = useRef<FloatingPromptInputRef>(null);
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const isMountedRef = useRef(true);

  // Effects
  useEffect(() => {
    isMountedRef.current = true;
    return () => {
      isMountedRef.current = false;
    };
  }, []);

  useEffect(() => {
    if (onStreamingChange) {
      onStreamingChange(isLoading, currentSession?.id || null);
    }
  }, [isLoading, currentSession?.id, onStreamingChange]);

  useEffect(() => {
    // Auto-scroll to bottom when new messages arrive
    if (messagesEndRef.current) {
      messagesEndRef.current.scrollIntoView({ behavior: 'smooth' });
    }
  }, [messages]);

  // Event handlers
  useEffect(() => {
    const unsubscribe = onEvent((event) => {
      switch (event.type) {
        case 'session-error':
          if (event.sessionId === currentSession?.id) {
            setError(event.error?.message || 'Session error occurred');
            setLoading(false);
          }
          break;
        case 'server-started':
          setError(null);
          break;
      }
    });

    return unsubscribe;
  }, [onEvent, currentSession?.id]);

  // Handlers
  const handleSelectPath = async () => {
    try {
      const selected = await open({
        directory: true,
        multiple: false,
        title: "Select Project Directory"
      });
      
      if (selected) {
        setProjectPath(selected as string);
        setError(null);
      }
    } catch (err) {
      console.error("Failed to select directory:", err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`Failed to select directory: ${errorMessage}`);
    }
  };

  const handleStartNewSession = async () => {
    if (!projectPath) {
      setError("Please select a project directory first");
      return;
    }

    try {
      setError(null);
      const session = await createSession();
      // Note: OpenCode sessions don't need project path binding like Claude CLI
      // The session will use the current working directory or can be configured separately
      console.log("Created new OpenCode session:", session.id);
    } catch (err) {
      console.error("Failed to create session:", err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`Failed to create session: ${errorMessage}`);
    }
  };

  const handleSendPrompt = async (prompt: string, model: "sonnet" | "opus") => {
    if (!currentSession) {
      await handleStartNewSession();
      // Wait a bit for session to be created
      setTimeout(() => handleSendPrompt(prompt, model), 100);
      return;
    }

    if (!projectPath) {
      setError("Please select a project directory first");
      return;
    }

    try {
      setError(null);
      const modelId = model === "sonnet" ? "claude-3-5-sonnet-20241022" : "claude-3-opus-20240229";
      await sendMessage(prompt, modelId, selectedProvider);
    } catch (err) {
      console.error("Failed to send message:", err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`Failed to send message: ${errorMessage}`);
    }
  };

  const handleStopExecution = async () => {
    if (currentSession) {
      try {
        await abortSession(currentSession.id);
      } catch (err) {
        console.error("Failed to stop execution:", err);
        const errorMessage = err instanceof Error ? err.message : String(err);
        setError(`Failed to stop execution: ${errorMessage}`);
      }
    }
  };

  const handleStartServer = async () => {
    try {
      setError(null);
      await startServer();
    } catch (err) {
      console.error("Failed to start OpenCode server:", err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`Failed to start OpenCode server: ${errorMessage}`);
    }
  };

  const handleStopServer = async () => {
    try {
      setError(null);
      await stopServer();
    } catch (err) {
      console.error("Failed to stop OpenCode server:", err);
      const errorMessage = err instanceof Error ? err.message : String(err);
      setError(`Failed to stop OpenCode server: ${errorMessage}`);
    }
  };

  // Convert OpenCode messages to format compatible with StreamMessage component
  const convertMessagesToStreamFormat = (messages: OpenCodeMessage[]) => {
    return messages.flatMap(message => {
      const result = [];
      
      // Add a message header
      result.push({
        type: 'system',
        subtype: 'message_start',
        role: message.role,
        message_id: message.id,
        timestamp: new Date(message.time.created).toISOString(),
      });

      // Convert message parts
      message.parts.forEach(part => {
        switch (part.type) {
          case 'text':
            result.push({
              type: 'text',
              text: part.text,
              message_id: message.id,
              timestamp: new Date().toISOString(),
            });
            break;
          case 'tool':
            result.push({
              type: 'tool_use',
              tool_name: part.tool,
              tool_input: part.state.status === 'running' || part.state.status === 'completed' 
                ? part.state.input : {},
              tool_call_id: part.id,
              message_id: message.id,
              timestamp: new Date().toISOString(),
            });
            
            if (part.state.status === 'completed') {
              result.push({
                type: 'tool_result',
                tool_call_id: part.id,
                content: part.state.output,
                message_id: message.id,
                timestamp: new Date().toISOString(),
              });
            }
            break;
        }
      });

      return result;
    });
  };

  const streamMessages = convertMessagesToStreamFormat(messages);

  // Server status indicator
  const getServerStatusIcon = () => {
    if (serverError) {
      return <AlertCircle className="h-4 w-4 text-red-500" />;
    }
    if (!isServerRunning) {
      return <Server className="h-4 w-4 text-gray-500" />;
    }
    return <CheckCircle className="h-4 w-4 text-green-500" />;
  };

  const getServerStatusText = () => {
    if (serverError) return `Server Error: ${serverError}`;
    if (!isServerRunning) return "Server Stopped";
    return `Server Running (Port: ${serverInfo?.port})`;
  };

  return (
    <ErrorBoundary>
      <div className={cn("flex flex-col h-full bg-background", className)}>
        {/* Header */}
        <div className="flex items-center justify-between p-4 border-b bg-muted/50">
          <div className="flex items-center gap-3">
            <Button variant="ghost" size="sm" onClick={onBack}>
              <ArrowLeft className="h-4 w-4" />
            </Button>
            <div className="flex items-center gap-2">
              <Terminal className="h-5 w-5" />
              <h1 className="text-lg font-semibold">OpenCode Session</h1>
            </div>
          </div>
          
          <div className="flex items-center gap-2">
            {/* Server Status */}
            <TooltipProvider>
              <Tooltip>
                <TooltipTrigger asChild>
                  <div className="flex items-center gap-2 px-2 py-1 rounded-md bg-background border">
                    {getServerStatusIcon()}
                    <span className="text-sm font-medium">
                      {isServerRunning ? "Running" : "Stopped"}
                    </span>
                  </div>
                </TooltipTrigger>
                <TooltipContent>
                  <p>{getServerStatusText()}</p>
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>

            {/* Server Controls */}
            {!isServerRunning ? (
              <Button size="sm" onClick={handleStartServer}>
                <Play className="h-4 w-4 mr-1" />
                Start Server
              </Button>
            ) : (
              <Button variant="outline" size="sm" onClick={handleStopServer}>
                <StopCircle className="h-4 w-4 mr-1" />
                Stop Server
              </Button>
            )}

            {/* Session Info */}
            {currentSession && (
              <div className="text-sm text-muted-foreground">
                Session: {currentSession.id.slice(0, 8)}...
              </div>
            )}
          </div>
        </div>

        {/* Content */}
        <div className="flex-1 flex flex-col">
          {/* Project Path Selection */}
          {!projectPath && (
            <div className="p-4 border-b bg-muted/30">
              <div className="flex items-center gap-3">
                <Label htmlFor="project-path" className="text-sm font-medium">
                  Project Directory:
                </Label>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={handleSelectPath}
                  className="flex items-center gap-2"
                >
                  <FolderOpen className="h-4 w-4" />
                  Select Directory
                </Button>
              </div>
            </div>
          )}

          {/* Selected Project Path */}
          {projectPath && (
            <div className="p-3 border-b bg-muted/20">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2 text-sm">
                  <FolderOpen className="h-4 w-4 text-muted-foreground" />
                  <span className="font-mono">{projectPath}</span>
                </div>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={handleSelectPath}
                  className="text-xs"
                >
                  Change
                </Button>
              </div>
            </div>
          )}

          {/* Error Display */}
          {error && (
            <div className="p-4 border-b bg-red-50 border-red-200">
              <div className="flex items-center gap-2 text-red-800">
                <AlertCircle className="h-4 w-4" />
                <span className="text-sm">{error}</span>
                <Button
                  variant="ghost"
                  size="sm"
                  onClick={() => setError(null)}
                  className="ml-auto"
                >
                  <X className="h-4 w-4" />
                </Button>
              </div>
            </div>
          )}

          {/* Messages Area */}
          <div className="flex-1 overflow-auto p-4">
            {streamMessages.length === 0 ? (
              <div className="flex items-center justify-center h-full text-muted-foreground">
                <div className="text-center">
                  <Terminal className="h-12 w-12 mx-auto mb-4 opacity-50" />
                  <p className="text-lg font-medium mb-2">OpenCode Session</p>
                  <p className="text-sm">
                    {!isServerRunning
                      ? "Start the OpenCode server to begin"
                      : !projectPath
                      ? "Select a project directory to start"
                      : "Send a message to start the conversation"
                    }
                  </p>
                </div>
              </div>
            ) : (
              <div className="space-y-4">
                <AnimatePresence>
                  {streamMessages.map((message, index) => (
                    <motion.div
                      key={`${message.message_id}-${index}`}
                      initial={{ opacity: 0, y: 20 }}
                      animate={{ opacity: 1, y: 0 }}
                      exit={{ opacity: 0, y: -20 }}
                      transition={{ duration: 0.2 }}
                    >
                      <StreamMessage
                        message={message}
                        isLatest={index === streamMessages.length - 1}
                      />
                    </motion.div>
                  ))}
                </AnimatePresence>
                <div ref={messagesEndRef} />
              </div>
            )}
          </div>

          {/* Loading Indicator */}
          {isLoading && (
            <div className="p-4 border-t bg-muted/30">
              <div className="flex items-center gap-2 text-sm text-muted-foreground">
                <Loader2 className="h-4 w-4 animate-spin" />
                <span>OpenCode is processing...</span>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={handleStopExecution}
                  className="ml-auto"
                >
                  <StopCircle className="h-4 w-4 mr-1" />
                  Stop
                </Button>
              </div>
            </div>
          )}

          {/* Prompt Input */}
          <div className="border-t">
            <FloatingPromptInput
              ref={promptInputRef}
              onSendPrompt={handleSendPrompt}
              disabled={!isServerRunning || !projectPath}
              showModelSelector={true}
              availableModels={[
                { id: "sonnet", name: "Claude 3.5 Sonnet", description: "Best for coding and analysis" },
                { id: "opus", name: "Claude 3 Opus", description: "Most capable model" }
              ]}
              placeholder={
                !isServerRunning
                  ? "Start the OpenCode server first..."
                  : !projectPath
                  ? "Select a project directory first..."
                  : isLoading
                  ? "Processing previous message..."
                  : "Ask OpenCode anything..."
              }
            />
          </div>
        </div>
      </div>
    </ErrorBoundary>
  );
};