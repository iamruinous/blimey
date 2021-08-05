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
    List (ListReleaseOpts),
    Create (CreateReleaseOpts),
    Update (UpdateReleaseOpts),
}

#[derive(StructOpt, Debug)]
struct ListReleaseOpts{
    #[structopt(short, long)]
    product_id: String,
}

#[derive(StructOpt, Debug)]
struct CreateReleaseOpts{
    #[structopt(short, long)]
    name: String,

    #[structopt(short, long)]
    product_id: String,
}

#[derive(StructOpt, Debug)]
struct UpdateReleaseOpts{
    #[structopt(short, long)]
    name: String,

    #[structopt(short, long)]
    product_id: String,

    #[structopt(short="u", long="rollup-release-id")]
    parent_id: Option<String>,

    #[structopt(short="r", long="release-id")]
    release_id: String,
}


#[async_std::main]
async fn main() -> surf::Result<()> {
    let args = Cli::from_args();
    let aha_request = AhaRequest::new(args.token.clone(), args.subdomain.clone());
    if let Some(subcommand) = args.commands{
        match subcommand {
            Aha::Release(cfg) => {
                if let Some(releasecmd) = cfg.commands{
                    match releasecmd {
                        Release::List(subcfg) => {
                            aha_request.list_releases_for_product(subcfg.product_id.clone()).await?;
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
