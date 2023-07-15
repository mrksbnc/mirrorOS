import { TransportMode } from './traffic_config_helper';

let sharedInstance: MirrorOSConfig | null = null;

export default class MirrorOSConfig {
	public static get sharedInstance(): MirrorOSConfig {
		if (sharedInstance === null) {
			sharedInstance = new MirrorOSConfig();
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
		unit: '',
	};

	public readonly traffic = {
		baseURL: '',
		transitBaseUrl: '',
		transportMode: {
			car: TransportMode.Car,
			taxi: TransportMode.Taxi,
			bicycle: TransportMode.Bicycle,
			scooter: TransportMode.Scooter,
			transit: TransportMode.Transit,
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
		outlook: {
			port: 993,
			host: '',
			auth: {
				user: '',
				password: '',
			},
		},
		iCloud: {
			port: 993,
			host: '',
			auth: {
				user: '',
				password: '',
			},
		},
	};
}
