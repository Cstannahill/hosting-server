# Stage 1 - build
FROM node:20-alpine AS builder
WORKDIR /app
COPY package.json tsconfig.json vite.config.ts ./
COPY public ./public
COPY src ./src
RUN npm install && npm run build

# Stage 2 - serve
FROM nginx:alpine
COPY --from=builder /app/dist /usr/share/nginx/html
EXPOSE 3000
CMD ["nginx", "-g", "daemon off;"]
