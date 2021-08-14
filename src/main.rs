use blimey::aha::AhaRequest;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
/// Blimey!
///
/// Use blimey to meet all your aha.io needs.
#[structopt(name = "blimey", about = "A cli for aha.io", author)]
struct Cli {
    /// This is your aha subdomain: <subdomain>.aha.io
    #[structopt(short, long, env = "BLIMEY_AHA_SUBDOMAIN")]
    subdomain: String,

    /// Generate an API token from your aha.io account
    #[structopt(short, long, env = "BLIMEY_AHA_TOKEN")]
    token: String,

    /// Output format. JSON is the only supported option right now
    #[structopt(short, long, default_value = "json", env = "BLIMEY_FORMAT")]
    format: String,

    #[structopt(subcommand)]
    subcommands: Option<Aha>,
}

#[derive(StructOpt, Debug)]
enum Aha {
    /// Create, get, list, and update aha.io products (workspaces)
    Product {
        #[structopt(subcommand)]
        commands: Option<Product>,
    },

    /// Create, get, list, and update aha.io releases for a given product
    Release {
        #[structopt(subcommand)]
        commands: Option<Release>,
    },

    /// Create, get, list, and update aha.io features
    Feature {
        #[structopt(subcommand)]
        commands: Option<Feature>,
    },
}

#[derive(StructOpt, Debug)]
enum Product {
    /// List all products for an account
    List {
        /// Only return product/workspaces updated since this date
        #[structopt(short, long)]
        updated_since: Option<String>,
    },
    /// Get a product by name or id
    Get {
        /// Product name or id
        #[structopt(short, long)]
        product_id: String,
    },
    /// Create a new product/workspace
    Create {
        /// Product name
        #[structopt(short, long)]
        name: String,

        /// Short product prefix
        #[structopt(short = "s", long)]
        prefix: String,

        /// The workspace line this product should belong to (optional)
        #[structopt(short = "w", long = "workspace-line")]
        parent_id: Option<String>,

        /// The workspace type: product_workspace, project_workspace, etc.
        #[structopt(short = "t", long, default_value = "product_workspace")]
        workspace_type: String,
    },
    /// Update an existing product
    Update {
        /// Product name or id
        #[structopt(short, long)]
        product_id: String,

        /// Updated product name (optional)
        #[structopt(short, long)]
        name: Option<String>,

        /// Short product prefix (optional)
        #[structopt(short = "s", long)]
        prefix: Option<String>,

        /// The workspace line this product should belong to (optional)
        #[structopt(short = "w", long = "workspace-line")]
        parent_id: Option<String>,
    },
}

#[derive(StructOpt, Debug)]
enum Release {
    /// List all releases for a given product
    List {
        /// Product name or id
        #[structopt(short, long)]
        product_id: String,
    },
    /// Get a release
    Get {
        /// Release name or id
        #[structopt(short, long)]
        release_id: String,
    },
    /// Create a new release for a given product
    Create {
        /// Release name
        #[structopt(short, long)]
        name: String,

        /// Product id this release belongs to
        #[structopt(short, long)]
        product_id: String,
    },
    /// Update a release for a given product
    Update {
        /// Product name or id
        #[structopt(short, long)]
        product_id: String,

        /// Release name or id
        #[structopt(short = "r", long = "release-id")]
        release_id: String,

        /// Updated release name (optional)
        #[structopt(short, long)]
        name: Option<String>,

        /// The updated product this release should belong to (optional)
        #[structopt(short = "u", long = "rollup-release-id")]
        parent_id: Option<String>,
    },
}

#[derive(StructOpt, Debug)]
enum Feature {
    /// List all features for a given product
    List {
        /// Product name or id
        #[structopt(short, long)]
        product_id: String,
    },
    /// Get a feature by name or id
    Get {
        /// Feature name or id
        #[structopt(short, long)]
        feature_id: String,
    },
    /// Update a feature
    Update {
        /// Feature name or id
        #[structopt(short, long)]
        feature_id: String,

        /// The updated feature name (optional)
        #[structopt(short, long)]
        name: Option<String>,

        /// The updated start date, format: YYYY-MM-DD (optional)
        #[structopt(short, long)]
        start_date: Option<String>,

        /// The updated due date, format: YYYY-MM-DD (optional)
        #[structopt(short, long)]
        due_date: Option<String>,
    },
}

#[async_std::main]
async fn main() -> surf::Result<()> {
    let args = Cli::from_args();
    let req = get_request(&args.token, &args.subdomain, &args.subcommands);
    let mut res = req.await?;
    assert_eq!(res.status(), http_types::StatusCode::Ok);
    if args.format == "json" {
        println!("{}", res.body_string().await?);
    }
    Ok(())
}

fn get_request(token: &str, subdomain: &str, subcommands: &Option<Aha>) -> surf::RequestBuilder {
    let aha_request = AhaRequest::new(token, subdomain);
    if let Some(scmd) = subcommands {
        match scmd {
            Aha::Product { commands } => {
                if let Some(cmd) = commands {
                    match cmd {
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
            Aha::Release { commands } => {
                if let Some(releasecmd) = commands {
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
            Aha::Feature { commands } => {
                if let Some(featurecmd) = commands {
                    match featurecmd {
                        Feature::List { product_id } => {
                            return aha_request.list_features_for_product(product_id)
                        }
                        Feature::Get { feature_id } => return aha_request.get_feature(feature_id),
                        Feature::Update {
                            feature_id,
                            name,
                            start_date,
                            due_date,
                        } => {
                            return aha_request
                                .update_feature(feature_id, name, start_date, due_date)
                        }
                    }
                }
            }
        }
    }
    surf::get("https://notfound")
}
