import { Body, Controller, Param, Post } from '@nestjs/common';
import { JobsService } from './jobs.service';
import { QUEUES } from './jobs.constants';

@Controller('jobs')
export class JobsController {
  constructor(private readonly jobsService: JobsService) {}

  @Post(':queue')
  async add(@Param('queue') queue: string, @Body() body: any) {
    if (!Object.values(QUEUES).includes(queue as any)) {
      return { error: 'unknown_queue' };
    }
    const job = await this.jobsService.addJob(queue, body);
    return { id: job.id, name: job.name };
  }
}
