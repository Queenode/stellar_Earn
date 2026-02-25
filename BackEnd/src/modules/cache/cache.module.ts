import { Module, Global } from '@nestjs/common';
import { CacheModule as NestCacheModule } from '@nestjs/cache-manager';
import { ConfigModule, ConfigService } from '@nestjs/config';
import * as redisStore from 'cache-manager-redis-store';
import { CacheService } from './cache.service';

@Global()
@Module({
  imports: [
    NestCacheModule.registerAsync({
      imports: [ConfigModule],
      inject: [ConfigService],
      useFactory: async (config: ConfigService) => ({
        store: redisStore,
        host: config.get('cache.host'),
        port: config.get('cache.port'),
        password: config.get('cache.password'),
        ttl: config.get('cache.ttl'),
        max: config.get('cache.max'),
        // Fallback to in-memory if Redis unavailable
        ...(process.env.NODE_ENV === 'test' && {
          store: 'memory',
          ttl: 300,
        }),
      }),
    }),
  ],
  providers: [CacheService],
  exports: [CacheService, NestCacheModule],
})
export class CacheModule {}