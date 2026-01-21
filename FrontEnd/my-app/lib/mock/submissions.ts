import { SubmissionStatus } from '../types/submission';
import type { Submission } from '../types/submission';

export const mockSubmissions: Submission[] = [
  {
    id: 'SUB-001',
    questId: 'quest-001',
    userId: 'user-001',
    status: SubmissionStatus.APPROVED,
    proof: { hash: '0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcd' },
    createdAt: '2024-01-15T14:32:00Z',
    updatedAt: '2024-01-15T14:32:00Z',
    quest: {
      id: 'quest-001',
      title: 'Smart Contract Security Review',
      description: 'Review and audit smart contract security',
      rewardAmount: 250,
      rewardAsset: 'XLM',
    },
  },
  {
    id: 'SUB-002',
    questId: 'quest-002',
    userId: 'user-001',
    status: SubmissionStatus.PENDING,
    proof: {},
    createdAt: '2024-01-14T09:15:00Z',
    updatedAt: '2024-01-14T09:15:00Z',
    quest: {
      id: 'quest-002',
      title: 'Documentation Update',
      description: 'Update project documentation',
      rewardAmount: 75,
      rewardAsset: 'XLM',
    },
  },
  {
    id: 'SUB-003',
    questId: 'quest-003',
    userId: 'user-001',
    status: SubmissionStatus.UNDER_REVIEW,
    proof: {},
    createdAt: '2024-01-13T16:45:00Z',
    updatedAt: '2024-01-13T16:45:00Z',
    quest: {
      id: 'quest-003',
      title: 'UI Component Library',
      description: 'Build reusable UI component library',
      rewardAmount: 150,
      rewardAsset: 'XLM',
    },
  },
  {
    id: 'SUB-004',
    questId: 'quest-004',
    userId: 'user-001',
    status: SubmissionStatus.APPROVED,
    proof: { hash: '0x567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12' },
    createdAt: '2024-01-12T11:20:00Z',
    updatedAt: '2024-01-12T11:20:00Z',
    quest: {
      id: 'quest-004',
      title: 'Bug Fix: Wallet Connection',
      description: 'Fix wallet connection issues',
      rewardAmount: 100,
      rewardAsset: 'XLM',
    },
  },
  {
    id: 'SUB-005',
    questId: 'quest-005',
    userId: 'user-001',
    status: SubmissionStatus.REJECTED,
    proof: {},
    rejectionReason: 'Proof does not meet requirements',
    createdAt: '2024-01-11T08:00:00Z',
    updatedAt: '2024-01-11T08:00:00Z',
    quest: {
      id: 'quest-005',
      title: 'API Rate Limiting',
      description: 'Implement API rate limiting',
      rewardAmount: 0,
      rewardAsset: 'XLM',
    },
  },
  // Add more mock data to reach 23 total
  ...Array.from({ length: 18 }, (_, i) => ({
    id: `SUB-${String(i + 6).padStart(3, '0')}`,
    questId: `quest-${i + 6}`,
    userId: 'user-001',
    status: SubmissionStatus.APPROVED,
    proof: { hash: `0x${Math.random().toString(16).substring(2, 66)}` },
    createdAt: new Date(2024, 0, 10 - i).toISOString(),
    updatedAt: new Date(2024, 0, 10 - i).toISOString(),
    quest: {
      id: `quest-${i + 6}`,
      title: `Quest ${i + 6}`,
      description: `Description for quest ${i + 6}`,
      rewardAmount: Math.floor(Math.random() * 200) + 50,
      rewardAsset: 'XLM',
    },
  })),
];

export function getSubmissionStats(submissions: Submission[]) {
  const total = submissions.length;
  const approved = submissions.filter((s) => s.status === SubmissionStatus.APPROVED).length;
  const pending = submissions.filter((s) => s.status === SubmissionStatus.PENDING).length;
  const underReview = submissions.filter((s) => s.status === SubmissionStatus.UNDER_REVIEW).length;
  const rejected = submissions.filter((s) => s.status === SubmissionStatus.REJECTED).length;

  return {
    total,
    approved,
    pending,
    underReview,
    rejected,
  };
}
