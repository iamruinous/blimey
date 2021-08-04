use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    #[structopt(short, long)]
    subdomain: Option<String>,

    #[structopt(short, long)]
    token: Option<String>,

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
    product_id: Option<String>,
}

#[derive(StructOpt, Debug)]
struct CreateReleaseOpts{
    #[structopt(short, long)]
    product_id: Option<String>,

    #[structopt(short, long)]
    name: Option<String>,
}

#[derive(Deserialize, Serialize)]
struct ReleaseData {
    name: String
}

#[async_std::main]
async fn main() -> surf::Result<()> {
    let args = Cli::from_args();
    let subdomain = &args.subdomain.unwrap();
    let token = &args.token.unwrap();
    let bearer_token = format!("Bearer {}", token);
    if let Some(subcommand) = args.commands{
        match subcommand {
            Aha::Release(cfg) => {
                if let Some(releasecmd) = cfg.commands{
                    match releasecmd {
                        Release::Get(rcfg) => {
                            println!("{:?}", rcfg);
                        },
                        Release::Create(rcfg) => {
                            println!("{:?}", rcfg);
                        },
                    }

                }
            }
                //Aha::GetRelease => {
                //    let product_id = cfg.product_id.unwrap();
                //    let url_str = format!("https://{}.aha.io/api/v1/products/{}/releases", subdomain, product_id);
                //    let mut res = surf::get(url_str).header("Authorization", bearer_token).await?;
                //    println!("{}", res.body_string().await?);
                //    assert_eq!(res.status(), http_types::StatusCode::Ok);
                //},
                // Aha::CreateRelease(cfg) => {
                //     let product_id = cfg.product_id.unwrap();
                //     let name = cfg.name.unwrap();
                //     let url_str = format!("https://{}.aha.io/api/v1/products/{}/releases", subdomain, product_id);
                //     let data = &ReleaseData{ name };
                //     let mut res = surf::post(url_str).header("Authorization", bearer_token).body(surf::Body::from_json(data)?).await?;
                //     println!("{}", res.body_string().await?);
                //     assert_eq!(res.status(), http_types::StatusCode::Ok);
                // },
            }
        }
        Ok(()) 
    }

