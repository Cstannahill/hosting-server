const express = require('express');
const helmet = require('helmet');
const morgan = require('morgan');
const compression = require('compression');

const app = express();
app.disable('x-powered-by');
app.use(helmet());
app.use(compression());
app.use(morgan('combined'));

const port = process.env.PORT || 4000;

app.get('/', (req, res) => {
  res.json({ message: 'Hello from Express' });
});

app.get('/health', (_req, res) => res.send('ok'));

const server = app.listen(port, () => {
  console.log(`Express app listening on port ${port}`);
});

function shutdown() {
  server.close(() => process.exit(0));
}

process.on('SIGINT', shutdown);
process.on('SIGTERM', shutdown);
process.on('unhandledRejection', (err) => {
  console.error('Unhandled rejection', err);
  shutdown();
});
