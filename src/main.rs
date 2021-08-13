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
    List {
        #[structopt(short, long)]
        updated_since: Option<String>,
    },
    Get {
        #[structopt(short, long)]
        product_id: String,
    },
    Create {
        #[structopt(short, long)]
        name: String,

        #[structopt(short = "s", long)]
        prefix: String,

        #[structopt(short = "w", long = "workspace-line")]
        parent_id: Option<String>,

        #[structopt(short = "t", long, default_value = "product_workspace")]
        workspace_type: String,
    },
    Update {
        #[structopt(short, long)]
        product_id: String,

        #[structopt(short, long)]
        name: Option<String>,

        #[structopt(short = "s", long)]
        prefix: Option<String>,

        #[structopt(short = "w", long = "workspace-line")]
        parent_id: Option<String>,
    },
}

#[derive(StructOpt, Debug)]
enum Release {
    List {
        #[structopt(short, long)]
        product_id: String,
    },
    Get {
        #[structopt(short, long)]
        release_id: String,
    },
    Create {
        #[structopt(short, long)]
        name: String,

        #[structopt(short, long)]
        product_id: String,
    },
    Update {
        #[structopt(short, long)]
        product_id: String,

        #[structopt(short = "r", long = "release-id")]
        release_id: String,

        #[structopt(short, long)]
        name: Option<String>,

        #[structopt(short = "u", long = "rollup-release-id")]
        parent_id: Option<String>,
    },
}

#[derive(StructOpt, Debug)]
enum Feature {
    List {
        #[structopt(short, long)]
        product_id: String,
    },
    Get {
        #[structopt(short, long)]
        feature_id: String,
    },
    Update {
        #[structopt(short, long)]
        feature_id: String,

        #[structopt(short, long)]
        name: Option<String>,
    },
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
                        Product::List { updated_since } => {
                            return aha_request.list_products(updated_since)
                        }
                        Product::Get { product_id } => return aha_request.get_product(product_id),
                        Product::Create {
                            name,
                            prefix,
                            parent_id,
                            workspace_type,
                        } => {
                            return aha_request.create_product(
                                name,
                                prefix,
                                parent_id,
                                workspace_type,
                            )
                        }
                        Product::Update {
                            product_id,
                            name,
                            prefix,
                            parent_id,
                        } => {
                            return aha_request.update_product(product_id, name, prefix, parent_id)
                        }
                    }
                }
            }
            Aha::Release(cfg) => {
                if let Some(releasecmd) = &cfg.commands {
                    match releasecmd {
                        Release::List { product_id } => {
                            return aha_request.list_releases_for_product(product_id)
                        }
                        Release::Get { release_id } => return aha_request.get_release(release_id),
                        Release::Create { product_id, name } => {
                            return aha_request.create_release_for_product(product_id, name)
                        }
                        Release::Update {
                            product_id,
                            release_id,
                            name,
                            parent_id,
                        } => {
                            return aha_request.update_release_for_product(
                                product_id, release_id, name, parent_id,
                            )
                        }
                    }
                }
            }
            Aha::Feature(cfg) => {
                if let Some(featurecmd) = &cfg.commands {
                    match featurecmd {
                        Feature::List { product_id } => {
                            return aha_request.list_features_for_product(product_id)
                        }
                        Feature::Get { feature_id } => return aha_request.get_feature(feature_id),
                        Feature::Update { feature_id, name } => {
                            return aha_request.update_feature(feature_id, name)
                        }
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
