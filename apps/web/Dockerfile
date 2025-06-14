# apps/web/Dockerfile (pnpm version)
FROM node:20-alpine

WORKDIR /app

COPY . .

# Install pnpm
RUN corepack enable && corepack prepare pnpm@8.15.5 --activate

# Install dependencies
RUN pnpm install --frozen-lockfile

# Build the app
RUN pnpm build

EXPOSE 3000

CMD ["pnpm", "start"]
