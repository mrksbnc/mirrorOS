use serde::{de::Error, Deserialize, Deserializer, Serialize};

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub enum RequestType {
    CurrentWeather,
    WeatherForecast,
}
