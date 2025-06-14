# apps/web/Dockerfile
FROM node:20-alpine

WORKDIR /app

COPY . .

RUN npm ci --omit=dev && npm run build

EXPOSE 3000

CMD ["npm", "start"]
