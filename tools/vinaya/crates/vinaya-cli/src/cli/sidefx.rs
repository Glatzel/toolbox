use std::env;
use std::str::FromStr;

use clap::{Parser, Subcommand};
use hou_variable::{
    HoudiniDownloadBuildVersion, HoudiniDownloadProduct, HoudiniLicenseProducts, HoudiniPlatform,
};
use sidefx_web::SideFXWeb;

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
    #[command(name = "download.get-daily-builds-list")]
    DownloadGetDailyBuildsList {
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        product: HoudiniDownloadProduct,
        #[command(flatten)]
        major: ArgMajor,
        #[command(flatten)]
        minor: ArgMinor,
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        platform: HoudiniPlatform,
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        all_build: bool,
    },
    #[command(name = "download.get-daily-build-download")]
    DownloadGetDailyBuildDownload {
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        product: HoudiniDownloadProduct,
        #[command(flatten)]
        major: ArgMajor,
        #[command(flatten)]
        minor: ArgMinor,
        #[arg(help_heading=HOUDINI_OPTIONS,long, help = "Houdini version patch")]
        build: HoudiniDownloadBuildVersion,
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        platform: HoudiniPlatform,
    },
    #[command(name = "license.get_non_commercial_license")]
    GetNonCommercialLicense {
        #[arg(long)]
        server_name: String,
        #[arg(long)]
        server_code: String,
        #[command(flatten)]
        major: Option<ArgMajor>,
        #[command(flatten)]
        minor: Option<ArgMinor>,
        #[arg(long)]
        products: String,
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
            major,
            minor,
            platform,
            all_build,
        } => {
            sidefx_web
                .download_get_daily_builds_list(
                    product,
                    major.value(),
                    minor.value(),
                    platform,
                    all_build,
                )
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
                    HoudiniLicenseProducts::from_str(&products)?,
                )
                .await?
        }
    };
    println!("{}", response.text().await?);
    Ok(())
}
