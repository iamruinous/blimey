use structopt::StructOpt;
use aha_cli::aha::AhaRequest;

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
    #[structopt(name = "product")]
    Product (ProductCli),
    #[structopt(name = "release")]
    Release (ReleaseCli),
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
enum Product {
    List (ListProducts),
    Get (GetProduct),
    Create (CreateProduct),
    // Update (UpdateProduct),
}

#[derive(StructOpt, Debug)]
enum Release {
    List (ListReleases),
    Get (GetRelease),
    Create (CreateRelease),
    Update (UpdateRelease),
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
    name: String,

    #[structopt(short, long)]
    product_id: String,

    #[structopt(short="u", long="rollup-release-id")]
    parent_id: Option<String>,

    #[structopt(short="r", long="release-id")]
    release_id: String,
}

#[derive(StructOpt, Debug)]
struct CreateProduct {
    #[structopt(short, long)]
    name: String,

    #[structopt(short, long)]
    prefix: String,

    #[structopt(short="w", long="workspace-line")]
    parent_id: Option<String>,

    #[structopt(short="t", long, default_value="product_workspace")]
    workspace_type: String,
}


#[async_std::main]
async fn main() -> surf::Result<()> {
    let args = Cli::from_args();
    let aha_request = AhaRequest::new(args.token.clone(), args.subdomain.clone());
    if let Some(subcommand) = args.commands{
        match subcommand {
            Aha::Product(cfg) => {
                if let Some(productcmd) = cfg.commands {
                    match productcmd {
                        Product::List(subcfg) => {
                            aha_request.list_products(subcfg.updated_since).await?;
                        }
                        Product::Get(subcfg) => {
                            aha_request.get_product(subcfg.product_id.clone()).await?;
                        }
                        Product::Create(subcfg) => {
                            aha_request.create_product(subcfg.name.clone(), subcfg.prefix.clone(), subcfg.parent_id, subcfg.workspace_type.clone()).await?;
                        }
                    }
                }
            },
            Aha::Release(cfg) => {
                if let Some(releasecmd) = cfg.commands{
                    match releasecmd {
                        Release::List(subcfg) => {
                            aha_request.list_releases_for_product(subcfg.product_id.clone()).await?;
                        },
                        Release::Get(subcfg) => {
                            aha_request.get_release(subcfg.release_id.clone()).await?;
                        },
                        Release::Create(subcfg) => {
                            aha_request.create_release_for_product(subcfg.product_id.clone(), subcfg.name.clone()).await?;
                        },
                        Release::Update(subcfg) => {
                            aha_request.update_release_for_product(subcfg.product_id.clone(), subcfg.name.clone(), subcfg.parent_id.clone()).await?;
                        },
                    }

                }
            }
        }
    }
    Ok(()) 
}
