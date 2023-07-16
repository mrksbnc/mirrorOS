pub struct ApiClient {
    pub client: reqwest::Client,
    api_key: String,
}

impl ApiClient {
    pub const BASE_API_URL: &'static str = "https://api.openweathermap.org/data/2.5/";

    pub fn new(api_key: String) -> Self {
        let client = reqwest::Client::new();

        return ApiClient { client, api_key };
    }

    async fn perform_query(
        &self,
        city_lat: f32,
        city_lon: f32,
        temperature_units: TemperatureFormat,
    ) -> Result<APIResponse, reqwest::Error> {
        let query_params = &[
            ("appid", &self.api_key),
            ("lat", &city_lat.to_string()),
            ("lon", &city_lon.to_string()),
            ("units", &temperature_units.to_string()),
        ];

        let api_request = self
            .client
            .get(APIClient::BASE_API_URL)
            .query(query_params)
            .send()
            .await?
            .json::<APIResponse>()
            .await?;

        Ok(api_request)
    }
}
