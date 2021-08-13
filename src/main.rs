use blimey::aha::AhaRequest;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Blimey!
///
/// Use blimey to meet all your aha.io needs.
#[structopt(name = "blimey", about = "A cli for aha.io", author)]
struct Cli {
    #[structopt(short, long, env = "AHA_CLI_SUBDOMAIN")]
    subdomain: String,

    #[structopt(short, long, env = "AHA_CLI_TOKEN")]
    token: String,

    #[structopt(short, long, default_value = "json", env = "AHA_CLI_FORMAT")]
    format: String,

    #[structopt(subcommand)]
    commands: Option<Aha>,
}

#[derive(StructOpt, Debug)]
enum Aha {
    #[structopt(
        name = "product",
        about = "create, get, list, and update aha.io products"
    )]
    Product(ProductCli),

    #[structopt(
        name = "release",
        about = "create, get, list, and update aha.io releases"
    )]
    Release(ReleaseCli),

    #[structopt(
        name = "feature",
        about = "create, get, list, and update aha.io features"
    )]
    Feature(FeatureCli),
}

#[derive(StructOpt, Debug)]
struct ProductCli {
    #[structopt(subcommand)]
    commands: Option<Product>,
}

#[derive(StructOpt, Debug)]
struct ReleaseCli {
    #[structopt(subcommand)]
    commands: Option<Release>,
}

#[derive(StructOpt, Debug)]
struct FeatureCli {
    #[structopt(subcommand)]
    commands: Option<Feature>,
}

#[derive(StructOpt, Debug)]
enum Product {
    List(ListProducts),
    Get(GetProduct),
    Create(CreateProduct),
    Update(UpdateProduct),
}

#[derive(StructOpt, Debug)]
enum Release {
    List(ListReleases),
    Get(GetRelease),
    Create(CreateRelease),
    Update(UpdateRelease),
}

#[derive(StructOpt, Debug)]
enum Feature {
    List(ListFeatures),
    Get(GetFeature),
    // Create(CreateFeature),
    // Update (UpdateFeature),
}

#[derive(StructOpt, Debug)]
struct ListProducts {
    #[structopt(short, long)]
    updated_since: Option<String>,
}

#[derive(StructOpt, Debug)]
struct GetProduct {
    #[structopt(short, long)]
    product_id: String,
}

#[derive(StructOpt, Debug)]
struct ListReleases {
    #[structopt(short, long)]
    product_id: String,
}

#[derive(StructOpt, Debug)]
struct GetRelease {
    #[structopt(short, long)]
    release_id: String,
}

#[derive(StructOpt, Debug)]
struct CreateRelease {
    #[structopt(short, long)]
    name: String,

    #[structopt(short, long)]
    product_id: String,
}

#[derive(StructOpt, Debug)]
struct UpdateRelease {
    #[structopt(short, long)]
    product_id: String,

    #[structopt(short = "r", long = "release-id")]
    release_id: String,

    #[structopt(short, long)]
    name: String,

    #[structopt(short = "u", long = "rollup-release-id")]
    parent_id: Option<String>,
}

#[derive(StructOpt, Debug)]
struct CreateProduct {
    #[structopt(short, long)]
    name: String,

    #[structopt(short = "s", long)]
    prefix: String,

    #[structopt(short = "w", long = "workspace-line")]
    parent_id: Option<String>,

    #[structopt(short = "t", long, default_value = "product_workspace")]
    workspace_type: String,
}

#[derive(StructOpt, Debug)]
struct UpdateProduct {
    #[structopt(short, long)]
    product_id: String,

    #[structopt(short, long)]
    name: String,

    #[structopt(short = "s", long)]
    prefix: String,

    #[structopt(short = "w", long = "workspace-line")]
    parent_id: Option<String>,
}

#[derive(StructOpt, Debug)]
struct ListFeatures {
    #[structopt(short, long)]
    product_id: String,
}

#[derive(StructOpt, Debug)]
struct GetFeature {
    #[structopt(short, long)]
    feature_id: String,
}

#[async_std::main]
async fn main() -> surf::Result<()> {
    let args = Cli::from_args();
    match get_request(&args.token, &args.subdomain, &args.commands) {
        Ok(req) => {
            let mut res = req.await?;
            assert_eq!(res.status(), http_types::StatusCode::Ok);
            if args.format == "json" {
                println!("{}", res.body_string().await?);
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

fn get_request(
    token: &str,
    subdomain: &str,
    commands: &Option<Aha>,
) -> surf::Result<surf::RequestBuilder> {
    let aha_request = AhaRequest::new(token, subdomain);
    if let Some(subcommand) = commands {
        match subcommand {
            Aha::Product(cfg) => {
                if let Some(productcmd) = &cfg.commands {
                    match productcmd {
                        Product::List(subcfg) => {
                            return aha_request.list_products(&subcfg.updated_since)
                        }
                        Product::Get(subcfg) => return aha_request.get_product(&subcfg.product_id),
                        Product::Create(subcfg) => {
                            return aha_request.create_product(
                                &subcfg.name,
                                &subcfg.prefix,
                                &subcfg.parent_id,
                                &subcfg.workspace_type,
                            )
                        }
                        Product::Update(subcfg) => {
                            return aha_request.update_product(
                                &subcfg.product_id,
                                &subcfg.name,
                                &subcfg.prefix,
                                &subcfg.parent_id,
                            )
                        }
                    }
                }
            }
            Aha::Release(cfg) => {
                if let Some(releasecmd) = &cfg.commands {
                    match releasecmd {
                        Release::List(subcfg) => {
                            return aha_request.list_releases_for_product(&subcfg.product_id)
                        }
                        Release::Get(subcfg) => return aha_request.get_release(&subcfg.release_id),
                        Release::Create(subcfg) => {
                            return aha_request
                                .create_release_for_product(&subcfg.product_id, &subcfg.name)
                        }
                        Release::Update(subcfg) => {
                            return aha_request.update_release_for_product(
                                &subcfg.product_id,
                                &subcfg.release_id,
                                &subcfg.name,
                                &subcfg.parent_id,
                            )
                        }
                    }
                }
            }
            Aha::Feature(cfg) => {
                if let Some(featurecmd) = &cfg.commands {
                    match featurecmd {
                        Feature::List(subcfg) => {
                            return aha_request.list_features_for_product(&subcfg.product_id)
                        }
                        Feature::Get(subcfg) => return aha_request.get_feature(&subcfg.feature_id),
                    }
                }
            }
        }
    }
    Err(surf::Error::from_str(
        surf::StatusCode::NotImplemented,
        "Invalid command",
    ))
}
