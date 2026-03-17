use std::env;

use clap::{Parser, Subcommand};

use super::{ArgMajor, ArgMinor, HOUDINI_OPTIONS};
use crate::hou::{HoudiniBuildVersion, HoudiniPlatform, HoudiniProduct, SideFXWeb};
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
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(name = "download.get-daily-builds-list")]
    DownloadGetDailyBuildsList {
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        product: HoudiniProduct,
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
        product: HoudiniProduct,
        #[command(flatten)]
        major: ArgMajor,
        #[command(flatten)]
        minor: ArgMinor,
        #[arg(help_heading=HOUDINI_OPTIONS,long, help = "Houdini version patch")]
        build: HoudiniBuildVersion,
        #[arg(help_heading=HOUDINI_OPTIONS,long)]
        platform: HoudiniPlatform,
    },
}

pub async fn execute(args: Args) -> mischief::Result<()> {
    let client_id = match args.client_id {
        Some(value) => value,
        None => match env::var("CLIENT_ID") {
            Ok(value) => value,
            Err(_) => dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_prompt("Client ID")
                .interact_text()
                .unwrap(),
        },
    };
    let client_secret = match args.client_secret {
        Some(value) => value,
        None => match env::var("CLIENT_SECRET") {
            Ok(value) => value,
            Err(_) => dialoguer::Input::with_theme(&dialoguer::theme::ColorfulTheme::default())
                .with_prompt("Client Secret")
                .interact_text()
                .unwrap(),
        },
    };
    let sidefx_web = SideFXWeb::new(
        client_id.as_str(),
        client_secret.as_str(),
        Some(args.token_url.as_str()),
        Some(args.api_url.as_str()),
        Some(args.timeout),
        Some(args.retries),
    )
    .await?;

    match args.command {
        Commands::DownloadGetDailyBuildsList {
            product,
            major,
            minor,
            platform,
            all_build,
        } => {
            command_download_get_daily_builds_list(
                &sidefx_web,
                product,
                major.value(),
                minor.value(),
                platform,
                all_build,
            )
            .await
        }
        Commands::DownloadGetDailyBuildDownload {
            product,
            major,
            minor,
            build,
            platform,
        } => {
            command_download_get_daily_build_download(
                &sidefx_web,
                product,
                major.value(),
                minor.value(),
                build,
                &platform,
            )
            .await
        }
    }?;
    Ok(())
}
async fn command_download_get_daily_builds_list(
    sidefx_web: &SideFXWeb,
    product: HoudiniProduct,
    major: u16,
    minor: u16,
    platform: HoudiniPlatform,
    all_build: bool,
) -> mischief::Result<()> {
    let response = sidefx_web
        .download_get_daily_builds_list(product, major, minor, platform, !all_build)
        .await?;
    println!("{}", response.text().await?);
    Ok(())
}
async fn command_download_get_daily_build_download(
    sidefx_web: &SideFXWeb,
    product: HoudiniProduct,
    major: u16,
    minor: u16,
    build: HoudiniBuildVersion,
    platform: &HoudiniPlatform,
) -> mischief::Result<()> {
    let response = sidefx_web
        .download_get_daily_build_download(product, major, minor, build, platform)
        .await?;
    println!("{}", response.text().await?);
    Ok(())
}
