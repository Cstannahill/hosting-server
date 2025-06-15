import { useEffect, useState } from 'react';

interface AppInfo {
  name: string;
}

export function App() {
  const base = import.meta.env.VITE_API_URL || '/api';
  const [status, setStatus] = useState('Loading...');
  const [apps, setApps] = useState<AppInfo[]>([]);

  useEffect(() => {
    fetch(`${base}/apps`)
      .then((r) => (r.ok ? r.json() : Promise.reject()))
      .then((data) => {
        setApps(data.apps.map((n: string) => ({ name: n })));
        setStatus('Ready');
      })
      .catch(() => setStatus('API unreachable'));
  }, [base]);

  const action = async (name: string, cmd: 'deploy' | 'stop') => {
    setStatus(`${cmd}ing ${name}...`);
    await fetch(`${base}/${cmd}/${name}`, { method: 'POST' });
    setStatus('Done');
  };

  return (
    <main>
      <h1>Platform Deployments</h1>
      <p>{status}</p>
      <ul>
        {apps.map((a) => (
          <li key={a.name}>
            {a.name}{' '}
            <button onClick={() => action(a.name, 'deploy')}>Deploy</button>{' '}
            <button onClick={() => action(a.name, 'stop')}>Stop</button>
          </li>
        ))}
      </ul>
    </main>
  );
}
