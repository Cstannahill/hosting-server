# Build stage
FROM golang:1.21-alpine AS builder
WORKDIR /app
COPY go.mod .
COPY main.go .
RUN go build -o go-api

# Final stage
FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/go-api /usr/local/bin/go-api
EXPOSE 8080
ENV PORT=8080
CMD ["go-api"]
