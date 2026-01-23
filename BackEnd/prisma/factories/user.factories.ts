import { PrismaClient } from '../../generated/prisma';
import { v4 as uuidv4 } from 'uuid';

const prisma = new PrismaClient();

export async function createUser(overrides?: Partial<{username: string; email: string; role: string; xp: number; level: number}>) {
  const id = uuidv4();
  return prisma.user.create({
    data: {
      id,
      stellarAddress: `G${id.replace(/-/g, '').slice(0, 16)}`,
      username: overrides?.username ?? `user_${id.slice(0, 5)}`,
      email: overrides?.email ?? `user_${id.slice(0, 5)}@example.com`,
      role: overrides?.role ?? 'USER',
      xp: overrides?.xp ?? 0,
      level: overrides?.level ?? 1,
    },
  });
}
