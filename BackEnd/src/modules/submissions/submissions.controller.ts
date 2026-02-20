import {
  Controller,
  Post,
  Get,
  Body,
  Param,
  UseGuards,
  Request,
  HttpCode,
  HttpStatus,
} from '@nestjs/common';
import {
  ApiTags,
  ApiOperation,
  ApiResponse,
  ApiBearerAuth,
  ApiParam,
} from '@nestjs/swagger';
import { SubmissionsService } from './submissions.service';
import { ApproveSubmissionDto } from './dto/approve-submission.dto';
import { RejectSubmissionDto } from './dto/reject-submission.dto';
import { JwtAuthGuard } from '../auth/guards/jwt-auth.guard';
import { VerifierGuard } from '../auth/guards/verifier.guard';

@ApiTags('Submissions')
@Controller('quests/:questId/submissions')
@UseGuards(JwtAuthGuard)
@ApiBearerAuth()
export class SubmissionsController {
  constructor(private readonly submissionsService: SubmissionsService) {}

  /**
   * Approve a submission
   * POST /quests/:questId/submissions/:id/approve
   */
  @Post(':id/approve')
  @UseGuards(VerifierGuard)
  @HttpCode(HttpStatus.OK)
  @ApiOperation({ summary: 'Approve a quest submission' })
  @ApiParam({ name: 'questId', description: 'Quest UUID' })
  @ApiParam({ name: 'id', description: 'Submission UUID' })
  @ApiResponse({
    status: 200,
    description: 'Submission approved successfully',
    schema: {
      example: {
        success: true,
        message: 'Submission approved successfully',
        data: {
          submission: {
            id: 'subm_123',
            status: 'APPROVED',
            approvedAt: '2026-01-24T08:00:00.000Z',
            approvedBy: 'verifier_1',
            quest: { id: 'quest_123', title: 'Complete KYC', rewardAmount: 10.5 },
            user: { id: 'user_123', stellarAddress: 'G...'
            },
          },
        },
      },
    },
  })
  @ApiResponse({ status: 401, description: 'Unauthorized' })
  @ApiResponse({ status: 403, description: 'Forbidden - Verifier role required' })
  async approveSubmission(
    @Param('questId') questId: string,
    @Param('id') submissionId: string,
    @Body() approveDto: ApproveSubmissionDto,
    @Request() req: Request & { user: { id: string } },
  ) {
    const verifierId = req.user.id;
    const submission = await this.submissionsService.approveSubmission(
      submissionId,
      approveDto,
      verifierId,
    );

    return {
      success: true,
      message: 'Submission approved successfully',
      data: {
        submission: {
          id: submission.id,
          status: submission.status,
          approvedAt: submission.approvedAt,
          approvedBy: submission.approvedBy,
          quest: {
            id: submission.quest.id,
            title: submission.quest.title,
            rewardAmount: submission.quest.rewardAmount,
          },
          user: {
            id: submission.user.id,
            stellarAddress: submission.user.stellarAddress,
          },
        },
      },
    };
  }

  /**
   * Reject a submission
   * POST /quests/:questId/submissions/:id/reject
   */
  @Post(':id/reject')
  @UseGuards(VerifierGuard)
  @HttpCode(HttpStatus.OK)
  @ApiOperation({ summary: 'Reject a quest submission' })
  @ApiParam({ name: 'questId', description: 'Quest UUID' })
  @ApiParam({ name: 'id', description: 'Submission UUID' })
  @ApiResponse({
    status: 200,
    description: 'Submission rejected',
    schema: {
      example: {
        success: true,
        message: 'Submission rejected',
        data: {
          submission: {
            id: 'subm_123',
            status: 'REJECTED',
            rejectedAt: '2026-01-24T09:00:00.000Z',
            rejectedBy: 'verifier_1',
            rejectionReason: 'Insufficient proof of identity',
            quest: { id: 'quest_123', title: 'Complete KYC' },
            user: { id: 'user_123' },
          },
        },
      },
    },
  })
  @ApiResponse({ status: 401, description: 'Unauthorized' })
  @ApiResponse({ status: 403, description: 'Forbidden - Verifier role required' })
  async rejectSubmission(
    @Param('questId') questId: string,
    @Param('id') submissionId: string,
    @Body() rejectDto: RejectSubmissionDto,
    @Request() req: Request & { user: { id: string } },
  ) {
    const verifierId = req.user.id;
    const submission = await this.submissionsService.rejectSubmission(
      submissionId,
      rejectDto,
      verifierId,
    );

    return {
      success: true,
      message: 'Submission rejected',
      data: {
        submission: {
          id: submission.id,
          status: submission.status,
          rejectedAt: submission.rejectedAt,
          rejectedBy: submission.rejectedBy,
          rejectionReason: submission.rejectionReason,
          quest: {
            id: submission.quest.id,
            title: submission.quest.title,
          },
          user: {
            id: submission.user.id,
          },
        },
      },
    };
  }

  /**
   * Get submission details
   * GET /quests/:questId/submissions/:id
   */
  @Get(':id')
  @ApiOperation({ summary: 'Get submission details' })
  @ApiParam({ name: 'questId', description: 'Quest UUID' })
  @ApiParam({ name: 'id', description: 'Submission UUID' })
  @ApiResponse({ status: 200, description: 'Submission details retrieved' })
  @ApiResponse({ status: 404, description: 'Submission not found' })
  async getSubmission(
    @Param('questId') questId: string,
    @Param('id') submissionId: string,
  ) {
    const submission = await this.submissionsService.findOne(submissionId);

    return {
      success: true,
      data: { submission },
    };
  }

  /**
   * Get all submissions for a quest
   * GET /quests/:questId/submissions
   */
  @Get()
  @ApiOperation({ summary: 'Get all submissions for a quest' })
  @ApiParam({ name: 'questId', description: 'Quest UUID' })
  @ApiResponse({ status: 200, description: 'Submissions retrieved' })
  async getQuestSubmissions(@Param('questId') questId: string) {
    const submissions = await this.submissionsService.findByQuest(questId);

    return {
      success: true,
      data: {
        submissions,
        total: submissions.length,
      },
    };
  }
}
