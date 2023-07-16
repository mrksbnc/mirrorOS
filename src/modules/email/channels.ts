import {
	EmailModel,
	EmailChannelInterface,
	GetEmailsChannelParams,
	UseGetEmailsChannelArgs,
} from '@/types/email_channel';
import { CommandResult } from '@/types/command_result';
import { IpcChannel, IpcEvent } from '@/core/ipc_channel';

export class GetMailsChannel extends IpcChannel implements EmailChannelInterface {
	async use({ port, host, auth, sequence }: UseGetEmailsChannelArgs): Promise<CommandResult<EmailModel[]>> {
		const getEmailsChannelParams: GetEmailsChannelParams = {
			sequence,
			port: port,
			domain: host,
			mailbox: 'INBOX',
			email: auth.user,
			password: auth.password,
		};

		return await this.invoke<CommandResult<EmailModel[]>>(IpcEvent.GetEmails, getEmailsChannelParams);
	}
}
