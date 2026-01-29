import { useState, useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';

interface LogEntry {
  timestamp: string;
  level: 'log' | 'error' | 'warn' | 'info' | 'progress';
  message: string;
  source: string;
}

export default function DebugPage() {
  const [logs, setLogs] = useState<LogEntry[]>([]);
  const [isAutoScroll, setIsAutoScroll] = useState(true);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Capture all console methods
    const originalLog = console.log;
    const originalError = console.error;
    const originalWarn = console.warn;
    const originalInfo = console.info;

    const addLog = (level: 'log' | 'error' | 'warn' | 'info', source: string, ...args: any[]) => {
      const message = args.map(arg =>
        typeof arg === 'object' ? JSON.stringify(arg, null, 2) : String(arg)
      ).join(' ');

      setLogs(prev => [...prev, {
        timestamp: new Date().toLocaleTimeString(),
        level,
        message,
        source,
      }]);
    };

    console.log = (...args) => {
      addLog('log', 'FRONTEND', ...args);
      originalLog(...args);
    };

    console.error = (...args) => {
      addLog('error', 'FRONTEND', ...args);
      originalError(...args);
    };

    console.warn = (...args) => {
      addLog('warn', 'FRONTEND', ...args);
      originalWarn(...args);
    };

    console.info = (...args) => {
      addLog('info', 'FRONTEND', ...args);
      originalInfo(...args);
    };

    // Listen for backend progress events
    let unlistenProgress: (() => void) | undefined;
    listen('conversion-progress', (event: any) => {
      setLogs(prev => [...prev, {
        timestamp: new Date().toLocaleTimeString(),
        level: 'progress',
        message: JSON.stringify(event.payload, null, 2),
        source: 'BACKEND',
      }]);
    }).then(fn => {
      unlistenProgress = fn;
    });

    return () => {
      console.log = originalLog;
      console.error = originalError;
      console.warn = originalWarn;
      console.info = originalInfo;
      if (unlistenProgress) unlistenProgress();
    };
  }, []);

  // Auto-scroll to bottom
  useEffect(() => {
    if (isAutoScroll && scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [logs, isAutoScroll]);

  const handleClear = () => {
    setLogs([]);
  };

  const handleExport = () => {
    const logText = logs.map(log =>
      `[${log.timestamp}] [${log.level.toUpperCase()}] [${log.source}] ${log.message}`
    ).join('\n');

    const blob = new Blob([logText], { type: 'text/plain' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = `debug-logs-${new Date().toISOString()}.txt`;
    a.click();
    URL.revokeObjectURL(url);
  };

  const getLogColor = (level: string) => {
    switch (level) {
      case 'error':
        return 'text-red-600 dark:text-red-400';
      case 'warn':
        return 'text-yellow-600 dark:text-yellow-400';
      case 'info':
        return 'text-blue-600 dark:text-blue-400';
      case 'progress':
        return 'text-green-600 dark:text-green-400';
      default:
        return 'text-gray-600 dark:text-gray-400';
    }
  };

  const getLogBg = (level: string) => {
    switch (level) {
      case 'error':
        return 'bg-red-50 dark:bg-red-900/10';
      case 'warn':
        return 'bg-yellow-50 dark:bg-yellow-900/10';
      case 'info':
        return 'bg-blue-50 dark:bg-blue-900/10';
      case 'progress':
        return 'bg-green-50 dark:bg-green-900/10';
      default:
        return 'bg-gray-50 dark:bg-gray-900/10';
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 dark:from-gray-900 dark:to-gray-800">
      {/* Header */}
      <header className="bg-white dark:bg-gray-800 shadow-sm">
        <div className="max-w-7xl mx-auto px-4 py-4 sm:px-6 lg:px-8">
          <h1 className="text-2xl font-bold text-gray-900 dark:text-white">
            🐛 Debug Logs
          </h1>
        </div>
      </header>

      <main className="max-w-7xl mx-auto px-4 py-8 sm:px-6 lg:px-8">
        {/* Controls */}
        <div className="mb-4 flex gap-2">
          <button
            onClick={handleClear}
            className="px-4 py-2 bg-red-600 text-white rounded-lg hover:bg-red-700 transition-colors"
          >
            🗑️ Clear Logs
          </button>
          <button
            onClick={handleExport}
            className="px-4 py-2 bg-green-600 text-white rounded-lg hover:bg-green-700 transition-colors"
          >
            💾 Export to File
          </button>
          <label className="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 cursor-pointer transition-colors">
            <input
              type="checkbox"
              checked={isAutoScroll}
              onChange={(e) => setIsAutoScroll(e.target.checked)}
              className="rounded"
            />
            Auto-Scroll
          </label>
          <div className="flex-1 text-right text-gray-600 dark:text-gray-400">
            Total logs: {logs.length}
          </div>
        </div>

        {/* Logs Container */}
        <div
          ref={scrollRef}
          className="bg-white dark:bg-gray-800 rounded-lg shadow-lg p-4 font-mono text-sm overflow-y-auto h-[calc(100vh-200px)] border border-gray-200 dark:border-gray-700"
        >
          {logs.length === 0 ? (
            <div className="text-gray-500 dark:text-gray-400 text-center py-8">
              No logs yet. Perform a conversion to see logs here.
            </div>
          ) : (
            logs.map((log, idx) => (
              <div key={idx} className={`p-2 mb-1 rounded ${getLogBg(log.level)}`}>
                <div className={`${getLogColor(log.level)}`}>
                  <span className="font-bold">[{log.timestamp}]</span>
                  {' '}
                  <span className="font-bold">[{log.level.toUpperCase()}]</span>
                  {' '}
                  <span className="font-bold">[{log.source}]</span>
                  {' '}
                  <span className="break-words">{log.message}</span>
                </div>
              </div>
            ))
          )}
        </div>
      </main>
    </div>
  );
}
