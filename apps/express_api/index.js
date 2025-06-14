const express = require('express');
const app = express();
const port = process.env.PORT || 4000;

app.get('/', (req, res) => {
  res.json({ message: 'Hello from Express' });
});

app.listen(port, () => {
  console.log(`Express server listening on ${port}`);
});
