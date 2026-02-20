import { IsOptional, IsString, IsInt, Min, Max } from 'class-validator';
import { Type } from 'class-transformer';
import { ApiPropertyOptional } from '@nestjs/swagger';

export class SearchUsersDto {
  @ApiPropertyOptional({ description: 'Search query string (username or address)', example: 'alice' })
  @IsOptional()
  @IsString()
  query?: string;

  @ApiPropertyOptional({ description: 'Page number', example: 1, default: 1 })
  @IsOptional()
  @IsInt()
  @Min(1)
  @Type(() => Number)
  page?: number = 1;

  @ApiPropertyOptional({ description: 'Items per page', example: 20, default: 20, minimum: 1, maximum: 100 })
  @IsOptional()
  @IsInt()
  @Min(1)
  @Max(100)
  @Type(() => Number)
  limit?: number = 20;

  @ApiPropertyOptional({ description: 'Sort field', example: 'xp', default: 'xp' })
  @IsOptional()
  @IsString()
  sortBy?: string = 'xp';

  @ApiPropertyOptional({ description: 'Sort order', example: 'DESC', default: 'DESC', enum: ['ASC', 'DESC'] })
  @IsOptional()
  @IsString()
  order?: 'ASC' | 'DESC' = 'DESC';
}
