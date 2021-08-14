use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AhaRequest {
    token: String,
    subdomain: String,
    domain: String,
    protocol: String,
}

impl AhaRequest {
    pub fn new(token: &str, subdomain: &str) -> Self {
        Self {
            token: token.into(),
            subdomain: subdomain.into(),
            domain: "aha.io".to_string(),
            protocol: "https".to_string(),
        }
    }

    pub fn with_domain(token: &str, subdomain: &str, domain: &str, protocol: &str) -> Self {
        Self {
            token: token.into(),
            subdomain: subdomain.into(),
            domain: domain.into(),
            protocol: protocol.into(),
        }
    }

    fn build_url(&self, part: &str) -> String {
        format!(
            "{}://{}.{}{}",
            self.protocol, self.subdomain, self.domain, part
        )
    }

    fn add_headers(&self, res: surf::RequestBuilder) -> surf::RequestBuilder {
        res.header("Authorization", format!("Bearer {}", self.token))
    }

    fn get(&self, part: &str) -> surf::RequestBuilder {
        self.add_headers(surf::get(self.build_url(part)))
    }

    fn post(&self, part: &str) -> surf::RequestBuilder {
        self.add_headers(surf::post(self.build_url(part)))
    }

    fn put(&self, part: &str) -> surf::RequestBuilder {
        self.add_headers(surf::put(self.build_url(part)))
    }

    pub fn list_products(
        &self,
        updated_since: &Option<String>,
    ) -> surf::Result<surf::RequestBuilder> {
        let url_str = format!(
            "/api/v1/products?updated_since={}",
            updated_since.clone().unwrap_or_default()
        );
        Ok(self.get(&url_str))
    }

    pub fn get_product(&self, product_id: &str) -> surf::Result<surf::RequestBuilder> {
        let url_str = format!("/api/v1/products/{}", product_id);
        Ok(self.get(&url_str))
    }

    pub fn create_product(
        &self,
        name: &str,
        prefix: &str,
        parent_id: &Option<String>,
        workspace_type: &str,
    ) -> surf::Result<surf::RequestBuilder> {
        #[derive(Deserialize, Serialize)]
        struct Product {
            product: ProductData,
        }

        #[derive(Deserialize, Serialize)]
        struct ProductData {
            name: String,
            prefix: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            parent_id: Option<String>,
            workspace_type: String,
        }
        let url_str = "/api/v1/products";
        let data = &Product {
            product: ProductData {
                name: name.into(),
                prefix: prefix.into(),
                parent_id: parent_id.clone(),
                workspace_type: workspace_type.into(),
            },
        };
        Ok(self.post(url_str).body(surf::Body::from_json(data)?))
    }

    pub fn update_product(
        &self,
        product_id: &str,
        name: &Option<String>,
        prefix: &Option<String>,
        parent_id: &Option<String>,
    ) -> surf::Result<surf::RequestBuilder> {
        #[derive(Deserialize, Serialize)]
        struct Product {
            product: ProductData,
        }

        #[derive(Deserialize, Serialize)]
        struct ProductData {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            prefix: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            parent_id: Option<String>,
        }
        let url_str = format!("/api/v1/products/{}", product_id);
        let data = &Product {
            product: ProductData {
                name: name.clone(),
                prefix: prefix.clone(),
                parent_id: parent_id.clone(),
            },
        };
        Ok(self.put(&url_str).body(surf::Body::from_json(data)?))
    }

    pub fn get_release(&self, release_id: &str) -> surf::Result<surf::RequestBuilder> {
        let url_str = format!("/api/v1/releases/{}", release_id);
        Ok(self.get(&url_str))
    }

    pub fn list_releases_for_product(
        &self,
        product_id: &str,
    ) -> surf::Result<surf::RequestBuilder> {
        let url_str = format!("/api/v1/products/{}/releases", product_id);
        Ok(self.get(&url_str))
    }

    pub fn create_release_for_product(
        &self,
        product_id: &str,
        name: &str,
    ) -> surf::Result<surf::RequestBuilder> {
        #[derive(Deserialize, Serialize)]
        struct Release {
            release: ReleaseData,
        }

        #[derive(Deserialize, Serialize)]
        struct ReleaseData {
            name: String,
        }
        let url_str = format!("/api/v1/products/{}/releases", product_id);
        let data = &Release {
            release: ReleaseData { name: name.into() },
        };
        Ok(self.post(&url_str).body(surf::Body::from_json(data)?))
    }

    pub fn update_release_for_product(
        &self,
        product_id: &str,
        release_id: &str,
        name: &Option<String>,
        parent_id: &Option<String>,
    ) -> surf::Result<surf::RequestBuilder> {
        #[derive(Deserialize, Serialize)]
        struct Release {
            release: ReleaseData,
        }

        #[derive(Deserialize, Serialize)]
        struct ReleaseData {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            parent_id: Option<String>,
        }
        let url_str = format!("/api/v1/products/{}/releases/{}", product_id, release_id);
        let data = &Release {
            release: ReleaseData {
                name: name.clone(),
                parent_id: parent_id.clone(),
            },
        };
        Ok(self.put(&url_str).body(surf::Body::from_json(data)?))
    }

    pub fn list_features_for_product(
        &self,
        product_id: &str,
    ) -> surf::Result<surf::RequestBuilder> {
        let url_str = format!("/api/v1/products/{}/features", product_id);
        Ok(self.get(&url_str))
    }

    pub fn get_feature(&self, feature_id: &str) -> surf::Result<surf::RequestBuilder> {
        let url_str = format!("/api/v1/features/{}", feature_id);
        Ok(self.get(&url_str))
    }

    pub fn update_feature(
        &self,
        feature_id: &str,
        name: &Option<String>,
        start_date: &Option<String>,
        due_date: &Option<String>,
    ) -> surf::Result<surf::RequestBuilder> {
        #[derive(Deserialize, Serialize)]
        struct Feature {
            feature: FeatureData,
        }

        #[derive(Deserialize, Serialize)]
        struct FeatureData {
            #[serde(skip_serializing_if = "Option::is_none")]
            name: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            start_date: Option<String>,
            #[serde(skip_serializing_if = "Option::is_none")]
            due_date: Option<String>,
        }
        let url_str = format!("/api/v1/features/{}", feature_id);
        let data = &Feature {
            feature: FeatureData {
                name: name.clone(),
                start_date: start_date.clone(),
                due_date: due_date.clone(),
            },
        };
        Ok(self.put(&url_str).body(surf::Body::from_json(data)?))
    }
}
