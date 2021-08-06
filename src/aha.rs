use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AhaRequest {
    token: String,
    subdomain: String,
}

impl AhaRequest {
    pub fn new(token: &str, subdomain: &str) -> Self {
        Self {
            token: token.into(),
            subdomain: subdomain.into(),
        }
    }

    fn build_url(&self, part: &str) -> String {
        format!("https://{}.aha.io{}", self.subdomain, part)
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

    pub async fn list_products(&self, updated_since: Option<String>) -> surf::Result<()> {
        let url_str = format!(
            "/api/v1/products?updated_since={}",
            updated_since.unwrap_or_default()
        );
        let mut res = self.get(&url_str).await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn get_product(&self, product_id: &str) -> surf::Result<()> {
        let url_str = format!("/api/v1/products/{}", product_id);
        let mut res = self.get(&url_str).await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn create_product(
        &self,
        name: &str,
        prefix: &str,
        parent_id: Option<String>,
        workspace_type: &str,
    ) -> surf::Result<()> {
        #[derive(Deserialize, Serialize)]
        struct Product {
            product: ProductData,
        }

        #[derive(Deserialize, Serialize)]
        struct ProductData {
            name: String,
            prefix: String,
            parent_id: Option<String>,
            workspace_type: String,
        }
        let url_str = "/api/v1/products";
        let data = &Product {
            product: ProductData {
                name: name.into(),
                prefix: prefix.into(),
                parent_id,
                workspace_type: workspace_type.into(),
            },
        };
        let mut res = self
            .post(&url_str)
            .body(surf::Body::from_json(data)?)
            .await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn get_release(&self, release_id: &str) -> surf::Result<()> {
        let url_str = format!("/api/v1/releases/{}", release_id);
        let mut res = self.get(&url_str).await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn list_releases_for_product(&self, product_id: &str) -> surf::Result<()> {
        let url_str = format!("/api/v1/products/{}/releases", product_id);
        let mut res = self.get(&url_str).await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn create_release_for_product(
        &self,
        product_id: &str,
        name: &str,
    ) -> surf::Result<()> {
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
        let mut res = self
            .post(&url_str)
            .body(surf::Body::from_json(data)?)
            .await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn update_release_for_product(
        &self,
        product_id: &str,
        release_id: &str,
        name: &str,
        parent_id: Option<String>,
    ) -> surf::Result<()> {
        #[derive(Deserialize, Serialize)]
        struct Release {
            release: ReleaseData,
        }

        #[derive(Deserialize, Serialize)]
        struct ReleaseData {
            name: String,
            parent_id: Option<String>,
        }
        let url_str = format!("/api/v1/products/{}/releases/{}", product_id, release_id);
        let data = &Release {
            release: ReleaseData {
                name: name.into(),
                parent_id,
            },
        };
        let mut res = self
            .put(&url_str)
            .body(surf::Body::from_json(data)?)
            .await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }
}
