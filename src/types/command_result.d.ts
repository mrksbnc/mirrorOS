export declare type CommandResult<T = undefined> = {
	data?: T;
	success: boolean;
	message: string;
};
