import { useEffect, useState } from 'react';

export function App() {
  const base = import.meta.env.VITE_API_URL || '/api';
  const [status, setStatus] = useState('Loading...');

  useEffect(() => {
    fetch(`${base}/health`)
      .then((r) => (r.ok ? r.json() : Promise.reject()))
      .then(() => setStatus('API online'))
      .catch(() => setStatus('API unreachable'));
  }, [base]);

  return (
    <main>
      <h1>It works!</h1>
      <p>{status}</p>
    </main>
  );
}
