import { PrismaClient } from '../../generated/prisma';
import { v4 as uuidv4 } from 'uuid';

const prisma = new PrismaClient();

export async function createPayout(overrides?: Partial<{userId: string; amount: number; status: string;}>) {
  return prisma.payout.create({
    data: {
      id: uuidv4(),
      userId: overrides?.userId ?? '',
      amount: overrides?.amount ?? 1000,
      asset: 'TOKEN',
      status: overrides?.status ?? 'PENDING',
    },
  });
}
