import { useState, useRef, useEffect } from 'react';
import Home from './pages/page';

function App() {
  const [showDebugLogs, setShowDebugLogs] = useState(false);

  return (
    <div className="min-h-screen flex">
      {/* Main content */}
      <div className={`flex-1 ${showDebugLogs ? 'w-2/3' : 'w-full'}`}>
        <Home />
      </div>

      {/* Debug Logs Sidebar */}
      {showDebugLogs && (
        <div className="w-1/3 bg-gray-900 border-l border-gray-700 flex flex-col">
          <DebugLogsSidebar onClose={() => setShowDebugLogs(false)} />
        </div>
      )}

      {/* Toggle button */}
      <button
        onClick={() => setShowDebugLogs(!showDebugLogs)}
        className="fixed bottom-4 right-4 z-50 px-4 py-2 bg-gray-600 text-white rounded-lg hover:bg-gray-700 transition-colors text-sm"
        title="Toggle debug logs"
      >
        {showDebugLogs ? '✕ Close Logs' : '🐛 Show Logs'}
      </button>
    </div>
  );
}

// Inline debug logs component
function DebugLogsSidebar({ onClose }: { onClose: () => void }) {
  const [logs, setLogs] = useState<Array<{timestamp: string; message: string}>>([]);
  const scrollRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    // Capture console logs
    const originalLog = console.log;
    const originalError = console.error;

    const addLog = (...args: any[]) => {
      const message = args.map(arg =>
        typeof arg === 'object' ? JSON.stringify(arg, null, 2) : String(arg)
      ).join(' ');

      setLogs(prev => [...prev, {
        timestamp: new Date().toLocaleTimeString(),
        message,
      }]);
    };

    console.log = (...args) => {
      addLog(...args);
      originalLog(...args);
    };

    console.error = (...args) => {
      addLog(...args);
      originalError(...args);
    };

    return () => {
      console.log = originalLog;
      console.error = originalError;
    };
  }, []);

  // Auto-scroll
  useEffect(() => {
    if (scrollRef.current) {
      scrollRef.current.scrollTop = scrollRef.current.scrollHeight;
    }
  }, [logs]);

  return (
    <>
      <div className="p-3 border-b border-gray-700 flex justify-between items-center bg-gray-800">
        <h3 className="text-white font-semibold text-sm">🐛 Debug Logs</h3>
        <button
          onClick={onClose}
          className="text-gray-400 hover:text-white text-lg"
        >
          ✕
        </button>
      </div>
      <div
        ref={scrollRef}
        className="flex-1 overflow-y-auto p-2 font-mono text-xs bg-gray-900"
      >
        {logs.length === 0 ? (
          <div className="text-gray-500">Logs will appear here...</div>
        ) : (
          logs.map((log, idx) => (
            <div key={idx} className="text-gray-300 mb-1 p-1 hover:bg-gray-800 rounded">
              <span className="text-blue-400">[{log.timestamp}]</span> {log.message}
            </div>
          ))
        )}
      </div>
      <button
        onClick={() => setLogs([])}
        className="p-2 bg-gray-800 text-gray-300 hover:bg-gray-700 text-xs border-t border-gray-700"
      >
        Clear Logs
      </button>
    </>
  );
}

export default App;
