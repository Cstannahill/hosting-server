const express = require('express');
const helmet = require('helmet');

const app = express();
app.disable('x-powered-by');
app.use(helmet());

const port = process.env.PORT || 4000;

app.get('/', (req, res) => {
  res.json({ message: 'Hello from Express' });
});

app.get('/health', (_req, res) => res.send('ok'));

const server = app.listen(port, () => {
  console.log(`Express server listening on ${port}`);
});

function shutdown() {
  server.close(() => process.exit(0));
}

process.on('SIGINT', shutdown);
process.on('SIGTERM', shutdown);
