import { CommandResult } from './command_result';

export declare type EmailModel = {
	uuid: String;
	from: String;
	to: String;
	subject: String;
	created_at: String;
};

export declare type UseGetEmailsChannelArgs = {
	port: number;
	host: string;
	sequence: string;
	auth: {
		user: string;
		password: string;
	};
};

export declare type GetEmailsChannelParams = {
	port: number;
	email: string;
	domain: string;
	mailbox: string;
	sequence: string;
	password: string;
};

export interface EmailChannelInterface {
	use({ port, host, auth, sequence }: UseGetEmailsChannelArgs): Promise<CommandResult<EmailModel[]>>;
}
