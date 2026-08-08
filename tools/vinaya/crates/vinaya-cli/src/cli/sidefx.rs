use clap::{Parser, Subcommand};
use hou_variable::HoudiniVersionShort;
use sidefx_web::{
    SideFXWeb, SidefxDownloadBuildVersion, SidefxDownloadProduct, SidefxLicenseProducts,
    SidefxPlatform,
};

use super::HOUDINI_OPTIONS;
use crate::cli::custom_parser::parse_generic;
#[derive(Parser, Debug)]
pub struct Args {
    #[arg(env = "CLIENT_ID")]
    client_id: Option<String>,

    #[arg(env = "CLIENT_SECRET")]
    client_secret: Option<String>,

    #[arg(default_value = "https://www.sidefx.com/oauth2/application_token")]
    token_url: String,

    #[arg(default_value = "https://www.sidefx.com/api/")]
    api_url: String,

    #[arg(default_value_t = 5.0)]
    timeout: f32,

    #[arg(default_value_t = 3)]
    retries: u8,

    #[command(subcommand)]
    pub command: Commands,
}
#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    /// Returns a JSON list of all available daily builds.
    #[command(name = "download.get-daily-builds-list")]
    DownloadGetDailyBuildsList {
        #[arg(value_enum)]
        product: SidefxDownloadProduct,

        ///The major version of Houdini. e.g. 19.5, 20.0. Supports multiple
        /// values.
        #[arg(help_heading=HOUDINI_OPTIONS,long,value_parser = parse_generic::<HoudiniVersionShort>)]
        version: Vec<HoudiniVersionShort>,

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
        #[arg(value_enum)]
        product: SidefxDownloadProduct,

        ///The major version of Houdini, e.g. 19.5, 20.0
        #[arg(value_parser = parse_generic::<HoudiniVersionShort>)]
        version: HoudiniVersionShort,

        ///Either a specific build number, e.g. 382, or the string 'production'
        /// to get the latest production build
        #[arg(value_enum)]
        build: SidefxDownloadBuildVersion,

        /// The operating system to install Houdini on.
        ///
        /// Please note this parameter is ignored for Docker and SideFXLabs
        /// builds.
        #[arg(value_enum)]
        platform: SidefxPlatform,
    },

    ///Returns licenses and server keys for a non commercial product.
    #[command(name = "license.get_non_commercial_license")]
    GetNonCommercialLicense {
        /// Your server name.
        server_name: String,

        /// Your server code.
        server_code: String,

        ///The major version of Houdini, e.g. 19.5, 20.0.
        ///
        /// If no version is
        /// passed this function will use the latest version publicly available.
        /// Please note that only the currently supported version of Houdini are
        /// accepted.
        #[arg(help_heading=HOUDINI_OPTIONS,long,value_parser = parse_generic::<HoudiniVersionShort>)]
        version: Option<HoudiniVersionShort>,

        ///A list of non-commercial products you want to generate licenses for.
        #[arg(value_enum)]
        products: SidefxLicenseProducts,
    },
}

pub async fn execute(args: &Args) -> mischief::Result<()> {
    let sidefx_web = SideFXWeb::new(
        args.client_id
            .clone()
            .ok_or_else(|| mischief::mischief!("CLIENT_ID not set"))?
            .as_str(),
        args.client_secret
            .clone()
            .ok_or_else(|| mischief::mischief!("CLIENT_SECRET not set"))?
            .as_str(),
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
                .download_get_daily_build_download(product, &version, build, platform)
                .await?
        }
        Commands::GetNonCommercialLicense {
            server_name,
            server_code,
            version,
            products,
        } => {
            sidefx_web
                .get_non_commercial_license(&server_name, &server_code, version.as_ref(), products)
                .await?
        }
    };
    println!("{}", response.text().await?);
    Ok(())
}
