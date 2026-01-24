import { PrismaClient } from '../../generated/prisma';
import { v4 as uuidv4 } from 'uuid';

const prisma = new PrismaClient();

export async function createSubmission(overrides?: Partial<{questId: string; userId: string; status: string;}>) {
  return prisma.submission.create({
    data: {
      id: uuidv4(),
      questId: overrides?.questId ?? '',
      userId: overrides?.userId ?? '',
      proof: { file: 'sample-proof.pdf' },
      status: overrides?.status ?? 'PENDING',
    },
  });
}
