import { IsOptional, IsEnum, IsString, IsInt, Min, Max } from 'class-validator';
import { Type } from 'class-transformer';
import { ApiPropertyOptional, ApiProperty } from '@nestjs/swagger';
import { PayoutStatus, PayoutType } from '../entities/payout.entity';

export class PayoutQueryDto {
  @ApiPropertyOptional({ description: 'Filter by Stellar address', example: 'GXXXX...' })
  @IsOptional()
  @IsString()
  stellarAddress?: string;

  @ApiPropertyOptional({ description: 'Filter by payout status', enum: PayoutStatus })
  @IsOptional()
  @IsEnum(PayoutStatus)
  status?: PayoutStatus;

  @ApiPropertyOptional({ description: 'Filter by payout type', enum: PayoutType })
  @IsOptional()
  @IsEnum(PayoutType)
  type?: PayoutType;

  @IsOptional()
  @Type(() => Number)
  @IsInt()
  @Min(1)
  page?: number = 1;

  @IsOptional()
  @Type(() => Number)
  @IsInt()
  @Min(1)
  @Max(100)
  limit?: number = 20;
}
export class PayoutResponseDto {
  @ApiProperty()
  id: string;

  @ApiProperty()
  stellarAddress: string;

  @ApiProperty()
  amount: number;

  @ApiProperty({ nullable: true })
  asset: string;

  @ApiProperty({ enum: PayoutStatus })
  status: PayoutStatus;

  @ApiProperty({ enum: PayoutType })
  type: PayoutType;

  @ApiProperty({ nullable: true })
  questId: string | null;

  @ApiProperty({ nullable: true })
  submissionId: string | null;

  @ApiProperty({ nullable: true })
  transactionHash: string | null;

  @ApiProperty({ nullable: true })
  stellarLedger: number | null;

  @ApiProperty({ nullable: true })
  failureReason: string | null;

  @ApiProperty()
  retryCount: number;

  @ApiProperty({ nullable: true })
  processedAt: Date | null;

  @ApiProperty({ nullable: true })
  claimedAt: Date | null;

  @ApiProperty()
  createdAt: Date;
}

export class PayoutHistoryResponseDto {
  @ApiProperty({ type: [PayoutResponseDto] })
  payouts: PayoutResponseDto[];

  @ApiProperty()
  total: number;

  @ApiProperty()
  page: number;

  @ApiProperty()
  limit: number;

  @ApiProperty()
  totalPages: number;
}

export class PayoutStatsDto {
  @ApiProperty()
  totalPayouts: number;

  @ApiProperty()
  totalAmount: number;

  @ApiProperty()
  pendingPayouts: number;

  @ApiProperty()
  pendingAmount: number;

  @ApiProperty()
  completedPayouts: number;

  @ApiProperty()
  completedAmount: number;

  @ApiProperty()
  failedPayouts: number;
}
