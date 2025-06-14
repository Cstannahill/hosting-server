import React from 'react';
import ReactDOM from 'react-dom/client';
import { ErrorBoundary } from './ErrorBoundary';
import { useEffect, useState } from 'react';

function App() {
  const [message, setMessage] = useState('Loading...');
  useEffect(() => {
    const base = import.meta.env.VITE_API_URL || '/api';
    fetch(base + '/health')
      .then((r) => r.json())
      .then(() => setMessage('It works!'))
      .catch(() => setMessage('API unreachable'));
  }, []);
  return <h1>{message}</h1>;
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <ErrorBoundary>
      <App />
    </ErrorBoundary>
  </React.StrictMode>
);
