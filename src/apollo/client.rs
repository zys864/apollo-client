use std::{collections::HashMap, path::PathBuf};

use anyhow::Context;
use derive_builder::Builder;
use once_cell::sync::Lazy;
use reqwest::{Client, ClientBuilder};
use tokio::io::AsyncWriteExt;

use super::models::Config;

static HTTP_CLIENT: Lazy<Client> = Lazy::new(|| ClientBuilder::new().build().unwrap());
/// Http接口
#[derive(Debug, Clone, Builder)]
pub struct ApolloClient {
    /// Apollo配置服务的地址
    pub config_server_url: String,
    /// 应用的appId
    pub app_id: String,
    /// 集群名
    pub cluster_name: String,
    /// 应用部署的机器ip
    pub ip: Option<String>,
    cache_path: String,
    release_key: Option<String>,
}

impl ApolloClient {
    pub fn new(app_id: String) -> Self {
        Self {
            config_server_url: "http://localhost:8090".to_string(),
            app_id,
            cluster_name: "default".to_string(),
            ip: None,
            cache_path: "config/".to_string(),
            release_key: None,
        }
    }
    pub fn builder() -> ApolloClientBuilder {
        ApolloClientBuilder::default()
    }
    pub async fn test_connect(&self) -> reqwest::Result<bool> {
        let resp = HTTP_CLIENT
            .get(&self.config_server_url)
            .send()
            .await?
            .status()
            == 200;
        Ok(resp)
    }
    pub async fn get_config_by_namespace<T, R>(
        &self,
        namespace: T,
        params: R,
    ) -> reqwest::Result<Config>
    where
        T: Into<Option<String>>,
        R: Into<Option<String>>,
    {
        let url = format!(
            "{}/apps/{}/clusters/{}/namespaces/{}/releases/latest",
            self.config_server_url,
            self.app_id,
            self.cluster_name,
            namespace.into().unwrap_or("application".to_string())
        );
        let request_builder = if let Some(client_ip) = params.into() {
            HTTP_CLIENT.get(url).query(&[("ip", client_ip)])
        } else {
            HTTP_CLIENT.get(url)
        };
        let config = request_builder.send().await?.json().await?;
        Ok(config)
    }
    pub async fn update_cache(
        &mut self,
        release_key: &str,
        configurations: &HashMap<String, String>,
        namespace: &str,
    ) -> crate::AnyResult<()> {
        let file_name = format!("{}_configuration_{}.txt", self.app_id, namespace);
        let path: PathBuf = [self.cache_path.clone(),file_name].iter().collect();
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .await.with_context(||format!("can't open or create file:{}",path.to_string_lossy()))?;
        file.write_all(serde_json::to_string(configurations)?.as_bytes())
            .await?;
        self.release_key = Some(release_key.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_connection() {
        let apollo = ApolloClient::new("app".to_string());
        let r = apollo.test_connect().await.unwrap();
        println!("{r}");
    }
    #[tokio::test]
    async fn test_get_config_by_namespace() {
        let mut apollo = ApolloClient::new("app".to_string());
        let r = apollo.get_config_by_namespace(None, None).await.unwrap();
        apollo.update_cache(&r.release_key,&r.configurations,"application").await.unwrap();
        println!("{:#?}", r);
    }
}
