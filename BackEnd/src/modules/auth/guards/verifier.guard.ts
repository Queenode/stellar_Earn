import {
  Injectable,
  CanActivate,
  ExecutionContext,
  ForbiddenException,
  BadRequestException,
} from '@nestjs/common';
import { InjectRepository } from '@nestjs/typeorm';
import { Repository } from 'typeorm';
import { Quest } from '../../quests/entities/quest.entity';
import { User } from '../../users/entities/user.entity';

interface RequestUser {
  id: string;
  stellarAddress: string;
  role: string;
}

interface AuthenticatedRequest {
  user: RequestUser;
  params: {
    questId: string;
  };
}

@Injectable()
export class VerifierGuard implements CanActivate {
  constructor(
    @InjectRepository(Quest)
    private questRepository: Repository<Quest>,
    @InjectRepository(User)
    private userRepository: Repository<User>,
  ) {}

  async canActivate(context: ExecutionContext): Promise<boolean> {
    const request = context.switchToHttp().getRequest<AuthenticatedRequest>();
    const user = request.user;
    const questId = request.params.questId;

    if (!user) {
      throw new ForbiddenException('User not authenticated');
    }

    if (!questId) {
      throw new BadRequestException('Quest ID is required');
    }

    const quest = await this.questRepository.findOne({
      where: { id: questId },
      relations: ['verifiers', 'creator'],
    });

    if (!quest) {
      throw new BadRequestException('Quest not found');
    }

    const isVerifier = quest.verifiers?.some(
      (v: { id: string }) => v.id === user.id,
    );
    const isCreator = quest.creator?.id === user.id;
    const isAdmin = await this.checkAdminRole(user.id);

    if (!isVerifier && !isCreator && !isAdmin) {
      throw new ForbiddenException(
        'You are not authorized to verify submissions for this quest',
      );
    }

    return true;
  }

  private async checkAdminRole(userId: string): Promise<boolean> {
    const user = await this.userRepository.findOne({
      where: { id: userId },
    });

    return user?.role === 'ADMIN';
  }
}
