const express = require('express');
const helmet = require('helmet');
const compression = require('compression');
const morgan = require('morgan');

const app = express();
const port = process.env.PORT || 4000;

app.use(helmet());
app.use(compression());
app.use(morgan('combined'));

app.get('/health', (_req, res) => {
  res.status(200).send('ok');
});

app.get('/', (_req, res) => {
  res.json({ message: 'Hello from Express' });
});

app.use((err, _req, res, _next) => {
  console.error(err);
  res.status(500).send('internal error');
});

app.listen(port, () => {
  console.log(`Express app listening on port ${port}`);
});
