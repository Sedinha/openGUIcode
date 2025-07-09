import { useState, useEffect, useRef, useCallback } from 'react';
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { api, type OpenCodeSession, type OpenCodeMessage, type OpenCodeServerInfo } from '@/lib/api';

export interface OpenCodeEvent {
  type: string;
  [key: string]: any;
}

export interface UseOpenCodeOptions {
  /** Whether to auto-start the server on mount */
  autoStart?: boolean;
  /** Whether to auto-connect to event stream */
  autoConnect?: boolean;
  /** Session ID to focus on for events */
  sessionId?: string;
}

export interface UseOpenCodeReturn {
  // Server state
  serverInfo: OpenCodeServerInfo | null;
  isServerRunning: boolean;
  serverError: string | null;
  
  // Session state
  sessions: OpenCodeSession[];
  currentSession: OpenCodeSession | null;
  messages: OpenCodeMessage[];
  isLoading: boolean;
  
  // Actions
  startServer: () => Promise<void>;
  stopServer: () => Promise<void>;
  createSession: () => Promise<OpenCodeSession>;
  sendMessage: (message: string, model?: string, provider?: string) => Promise<void>;
  loadSession: (sessionId: string) => Promise<void>;
  abortSession: (sessionId?: string) => Promise<void>;
  
  // Event handlers
  onEvent: (callback: (event: OpenCodeEvent) => void) => UnlistenFn;
}

/**
 * Hook for managing OpenCode integration
 */
