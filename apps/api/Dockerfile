FROM python:3.12-slim
ENV PYTHONUNBUFFERED=1
WORKDIR /app
COPY requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt
COPY . .
RUN chmod +x start.sh prestart.sh
EXPOSE ${PORT:-8000}
CMD ["./start.sh"]
