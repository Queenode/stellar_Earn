
import { NestFactory } from '@nestjs/core';
import { ValidationPipe } from '@nestjs/common';
import { ConfigService } from '@nestjs/config';
import { SwaggerModule } from '@nestjs/swagger';
import { VersioningType } from '@nestjs/common';
import { setupSwagger } from './config/swagger.config';
import helmet from 'helmet';
import { AppModule } from './app.module';
import { CustomValidationPipe } from './common/pipes/validation.pipe';
import { SanitizationPipe } from './common/pipes/sanitization.pipe';
import { ValidationExceptionFilter } from './common/filters/validation-exception.filter';
import { SecurityExceptionFilter } from './common/filters/security-exception.filter';
import { SecurityMiddleware } from './common/middleware/security.middleware';
import { getSecurityConfig } from './config/security.config';
import { getCorsConfig } from './config/cors.config';

// Catch all unhandled errors
process.on('unhandledRejection', (reason, promise) => {
  console.error('Unhandled Rejection at:', promise, 'reason:', reason);
  process.exit(1);
});

process.on('uncaughtException', (error) => {
  console.error('Uncaught Exception:', error);
  process.exit(1);
});

async function bootstrap() {
  try {
    console.log('🚀 Starting StellarEarn API...');

    // Create app with detailed logging
    const app = await NestFactory.create(AppModule, {
      logger: ['error', 'warn', 'log', 'debug', 'verbose'],
      abortOnError: false, // Don't exit on initialization errors
    });

    console.log('✅ App created successfully');

    // Apply security middleware first
    app.use(new SecurityMiddleware().use.bind(new SecurityMiddleware()));

    // Apply Helmet security headers
    const configService = app.get(ConfigService);
    app.use(helmet(getSecurityConfig(configService)));

    // Configure CORS with whitelist
    app.enableCors(getCorsConfig());

    // Global pipes for validation and sanitization
    app.useGlobalPipes(
      new SanitizationPipe(),
      new CustomValidationPipe(),
      new ValidationPipe({
        whitelist: true,
        forbidNonWhitelisted: true,
        transform: true,
        disableErrorMessages: false,
        exceptionFactory: (errors) => {
          return new Error(
            JSON.stringify({
              message: 'Validation failed',
              errors: errors.map((error) => ({
                property: error.property,
                constraints: error.constraints,
              })),
            }),
          );
        },
      }),
    );

    // Global exception filters
    app.useGlobalFilters(
      new SecurityExceptionFilter(),
      new ValidationExceptionFilter(),
    );

    console.log('✅ Middleware configured');

    // API versioning and global prefix
    app.setGlobalPrefix('api');
    app.enableVersioning({ type: VersioningType.URI, defaultVersion: '1' });

    // Swagger (centralized setup)
    setupSwagger(app, configService);

    console.log('✅ Swagger configured and versioning enabled');

    // Terminus handles SIGTERM/SIGINT: marks health checks unhealthy first,
    // drains in-flight requests, then closes the app cleanly.
    app.enableShutdownHooks();

    const port = process.env.PORT || 3001;

    console.log(`📡 Attempting to listen on port ${port}...`);

    await app.listen(port);

    console.log(`🎉 Application is running on: http://localhost:${port}`);
    console.log(
      `📚 Swagger docs available at: http://localhost:${port}/api/docs`,
    );
  } catch (error) {
    console.error('💥 Bootstrap failed:', error);
    console.error('Error stack:', error.stack);
    process.exit(1);
  }
}

// Start the application
bootstrap().catch((error) => {
  console.error('💥 Fatal error during bootstrap:', error);
  process.exit(1);
});
