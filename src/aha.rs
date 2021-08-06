use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AhaRequest {
    token: String,
    subdomain: String,
    url_base_str: String,
    bearer_token: String,
}

impl AhaRequest {
    pub fn new(token: String, subdomain: String) -> Self {
        Self {
            token: token.clone(),
            subdomain: token.clone(),
            url_base_str: format!("https://{}.aha.io", subdomain),
            bearer_token: format!("Bearer {}", token),
        }
    }

    fn get(&self, url_str: String) -> surf::RequestBuilder {
        let req = surf::get(url_str).header("Authorization", self.bearer_token.clone());
        req
    }

    fn post(&self, url_str: String) -> surf::RequestBuilder {
        let req = surf::post(url_str).header("Authorization", self.bearer_token.clone());
        req
    }

    fn put(&self, url_str: String) -> surf::RequestBuilder {
        let req = surf::put(url_str).header("Authorization", self.bearer_token.clone());
        req 
    }

    pub async fn list_products(&self, updated_since: Option<String>) -> surf::Result<()> {
        let url_str = format!("{}/api/v1/products?updated_since={}", self.url_base_str, updated_since.unwrap_or_default());
        let mut res = self.get(url_str).await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn get_product(&self, product_id: String) -> surf::Result<()> {
        let url_str = format!("{}/api/v1/products/{}", self.url_base_str, product_id);
        let mut res = self.get(url_str).await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn get_release(&self, release_id: String) -> surf::Result<()> {
        let url_str = format!("{}/api/v1/releases/{}", self.url_base_str, release_id);
        let mut res = self.get(url_str).await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn list_releases_for_product(&self, product_id: String) -> surf::Result<()> {
        let url_str = format!("{}/api/v1/products/{}/releases", self.url_base_str, product_id);
        let mut res = self.get(url_str).await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn create_release_for_product(&self, product_id: String, name: String) -> surf::Result<()> {
        #[derive(Deserialize, Serialize)]
        struct CreateReleaseData {
            release: CreateReleaseDataInner,
        }

        #[derive(Deserialize, Serialize)]
        struct CreateReleaseDataInner {
            name: String,
        }
        let url_str = format!("{}/api/v1/products/{}/releases", self.url_base_str, product_id);
        let data = &CreateReleaseData{ release: CreateReleaseDataInner { name } };
        let mut res = self.post(url_str).body(surf::Body::from_json(data)?).await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }

    pub async fn update_release_for_product(&self, product_id: String, name: String, parent_id: Option<String>) -> surf::Result<()> {
        #[derive(Deserialize, Serialize)]
        struct CreateReleaseData {
            release: CreateReleaseDataInner,
        }

        #[derive(Deserialize, Serialize)]
        struct CreateReleaseDataInner {
            name: String,
            parent_id: Option<String>,
        }
        let url_str = format!("{}/api/v1/products/{}/releases", self.url_base_str, product_id);
        let data = &CreateReleaseData{ release: CreateReleaseDataInner { name, parent_id } };
        let mut res = self.put(url_str)
            .body(surf::Body::from_json(data)?)
            .await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }
}
