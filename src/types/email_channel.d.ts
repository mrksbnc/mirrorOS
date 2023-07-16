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
	domain: string;
	email: string;
	password: string;
	sequence: string;
	mailbox: string;
};

export interface EmailChannelInterface {
	use({ port, host, auth, sequence }: UseGetEmailsChannelArgs): Promise<CommandResult<EmailModel[]>>;
}
