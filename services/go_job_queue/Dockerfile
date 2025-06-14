# Build stage
FROM golang:1.21-alpine AS builder
WORKDIR /app
COPY go.mod .
COPY main.go .
RUN go build -o go-job-queue

# Final stage
FROM alpine:latest
WORKDIR /app
COPY --from=builder /app/go-job-queue /usr/local/bin/go-job-queue
EXPOSE 9000
ENV PORT=9000
CMD ["go-job-queue"]
