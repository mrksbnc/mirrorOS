import { CommandResult } from './command_result';

export declare type Email = {
	uuid: String;
	from: String;
	to: String;
	subject: String;
	created_at: String;
};

export interface EmailChannelInterface {
	use(): Promise<CommandResult<Email[]>>;
}
