import { NestFactory } from '@nestjs/core';
import { Module, Controller, Get, ValidationPipe } from '@nestjs/common';
import * as compression from 'compression';

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

@Module({
  controllers: [AppController],
})
class AppModule {}

async function bootstrap() {
  const app = await NestFactory.create(AppModule);
  app.enableShutdownHooks();
  app.use(compression());
  app.enableCors();
  app.useGlobalPipes(new ValidationPipe({ transform: true }));
  const port = parseInt(process.env.PORT || '4001', 10);
  await app.listen(port);
}
bootstrap();
