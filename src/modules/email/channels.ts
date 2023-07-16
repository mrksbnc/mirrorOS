import { CommandResult } from '@/types/command_result';
import { IpcChannel, IpcEvent } from '@/core/ipc_channel';
import {
	EmailChannelInterface,
	EmailModel,
	GetEmailsChannelParams,
	UseGetEmailsChannelArgs,
} from '@/types/email_channel';

export class GetMailsChannel extends IpcChannel implements EmailChannelInterface {
	async use({ port, host, auth, sequence }: UseGetEmailsChannelArgs): Promise<CommandResult<EmailModel[]>> {
		const getEmailsChannelParams: GetEmailsChannelParams = {
			port: port,
			domain: host,
			email: auth.user,
			password: auth.password,
			sequence,
			mailbox: 'INBOX',
		};

		return await this.invoke<CommandResult<EmailModel[]>>(IpcEvent.GetEmails, {
			port: port,
			domain: host,
			email: auth.user,
			password: auth.password,
			sequence,
			mailbox: 'INBOX',
		});
	}
}
