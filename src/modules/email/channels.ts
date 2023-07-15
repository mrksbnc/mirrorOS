import { CommandResult } from '@/types/command_result';
import { IpcChannel, IpcEvent } from '@/core/ipc_channel';
import { GetEmailsChannelParams, UseGetEmailsChannelArgs } from '@/types/email_channel';

export class GetMailsChannel extends IpcChannel {
	async use({ port, host, auth }: UseGetEmailsChannelArgs): Promise<CommandResult<boolean>> {
		const getEmailsChannelParams: GetEmailsChannelParams = {
			port: port,
			domain: host,
			email: auth.user,
			password: auth.password,
			sequence: '1:*',
			mailbox: 'INBOX',
		};

		console.log(getEmailsChannelParams);

		const response = await this.invoke<CommandResult<boolean>>(IpcEvent.GET_EMAILS, {
			port: port,
			domain: host,
			email: auth.user,
			password: auth.password,
			sequence: '1:*',
			mailbox: 'INBOX',
		});
		return response;
	}
}
