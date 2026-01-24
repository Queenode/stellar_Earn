import { PrismaClient } from '../../generated/prisma';
import { v4 as uuidv4 } from 'uuid';

const prisma = new PrismaClient();

export async function createNotification(overrides?: Partial<{userId: string; title: string; message: string; type: string;}>) {
  return prisma.notification.create({
    data: {
      id: uuidv4(),
      userId: overrides?.userId ?? '',
      type: overrides?.type ?? 'INFO',
      title: overrides?.title ?? 'Notification',
      message: overrides?.message ?? 'This is a test notification',
    },
  });
}
