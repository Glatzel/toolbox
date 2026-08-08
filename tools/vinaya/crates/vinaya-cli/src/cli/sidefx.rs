use std::env;

use clap::{Parser, Subcommand};
use sidefx_web::{
    SideFXWeb, SidefxDownloadBuildVersion, SidefxDownloadProduct, SidefxLicenseProducts,
    SidefxPlatform,
};

use super::HOUDINI_OPTIONS;
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(long)]
    client_id: Option<String>,

    #[arg(long)]
    client_secret: Option<String>,

    #[arg(
        long,
        default_value = "https://www.sidefx.com/oauth2/application_token"
    )]
    token_url: String,

    #[arg(long, default_value = "https://www.sidefx.com/api/")]
    api_url: String,

    #[arg(long, default_value_t = 5.0)]
    timeout: f32,

    #[arg(long, default_value_t = 3)]
    retries: u8,

    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Returns a JSON list of all available daily builds.
    #[command(name = "download.get-daily-builds-list")]
    DownloadGetDailyBuildsList {
        #[arg(help_heading=HOUDINI_OPTIONS,value_enum)]
        product: SidefxDownloadProduct,

        ///The major version of Houdini. e.g. 19.5, 20.0.
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        version: Vec<String>,

        /// The operating system to install Houdini on.
        ///
        /// Does not effect Docker and SideFXLabs builds.
        #[arg(help_heading=HOUDINI_OPTIONS,long,value_enum)]
        platform: Option<SidefxPlatform>,

        /// If Set, will only return the production builds, else
        /// ignoring this parameter will return all builds (daily and
        /// production).
        ///
        /// Does not effect Docker and
        /// SideFXLabs builds.
        #[arg(help_heading=HOUDINI_OPTIONS,long,default_value_t = false)]
        only_production: bool,
    },

    ///Returns a JSON object containing a valid temporary download link to the
    /// requested build as well as other information about the build.
    #[command(name = "download.get-daily-build-download")]
    DownloadGetDailyBuildDownload {
        #[arg(help_heading=HOUDINI_OPTIONS,long,value_enum)]
        product: SidefxDownloadProduct,

        ///The major version of Houdini, e.g. 19.5, 20.0
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        version: String,

        ///Either a specific build number, e.g. 382, or the string 'production'
        /// to get the latest production build
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        build: SidefxDownloadBuildVersion,

        /// The operating system to install Houdini on.
        ///
        /// Please note this parameter is ignored for Docker and SideFXLabs
        /// builds.
        #[arg(help_heading=HOUDINI_OPTIONS,long,value_enum)]
        platform: SidefxPlatform,
    },

    ///Returns licenses and server keys for a non commercial product.
    #[command(name = "license.get_non_commercial_license")]
    GetNonCommercialLicense {
        /// Your server name.
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        server_name: String,

        /// Your server code.
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        server_code: String,

        ///The major version of Houdini, e.g. 19.5, 20.0.
        ///
        /// If no version is
        /// passed this function will use the latest version publicly available.
        /// Please note that only the currently supported version of Houdini are
        /// accepted.
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        version: Option<String>,

        ///A list of non-commercial products you want to generate licenses for.
        #[arg(help_heading=HOUDINI_OPTIONS,long,value_enum)]
        products: SidefxLicenseProducts,
    },
}

pub async fn execute(args: &Args) -> mischief::Result<()> {
    let client_id = args.client_id.as_ref().map_or_else(
        || {
            env::var("SIDEFX_CLIENT_ID").unwrap_or_else(|_| {
                dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Client ID")
                    .interact_text()
                    .unwrap()
            })
        },
        core::clone::Clone::clone,
    );
    let client_secret = args.client_secret.as_ref().map_or_else(
        || {
            env::var("SIDEFX_CLIENT_SECRET").unwrap_or_else(|_| {
                dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
                    .with_prompt("Client Secret")
                    .interact_text()
                    .unwrap()
            })
        },
        core::clone::Clone::clone,
    );
    let sidefx_web = SideFXWeb::new(
        client_id.as_str(),
        client_secret.as_str(),
        Some(args.token_url.as_str()),
        Some(args.api_url.as_str()),
        Some(args.timeout),
        Some(args.retries),
    )
    .await?;

    let response = match args.command.clone() {
        Commands::DownloadGetDailyBuildsList {
            product,
            version,
            platform,
            only_production,
        } => {
            sidefx_web
                .download_get_daily_builds_list(product, version, platform, Some(only_production))
                .await?
        }
        Commands::DownloadGetDailyBuildDownload {
            product,
            version,
            build,
            platform,
        } => {
            sidefx_web
                .download_get_daily_build_download(product, &version, build, &platform)
                .await?
        }
        Commands::GetNonCommercialLicense {
            server_name,
            server_code,
            version,
            products,
        } => {
            sidefx_web
                .get_non_commercial_license(
                    &server_name,
                    &server_code,
                    version.as_deref(),
                    products,
                )
                .await?
        }
    };
    println!("{}", response.text().await?);
    Ok(())
}
