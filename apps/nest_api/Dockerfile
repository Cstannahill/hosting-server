FROM node:20-alpine AS build
WORKDIR /app
COPY package.json tsconfig.json ./
RUN npm ci
COPY src ./src
RUN npm run build

FROM node:20-alpine
WORKDIR /app
COPY --from=build /app/dist ./dist
COPY package.json ./
RUN npm ci --omit=dev
ENV NODE_ENV=production
EXPOSE 4001
CMD ["node", "dist/main.js"]
