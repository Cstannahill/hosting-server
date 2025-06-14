import { NestFactory } from '@nestjs/core';
import { Module, Controller, Get } from '@nestjs/common';
import compression from 'compression';
import * as express from 'express';

@Controller()
class AppController {
  @Get()
  getRoot() {
    return { message: 'Hello from NestJS' };
  }

  @Get('health')
  health() {
    return 'ok';
  }
}

@Module({
  controllers: [AppController],
})
class AppModule {}

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  app.enableCors();
  app.use(compression());
  const port = process.env.PORT || 4001;
  await app.listen(Number(port), '0.0.0.0');
}
bootstrap();
