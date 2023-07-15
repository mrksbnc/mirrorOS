import { invoke } from '@tauri-apps/api';
import { InvokeArgs } from '@tauri-apps/api/tauri';

export enum IpcEvent {
	GET_EMAILS = 'get_emails',
}

export abstract class IpcChannel {
	async invoke<TResult>(command: IpcEvent, payload: InvokeArgs): Promise<TResult> {
		const channelResponse: TResult = await invoke(command, payload);
		return channelResponse;
	}
}