export function useOpenCode(options: UseOpenCodeOptions = {}): UseOpenCodeReturn {
  const {
    autoStart = false,
    autoConnect = true,
    sessionId: focusSessionId
  } = options;

  // State
  const [serverInfo, setServerInfo] = useState<OpenCodeServerInfo | null>(null);
  const [serverError, setServerError] = useState<string | null>(null);
  const [sessions, setSessions] = useState<OpenCodeSession[]>([]);
  const [currentSession, setCurrentSession] = useState<OpenCodeSession | null>(null);
  const [messages, setMessages] = useState<OpenCodeMessage[]>([]);
  const [isLoading, setIsLoading] = useState(false);

  // Refs
  const eventListenersRef = useRef<UnlistenFn[]>([]);
  const eventCallbacksRef = useRef<((event: OpenCodeEvent) => void)[]>([]);
  const isMountedRef = useRef(true);

  // Cleanup listeners
  const cleanupListeners = useCallback(() => {
    eventListenersRef.current.forEach(unlisten => unlisten());
    eventListenersRef.current = [];
  }, []);

  // Emit event to callbacks
  const emitEvent = useCallback((event: OpenCodeEvent) => {
    eventCallbacksRef.current.forEach(callback => {
      try {
        callback(event);
      } catch (error) {
        console.error('Error in OpenCode event callback:', error);
      }
    });
  }, []);

  // Setup event listeners
  const setupEventListeners = useCallback(async () => {
    cleanupListeners();

    try {
      // Message updated events
      const messageUpdatedUnlisten = await listen<OpenCodeMessage>('opencode-message-updated', (event) => {
        if (!isMountedRef.current) return;
        
        const message = event.payload;
        
        // Update messages if it's for the current session
        if (currentSession && message.sessionID === currentSession.id) {
          setMessages(prev => {
            const index = prev.findIndex(m => m.id === message.id);
            if (index >= 0) {
              const newMessages = [...prev];
              newMessages[index] = message;
              return newMessages;
            } else {
              return [...prev, message];
            }
          });
        }
        
        emitEvent({ type: 'message-updated', message });
      });

      // Message part updated events
      const messagePartUpdatedUnlisten = await listen<{
        part: any;
        sessionId: string;
        messageId: string;
      }>('opencode-message-part-updated', (event) => {
        if (!isMountedRef.current) return;
        
        const { part, sessionId, messageId } = event.payload;
        
        // Update the specific message part if it's for the current session
        if (currentSession && sessionId === currentSession.id) {
          setMessages(prev => prev.map(message => {
            if (message.id === messageId) {
              const partIndex = message.parts.findIndex(p => 
                p.type === 'tool' && 'id' in p && p.id === part.id
              );
              if (partIndex >= 0) {
                const newParts = [...message.parts];
                newParts[partIndex] = part;
                return { ...message, parts: newParts };
              } else if (part.type === 'text') {
                // For text parts, append or update
                return { ...message, parts: [...message.parts, part] };
              }
            }
            return message;
          }));
        }
        
        emitEvent({ type: 'message-part-updated', part, sessionId, messageId });
      });

      // Session updated events
      const sessionUpdatedUnlisten = await listen<OpenCodeSession>('opencode-session-updated', (event) => {
        if (!isMountedRef.current) return;
        
        const session = event.payload;
        
        // Update sessions list
        setSessions(prev => {
          const index = prev.findIndex(s => s.id === session.id);
          if (index >= 0) {
            const newSessions = [...prev];
            newSessions[index] = session;
            return newSessions;
          } else {
            return [...prev, session];
          }
        });
        
        // Update current session if it matches
        if (currentSession && currentSession.id === session.id) {
          setCurrentSession(session);
        }
        
        emitEvent({ type: 'session-updated', session });
      });

      // Session idle events
      const sessionIdleUnlisten = await listen<string>('opencode-session-idle', (event) => {
        if (!isMountedRef.current) return;
        
        const sessionId = event.payload;
        
        // Stop loading if it's for the current session
        if (currentSession && currentSession.id === sessionId) {
          setIsLoading(false);
        }
        
        emitEvent({ type: 'session-idle', sessionId });
      });

      // Session error events
      const sessionErrorUnlisten = await listen<{
        sessionId?: string;
        error: any;
      }>('opencode-session-error', (event) => {
        if (!isMountedRef.current) return;
        
        const { sessionId, error } = event.payload;
        
        // Handle error for current session
        if (currentSession && sessionId === currentSession.id) {
          setIsLoading(false);
          console.error('OpenCode session error:', error);
        }
        
        emitEvent({ type: 'session-error', sessionId, error });
      });

      // Server events
      const serverStartedUnlisten = await listen<OpenCodeServerInfo>('opencode-server-started', (event) => {
        if (!isMountedRef.current) return;
        setServerInfo(event.payload);
        setServerError(null);
        emitEvent({ type: 'server-started', serverInfo: event.payload });
      });

      // Store listeners for cleanup
      eventListenersRef.current = [
        messageUpdatedUnlisten,
        messagePartUpdatedUnlisten,
        sessionUpdatedUnlisten,
        sessionIdleUnlisten,
        sessionErrorUnlisten,
        serverStartedUnlisten,
      ];

      // If we have a focus session, also listen to session-specific events
      if (focusSessionId) {
        const sessionSpecificUnlisteners = await setupSessionSpecificListeners(focusSessionId);
        eventListenersRef.current.push(...sessionSpecificUnlisteners);
      }

    } catch (error) {
      console.error('Failed to setup OpenCode event listeners:', error);
    }
  }, [currentSession, focusSessionId, emitEvent, cleanupListeners]);

  // Setup session-specific listeners
  const setupSessionSpecificListeners = useCallback(async (sessionId: string): Promise<UnlistenFn[]> => {
    const listeners: UnlistenFn[] = [];

    try {
      // Session-specific message events
      const messageUnlisten = await listen<OpenCodeMessage>(`opencode-message-updated:${sessionId}`, (event) => {
        if (!isMountedRef.current) return;
        
        const message = event.payload;
        setMessages(prev => {
          const index = prev.findIndex(m => m.id === message.id);
          if (index >= 0) {
            const newMessages = [...prev];
            newMessages[index] = message;
            return newMessages;
          } else {
            return [...prev, message];
          }
        });
      });

      const partUnlisten = await listen<{
        part: any;
        sessionId: string;
        messageId: string;
      }>(`opencode-message-part-updated:${sessionId}`, (event) => {
        if (!isMountedRef.current) return;
        
        const { part, messageId } = event.payload;
        setMessages(prev => prev.map(message => {
          if (message.id === messageId) {
            const partIndex = message.parts.findIndex(p => 
              p.type === 'tool' && 'id' in p && p.id === part.id
            );
            if (partIndex >= 0) {
              const newParts = [...message.parts];
              newParts[partIndex] = part;
              return { ...message, parts: newParts };
            } else if (part.type === 'text') {
              return { ...message, parts: [...message.parts, part] };
            }
          }
          return message;
        }));
      });

      const idleUnlisten = await listen<string>(`opencode-session-idle:${sessionId}`, () => {
        if (!isMountedRef.current) return;
        setIsLoading(false);
      });

      listeners.push(messageUnlisten, partUnlisten, idleUnlisten);
    } catch (error) {
      console.error('Failed to setup session-specific listeners:', error);
    }

    return listeners;
  }, []);

  // Actions
  const startServer = useCallback(async () => {
    try {
      setServerError(null);
      const info = await api.startOpenCodeServer();
      setServerInfo(info);
      
      if (autoConnect) {
        await api.connectOpenCodeEventStream();
        await setupEventListeners();
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      setServerError(errorMessage);
      throw error;
    }
  }, [autoConnect, setupEventListeners]);

  const stopServer = useCallback(async () => {
    try {
      cleanupListeners();
      await api.stopOpenCodeServer();
      setServerInfo(null);
      setServerError(null);
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      setServerError(errorMessage);
      throw error;
    }
  }, [cleanupListeners]);

  const createSession = useCallback(async (): Promise<OpenCodeSession> => {
    try {
      const session = await api.createOpenCodeSession();
      setSessions(prev => [...prev, session]);
      return session;
    } catch (error) {
      console.error('Failed to create OpenCode session:', error);
      throw error;
    }
  }, []);

  const sendMessage = useCallback(async (
    message: string, 
    model: string = 'claude-3-5-sonnet-20241022', 
    provider: string = 'anthropic'
  ) => {
    if (!currentSession) {
      throw new Error('No current session');
    }

    try {
      setIsLoading(true);
      await api.sendOpenCodeChatMessage(currentSession.id, message, provider, model);
      // The response will come through event listeners
    } catch (error) {
      setIsLoading(false);
      throw error;
    }
  }, [currentSession]);

  const loadSession = useCallback(async (sessionId: string) => {
    try {
      // Find session in current list or fetch it
      let session = sessions.find(s => s.id === sessionId);
      if (!session) {
        const allSessions = await api.listOpenCodeSessions();
        session = allSessions.find(s => s.id === sessionId);
        if (session) {
          setSessions(allSessions);
        }
      }

      if (!session) {
        throw new Error(`Session ${sessionId} not found`);
      }

      setCurrentSession(session);
      
      // Load messages for the session
      const sessionMessages = await api.getOpenCodeSessionMessages(sessionId);
      setMessages(sessionMessages);

      // Setup session-specific listeners
      cleanupListeners();
      await setupEventListeners();
      
    } catch (error) {
      console.error('Failed to load OpenCode session:', error);
      throw error;
    }
  }, [sessions, setupEventListeners, cleanupListeners]);

  const abortSession = useCallback(async (sessionId?: string) => {
    const targetSessionId = sessionId || currentSession?.id;
    if (!targetSessionId) {
      throw new Error('No session to abort');
    }

    try {
      await api.abortOpenCodeSession(targetSessionId);
      setIsLoading(false);
    } catch (error) {
      console.error('Failed to abort OpenCode session:', error);
      throw error;
    }
  }, [currentSession]);

  const onEvent = useCallback((callback: (event: OpenCodeEvent) => void): UnlistenFn => {
    eventCallbacksRef.current.push(callback);
    
    return () => {
      const index = eventCallbacksRef.current.indexOf(callback);
      if (index >= 0) {
        eventCallbacksRef.current.splice(index, 1);
      }
    };
  }, []);

  // Effects
  useEffect(() => {
    isMountedRef.current = true;

    // Check server status on mount
    api.getOpenCodeServerStatus()
      .then(info => {
        if (info) {
          setServerInfo(info);
          if (autoConnect) {
            api.connectOpenCodeEventStream()
              .then(() => setupEventListeners())
              .catch(error => console.error('Failed to connect to event stream:', error));
          }
        } else if (autoStart) {
          startServer().catch(error => console.error('Failed to auto-start server:', error));
        }
      })
      .catch(error => console.error('Failed to get server status:', error));

    return () => {
      isMountedRef.current = false;
      cleanupListeners();
    };
  }, [autoStart, autoConnect, setupEventListeners, startServer, cleanupListeners]);

  // Load sessions when server is running
  useEffect(() => {
    if (serverInfo && serverInfo.status === 'Running') {
      api.listOpenCodeSessions()
        .then(setSessions)
        .catch(error => console.error('Failed to load sessions:', error));
    }
  }, [serverInfo]);

  // Computed values
  const isServerRunning = serverInfo?.status === 'Running';

  return {
    // Server state
    serverInfo,
    isServerRunning,
    serverError,
    
    // Session state
    sessions,
    currentSession,
    messages,
    isLoading,
    
    // Actions
    startServer,
    stopServer,
    createSession,
    sendMessage,
    loadSession,
    abortSession,
    
    // Event handlers
    onEvent,
  };
}