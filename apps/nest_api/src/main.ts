import { NestFactory } from '@nestjs/core';
import { Module, Controller, Get } from '@nestjs/common';

@Controller()
class AppController {
  @Get()
  getRoot() {
    return { message: 'Hello from NestJS' };
  }

  @Get('health')
  health() {
    return { status: 'ok' };
  }
}

@Module({ controllers: [AppController] })
class AppModule {}

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  app.enableShutdownHooks();
  const port = parseInt(process.env.PORT || '4001', 10);
  await app.listen(port);
}

bootstrap();
