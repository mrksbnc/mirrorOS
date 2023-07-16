import { IpcEvent, IpcChannel } from '@/core';
import { CommandResult } from '@/types/command_result';
import { Email, EmailChannelInterface, GetEmailsChannelParams, UseGetEmailsChannelArgs } from '@/types/email_channel';

export class GetMailsChannel extends IpcChannel implements EmailChannelInterface {
	async use({ port, host, auth, sequence }: UseGetEmailsChannelArgs): Promise<CommandResult<Email[]>> {
		const getEmailsChannelParams: GetEmailsChannelParams = {
			sequence,
			port: port,
			domain: host,
			mailbox: 'INBOX',
			email: auth.user,
			password: auth.password,
		};

		return await this.invoke<CommandResult<Email[]>>(IpcEvent.EMAIL_GET_UNSEEN, getEmailsChannelParams);
	}
}
