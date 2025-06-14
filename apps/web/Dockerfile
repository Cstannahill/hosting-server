# Stage 1: build static assets
FROM node:20-alpine AS build
WORKDIR /app
COPY package.json tsconfig.json vite.config.ts ./
COPY src ./src
COPY index.html ./
RUN npm ci && npm run build

# Stage 2: serve with nginx
FROM nginx:1.27-alpine
COPY --from=build /app/dist /usr/share/nginx/html
COPY nginx.conf /etc/nginx/conf.d/default.conf
EXPOSE 3000
CMD ["nginx", "-g", "daemon off;"]
