import { INestApplication } from '@nestjs/common';
import { DocumentBuilder, SwaggerModule } from '@nestjs/swagger';
import { ConfigService } from '@nestjs/config';

export function setupSwagger(app: INestApplication, configService?: ConfigService) {
  const title = configService?.get('APP_NAME') || 'StellarEarn API';
  const version = configService?.get('API_VERSION') || '1.0';
  const description =
    configService?.get('API_DESCRIPTION') ||
    'Quest-based earning platform on Stellar blockchain';

  const builder = new DocumentBuilder()
    .setTitle(title)
    .setDescription(description)
    .setVersion(version)
    .addBearerAuth(
      { type: 'http', scheme: 'bearer', bearerFormat: 'JWT' },
      'JWT-auth',
    )
    .addTag('Authentication')
    .addTag('Health', 'System health and readiness probes');

  const document = SwaggerModule.createDocument(app, builder.build(), {
    deepScanRoutes: true,
  });

  SwaggerModule.setup('api/docs', app, document, {
    swaggerOptions: { persistAuthorization: true },
  });
}
