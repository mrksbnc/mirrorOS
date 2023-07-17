use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct WeatherApiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coord: Option<WeatherLocationCoord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<Vec<WeatherMeasurement>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<WeatherMainEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind: Option<WeatherWindEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain: Option<WeatherRainEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clouds: Option<WeatherCloudEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys: Option<WeatherSysEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cod: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WeatherLocationCoord {
    pub lon: f32,
    pub lat: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WeatherMeasurement {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WeatherMainEntity {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: u32,
    pub humidity: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sea_level: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnd_level: Option<u32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WeatherWindEntity {
    pub speed: f32,
    pub deg: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gust: Option<f32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WeatherRainEntity {
    pub all: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WeatherCloudEntity {
    pub all: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WeatherSysEntity {
    pub r#type: u32,
    pub id: u32,
    pub country: String,
    pub sunrise: u32,
    pub sunset: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastApiResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cod: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnt: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub list: Option<Vec<ForecastListEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<ForecastCityEntity>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastListEntity {
    pub dt: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main: Option<ForecastMainEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weather: Option<Vec<ForecastWeatherEntity>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clouds: Option<ForecastCloudEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wind: Option<ForecastWindEntity>,
    pub visibility: u32,
    pub pop: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rain: Option<ForecastRainEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys: Option<ForecastSysEntity>,
    pub dt_txt: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastMainEntity {
    pub temp: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub pressure: u32,
    pub sea_level: u32,
    pub grnd_level: u32,
    pub humidity: u32,
    pub temp_kf: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastWeatherEntity {
    pub id: u32,
    pub main: String,
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastCloudEntity {
    pub all: u32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastWindEntity {
    pub speed: f32,
    pub deg: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gust: Option<f32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastRainEntity {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "3h")]
    pub three_h: Option<f32>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastSysEntity {
    pub pod: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastCityCoordEntity {
    pub lat: f32,
    pub lon: f32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ForecastCityEntity {
    pub id: u32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coord: Option<ForecastCityCoordEntity>,
    pub country: String,
    pub population: u32,
    pub timezone: u32,
    pub sunrise: u32,
    pub sunset: u32,
}
