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
