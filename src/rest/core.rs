use super::enums::Endpoint;
use reqwest::{Client, Result};
use std::time::Duration;
use url::Url;

pub struct Rest {
    pub endpoint: Url,
    pub api_key: String,
    pub timestamp_millis: usize,
    sign: String,
    pub ent: Client,
}

pub struct RestBuilder<EndpointType, KeyType, TimeType, SignType, Client> {
    endpoint: EndpointType,
    api_key: KeyType,
    timestamp_millis: TimeType,
    sign: SignType,
    client: Client,
}

impl RestBuilder<Endpoint, String, usize, SignType, Client> {
    pub fn build(self) -> Rest {
        Rest {
            endpoint: self.endpoint.to_uri(),
            api_key: self.api_key,
            timestamp_millis: self.timestamp_millis,
            client: self.client,
        }
    }

}

impl RestBuilder<(), (), (), (), Client> {
    pub fn new() -> Self {
        RestBuilder {
            endpoint: (),
            api_key: (),
            timestamp_millis: (),
            sign: (),
            client: Client::builder()
                .timeout(Duration::from_secs(5))
                .build()
                .unwrap(),
        }
    }
}

impl<EndpointType, KeyType, TimeType, SignType, Client>
    RestBuilder<EndpointType, KeyType, TimeType, SignType, Client>
{
    pub fn endpoint(
        self,
        endpoint: Endpoint,
    ) -> RestBuilder<Endpoint, KeyType, TimeType, SignType, Client> {
        RestBuilder {
            endpoint,
            api_key: self.api_key,
            timestamp_millis: self.timestamp_millis,
            sign: self.sign,
            client: self.client,
        }
    }

    pub fn key(
        self,
        api_key: String,
    ) -> RestBuilder<EndpointType, String, TimeType, SignType, Client> {
        RestBuilder {
            endpoint: self.endpoint,
            api_key,
            timestamp_millis: self.timestamp_millis,
            sign: self.sign,
            client: self.client,
        }
    }

    pub fn timestamp(
        self,
        timestamp_millis: usize,
    ) -> RestBuilder<EndpointType, KeyType, usize, SignType, Client> {
        RestBuilder {
            endpoint: self.endpoint,
            api_key: self.api_key,
            timestamp_millis,
            sign: self.sign,
            client: self.client,
        }
    }

    // pub fn sign(
    //     self,
    //     sign: String,
    // ) -> RestBuilder<EndpointType, KeyType, TimeType, String, Client> {
    //     RestBuilder {
    //         endpoint: self.endpoint,
    //         api_key: self.api_key,
    //         timestamp_millis: self.timestamp_millis,
    //         sign,
    //         client: self.client,
    //     }
    // }
}

impl Rest {
    pub fn builder() -> RestBuilder<(), (), (), (), Client> {
        RestBuilder::new()
    }

    pub async fn get(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut uri = self.endpoint.clone();
        uri.set_query(Some(&format!("api_key={}", self.api_key)));
        let resp = reqwest::get(uri).await?;
        println!("{:#?}", resp);
        Ok(())
    }

    pub async fn server_time(&self) -> Result<String, Box<dyn std::error::Error>> {
        let mut uri = self.endpoint.clone();
        uri.join("/v2/public/time");

        let resp = reqwest::get(uri).await?.json().await?;

        Ok(resp)
    }
    fn sign(&self) -> String {
        let now = SystemTime::now();
        let unix_time = now.duration_since(UNIX_EPOCH).expect("back to the future");
        let expires = unix_time.as_secs() * 1000;

        let args = {
            "api_key": self.api_key,
            "timestamp": expires,
        };
    }
}
