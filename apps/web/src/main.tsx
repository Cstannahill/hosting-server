import React from 'react';
import ReactDOM from 'react-dom/client';
import { ErrorBoundary } from './ErrorBoundary';
import { useEffect, useState } from 'react';

function App() {
  const api = import.meta.env.VITE_API_URL || '';
  const [status, setStatus] = React.useState('');

  React.useEffect(() => {
    if (!api) return;
    fetch(`${api}/health`)
      .then((res) => {
        if (res.ok) setStatus('API online');
        else setStatus('API error');
      })
      .catch(() => setStatus('API unreachable'));
  }, [api]);

  return (
    <>
      <h1>It works!</h1>
      {api && <p>{status}</p>}
    </>
  );
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
    <App />

    <ErrorBoundary>
      <App />
    </ErrorBoundary>
  </React.StrictMode>
);
