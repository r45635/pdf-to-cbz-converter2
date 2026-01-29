// ============================================================================
// BUILD TIMESTAMP - Logged immediately when app starts
// ============================================================================
const buildTime = new Date().toISOString();
const timestamp = new Date().toLocaleTimeString('en-US', {
  hour12: true,
  hour: '2-digit',
  minute: '2-digit',
  second: '2-digit'
});
console.log('\n' + '='.repeat(100));
console.log(`🚀 APPLICATION STARTED AT: ${timestamp}`);
console.log(`📅 BUILD TIMESTAMP: ${buildTime}`);
console.log('='.repeat(100) + '\n');

import React from 'react';
import ReactDOM from 'react-dom/client';
import App from './App';
import './styles/globals.css';

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
