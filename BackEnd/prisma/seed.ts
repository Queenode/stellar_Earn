// prisma/seed.ts
import { PrismaClient } from '../generated/prisma';

const prisma = new PrismaClient();

async function main() {
  console.log('Seeding database...');

  // ---------- USERS ----------
  const users = [];
  for (let i = 1; i <= 5; i++) {
    const user = await prisma.user.create({
      data: {
        stellarAddress: `GUSER${i}ADDRESS`,
        username: `user${i}`,
        email: `user${i}@example.com`,
        role: i === 1 ? 'ADMIN' : 'USER',
        xp: i * 100,
        level: 1 + Math.floor(i / 2),
      },
    });
    users.push(user);
  }

  // ---------- QUESTS ----------
  const quests = [];
  for (let i = 1; i <= 5; i++) {
    const quest = await prisma.quest.create({
      data: {
        title: `Quest ${i}`,
        description: `This is the description for quest ${i}`,
        contractTaskId: `TASK_${i}`,
        rewardAsset: 'TOKEN',
        rewardAmount: 1000 * i,
        status: i % 2 === 0 ? 'INACTIVE' : 'ACTIVE',
        verifierType: 'ADMIN',
        verifierConfig: { approvalRequired: true },
        createdBy: users[i % users.length].id,
      },
    });
    quests.push(quest);
  }

  // ---------- SUBMISSIONS ----------
  for (let i = 0; i < users.length; i++) {
    const quest = quests[i % quests.length];
    await prisma.submission.create({
      data: {
        questId: quest.id,
        userId: users[i].id,
        proof: { file: `proof_${i + 1}.pdf` },
        status: 'PENDING',
      },
    });
  }

  // ---------- NOTIFICATIONS ----------
  for (let i = 0; i < users.length; i++) {
    await prisma.notification.create({
      data: {
        userId: users[i].id,
        type: 'INFO',
        title: 'Welcome!',
        message: `Hello ${users[i].username}, welcome to the platform.`,
      },
    });
  }

  // ---------- PAYOUTS ----------
  for (let i = 0; i < users.length; i++) {
    await prisma.payout.create({
      data: {
        userId: users[i].id,
        amount: 500 * (i + 1),
        asset: 'TOKEN',
        status: i % 2 === 0 ? 'PENDING' : 'COMPLETED',
      },
    });
  }

  console.log('Seeding completed!');
}

main()
  .catch((e) => {
    console.error(e);
    process.exit(1);
  })
  .finally(async () => {
    await prisma.$disconnect();
  });
