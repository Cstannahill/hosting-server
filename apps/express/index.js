const express = require('express');
const helmet = require('helmet');
const compression = require('compression');
const morgan = require('morgan');

const app = express();
app.disable('x-powered-by');
app.use(helmet());
app.use(morgan('combined'));
app.use(compression());

const port = process.env.PORT || 4000;

app.get('/', (req, res) => {
  res.json({ message: 'Hello from Express' });
});

app.get('/health', (_req, res) => res.send('ok'));

const server = app.listen(port, () => {
  console.log(`Express app listening on port ${port}`);
});

app.use((req, res) => {
  res.status(404).json({ error: 'Not found' });
});

app.use((err, _req, res, _next) => {
  console.error(err);
  res.status(500).json({ error: 'Server error' });
});

function shutdown() {
  server.close(() => process.exit(0));
}

process.on('SIGINT', shutdown);
process.on('SIGTERM', shutdown);
