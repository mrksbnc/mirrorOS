import { IpcEvent, IpcChannel } from '@/core';
import { CommandResult } from '@/types/command_result';
import { Email, EmailChannelInterface } from '@/types/email_channel';

export class GetUnseenMailsChannel extends IpcChannel implements EmailChannelInterface {
	async use(): Promise<CommandResult<Email[]>> {
		return await this.invoke<CommandResult<Email[]>>(IpcEvent.EMAIL_GET_UNSEEN, {});
	}
}
