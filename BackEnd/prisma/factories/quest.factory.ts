import { PrismaClient } from '../../generated/prisma';
import { v4 as uuidv4 } from 'uuid';

const prisma = new PrismaClient();

export async function createQuest(overrides?: Partial<{title: string; description: string; rewardAmount: number; status: string; createdBy: string;}>) {
  const id = uuidv4();
  return prisma.quest.create({
    data: {
      id,
      title: overrides?.title ?? `Quest ${id.slice(0, 5)}`,
      description: overrides?.description ?? 'Auto-generated quest',
      contractTaskId: `TASK_${id.slice(0, 5)}`,
      rewardAsset: 'TOKEN',
      rewardAmount: overrides?.rewardAmount ?? 1000,
      status: overrides?.status ?? 'ACTIVE',
      verifierType: 'ADMIN',
      verifierConfig: { approvalRequired: true },
      createdBy: overrides?.createdBy ?? '',
    },
  });
}
