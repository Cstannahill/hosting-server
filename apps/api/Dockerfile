FROM python:3.12-slim

WORKDIR /app

COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY . .

RUN chmod +x start.sh prestart.sh

ENV PORT=8000

EXPOSE $PORT

CMD ["./start.sh"]
