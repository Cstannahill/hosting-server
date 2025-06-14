import React from 'react';
import ReactDOM from 'react-dom/client';

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
}

ReactDOM.createRoot(document.getElementById('root')!).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>
);
