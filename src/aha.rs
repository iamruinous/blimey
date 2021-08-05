use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct AhaRequest {
    token: String,
    subdomain: String,
    url_base_str: String,
    bearer_token: String,
}

impl AhaRequest {
    pub fn new(
        token: String,
        subdomain: String,
    ) -> AhaRequest {
        AhaRequest {
            token: token.clone(),
            subdomain: token.clone(),
            url_base_str: format!("https://{}.aha.io", subdomain),
            bearer_token: format!("Bearer {}", token),
        }
    }

    pub async fn list_releases_for_product(&self, product_id: String) -> surf::Result<()> {
        let url_str = format!("{}/api/v1/products/{}/releases", self.url_base_str, product_id);
        let mut res = surf::get(url_str).header("Authorization", self.bearer_token.clone()).await?;
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
        let mut res = surf::post(url_str)
            .header("Authorization", self.bearer_token.clone())
            .body(surf::Body::from_json(data)?)
            .await?;
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
        let mut res = surf::post(url_str)
            .header("Authorization", self.bearer_token.clone())
            .body(surf::Body::from_json(data)?)
            .await?;
        println!("{}", res.body_string().await?);
        assert_eq!(res.status(), http_types::StatusCode::Ok);
        Ok(())
    }
}
