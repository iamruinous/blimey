use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "global")]
struct Cli {
    #[structopt(short, long)]
    subdomain: String,

    #[structopt(short, long)]
    token: String,

    #[structopt(short, long, default_value="json")]
    format: String,

    #[structopt(subcommand)]
    commands: Option<Aha>,
}

#[derive(StructOpt, Debug)]
enum Aha {
    #[structopt(name = "release")]
    Release (ReleaseOpts),
}

#[derive(StructOpt, Debug)]
struct ReleaseOpts{
    #[structopt(subcommand)]
    commands: Option<Release>,
}

#[derive(StructOpt, Debug)]
enum Release {
    Get (GetReleaseOpts),
    Create (CreateReleaseOpts),
}

#[derive(StructOpt, Debug)]
struct GetReleaseOpts{
    #[structopt(short, long)]
    product_id: String,
}

#[derive(StructOpt, Debug)]
struct CreateReleaseOpts{
    #[structopt(short, long)]
    name: String,

    #[structopt(short, long)]
    product_id: String,

    #[structopt(short="r", long="rollup-release-id")]
    parent_id: Option<String>
}

#[derive(Deserialize, Serialize)]
struct CreateReleaseData {
    release: CreateReleaseDataInner,
}

#[derive(Deserialize, Serialize)]
struct CreateReleaseDataInner {
    name: String,
    parent_id: Option<String>,
}

#[async_std::main]
async fn main() -> surf::Result<()> {
    let args = Cli::from_args();
    let subdomain = &args.subdomain;
    let token = &args.token;
    let bearer_token = format!("Bearer {}", token);
    let url_base_str = format!("https://{}.aha.io", subdomain);
    if let Some(subcommand) = args.commands{
        match subcommand {
            Aha::Release(cfg) => {
                if let Some(releasecmd) = cfg.commands{
                    match releasecmd {
                        Release::Get(subcfg) => {
                            let product_id = subcfg.product_id;
                            let url_str = format!("{}/api/v1/products/{}/releases", url_base_str, product_id);
                            let mut res = surf::get(url_str).header("Authorization", bearer_token).await?;
                            println!("{}", res.body_string().await?);
                            assert_eq!(res.status(), http_types::StatusCode::Ok);
                        },
                        Release::Create(subcfg) => {
                            let product_id = subcfg.product_id;
                            let parent_id = subcfg.parent_id;
                            let name = subcfg.name;
                            let url_str = format!("{}/api/v1/products/{}/releases", url_base_str, product_id);
                            let data = &CreateReleaseData{ release: CreateReleaseDataInner { name, parent_id } };
                            let mut res = surf::post(url_str)
                                .header("Authorization", bearer_token)
                                .body(surf::Body::from_json(data)?)
                                .await?;
                            println!("{}", res.body_string().await?);
                            assert_eq!(res.status(), http_types::StatusCode::Ok);
                        },
                    }

                }
            }
        }
    }
    Ok(()) 
}
