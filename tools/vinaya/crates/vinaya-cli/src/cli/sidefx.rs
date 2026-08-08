use std::env;

use clap::{Parser, Subcommand};
use sidefx_web::{
    SideFXWeb, SidefxDownloadBuildVersion, SidefxDownloadProduct, SidefxLicenseProducts,
    SidefxPlatform,
};

use super::{ArgMajor, ArgMinor, HOUDINI_OPTIONS};
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
        #[arg(help_heading=HOUDINI_OPTIONS,long,value_enum)]
        product: SidefxDownloadProduct,

        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        version: Vec<String>,

        /// The operating system to install Houdini on.
        ///
        /// Does not effect Docker and SideFXLabs builds.
        #[arg(help_heading=HOUDINI_OPTIONS,long,value_enum)]
        platform: Option<SidefxPlatform>,

        #[arg(help_heading=HOUDINI_OPTIONS,long,default_value_t = false)]
        only_production: bool,
    },

    ///Returns a JSON object containing a valid temporary download link to the
    /// requested build as well as other information about the build.
    #[command(name = "download.get-daily-build-download")]
    DownloadGetDailyBuildDownload {
        #[arg(help_heading=HOUDINI_OPTIONS,long,value_enum)]
        product: SidefxDownloadProduct,

        #[command(flatten)]
        major: ArgMajor,

        #[command(flatten)]
        minor: ArgMinor,

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
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        server_name: String,

        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        server_code: String,

        #[command(flatten)]
        major: Option<ArgMajor>,

        #[command(flatten)]
        minor: Option<ArgMinor>,

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
            major,
            minor,
            build,
            platform,
        } => {
            sidefx_web
                .download_get_daily_build_download(
                    product,
                    major.value(),
                    minor.value(),
                    build,
                    &platform,
                )
                .await?
        }
        Commands::GetNonCommercialLicense {
            server_name,
            server_code,
            major,
            minor,
            products,
        } => {
            sidefx_web
                .get_non_commercial_license(
                    &server_name,
                    &server_code,
                    major.map(super::common_arg::ArgMajor::value),
                    minor.map(super::common_arg::ArgMinor::value),
                    products,
                )
                .await?
        }
    };
    println!("{}", response.text().await?);
    Ok(())
}
