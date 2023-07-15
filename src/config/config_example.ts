import { MirrorOSConfig } from '@/types/mirror_config';
import { TransportMode } from './traffic_config_helper';

let sharedInstance: MirrorOSConfig | null = null;

export default class ImplMirrorOSConfig implements MirrorOSConfig {
	public static get sharedInstance(): MirrorOSConfig {
		if (sharedInstance === null) {
			sharedInstance = new ImplMirrorOSConfig();
		}
		return sharedInstance;
	}

	public readonly modules = [];

	public readonly apiKeys = {
		openWeatherMapKey: '',
		hereDeveloperApiKey: '',
	};

	public readonly weather = {
		baseURL: '',
		city: {
			name: '',
			lat: '',
			lon: '',
		},
		unit: 'metric' as const,
	};

	public readonly traffic = {
		baseURL: '',
		transitBaseUrl: '',
		transportMode: {
			car: TransportMode.Car as const,
			taxi: TransportMode.Taxi as const,
			bicycle: TransportMode.Bicycle as const,
			scooter: TransportMode.Scooter as const,
			transit: TransportMode.Transit as const,
		},
		origin: {
			lat: 0,
			lon: 0,
		},
		destination: {
			lat: 0,
			lon: 0,
		},
	};

	public readonly email = {
		clients: [
			{
				port: 0,
				host: '',
				secure: false,
				auth: {
					id: 0,
					email: '',
					password: '',
				},
			},
		],
	};
}
