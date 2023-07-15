import type { MirrorUiModule } from './mirror_ui_module';
import type { TransportMode } from '@/config/traffic_config_helper';

export declare type EmailConfigSecret = {
	id: number;
	email: string;
	password: string;
};

export declare type EmailClientConfig = {
	port: number;
	host: string;
	secure: boolean;
	auth: EmailConfigSecret;
};

export declare type WeatherCityConfig = {
	name: string;
	lat: string;
	lon: string;
};

export declare type WeatherMetricConfigOption = 'metric' | 'imperial';

export declare type AddressConfigCoordinate = {
	lat: number;
	lon: number;
};

export declare type AddressConfigSecret = {
	origin: AddressConfigCoordinate;
	destination: AddressConfigCoordinate;
};

export declare interface MirrorOSConfig {
	readonly modules: MirrorUiModule[];
	readonly apiKeys: {
		openWeatherMapKey: string;
		hereDeveloperApiKey: string;
	};
	readonly weather: {
		baseURL: string;
		city: WeatherCityConfig;
		unit: WeatherMetricConfigOption;
	};
	readonly traffic: {
		baseURL: string;
		transitBaseUrl: string;
		transportMode: {
			car: TransportMode;
			taxi: TransportMode;
			bicycle: TransportMode;
			scooter: TransportMode;
			transit: TransportMode;
		};
		origin: AddressConfigCoordinate;
		destination: AddressConfigCoordinate;
	};
	readonly email: {
		clients: EmailClientConfig[];
	};
}
