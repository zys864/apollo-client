use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub id: i64,
    pub release_key: String,
    pub name: String,
    pub app_id: String,
    pub cluster_name: String,
    pub namespace_name: String,
    #[serde(deserialize_with = "configurations_serde::deserialize")]
    pub configurations: HashMap<String, String>,
    pub comment: String,
    pub is_abandoned: bool,
    pub data_change_created_by: String,
    pub data_change_last_modified_by: String,
    pub data_change_created_time: String,
    pub data_change_last_modified_time: String,
}
mod configurations_serde {
    use std::collections::HashMap;

    use serde::{self, Deserialize, Deserializer};
    pub fn deserialize<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use serde::de::Error;
        let s = String::deserialize(deserializer)?;
        serde_json::from_str(&s).map_err(Error::custom)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let json = r#"
        {
            "appId": "app",
            "clusterName": "default",
            "comment": "",
            "configurations": "{\"test_key\":\"test_value\",\"tmp\":\"tmp_value\"}",
            "dataChangeCreatedBy": "apollo",
            "dataChangeCreatedTime": "2022-08-01T23:56:55.000+0800",
            "dataChangeLastModifiedBy": "apollo",
            "dataChangeLastModifiedTime": "2022-08-01T23:56:55.000+0800",
            "id": 3,
            "isAbandoned": false,
            "name": "20220801235653-release",
            "namespaceName": "application",
            "releaseKey": "20220801235655-e16f24aa9a85bd6f"
          }"#;
        let config = serde_json::from_str::<Config>(json).unwrap();
        println!("{config:#?}");
    }
}
