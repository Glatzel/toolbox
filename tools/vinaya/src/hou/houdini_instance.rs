use std::path::{Path, PathBuf};
use std::process;

use dunce::canonicalize;
use glob::glob;
use mischief::{IntoMischief, mischief};
use path_slash::PathExt;
use regex::Regex;
use validator::Validate;

use crate::hou::{
    HOUDINI_VERSION_MAJOR_MAX, HOUDINI_VERSION_MAJOR_MIN, HOUDINI_VERSION_MINOR_MAX,
    HOUDINI_VERSION_MINOR_MIN, HOUDINI_VERSION_PATCH_MAX, HOUDINI_VERSION_PATCH_MIN,
};

#[derive(Debug, Clone, Copy, Validate)]
pub struct HoudiniInstance {
    #[validate(range(min=HOUDINI_VERSION_MAJOR_MIN,max=HOUDINI_VERSION_MAJOR_MAX))]
    pub major: u16,
    #[validate(range(min=HOUDINI_VERSION_MINOR_MIN,max=HOUDINI_VERSION_MINOR_MAX))]
    pub minor: u16,
    #[validate(range(min=HOUDINI_VERSION_PATCH_MIN,max=HOUDINI_VERSION_PATCH_MAX))]
    pub patch: u16,
}

impl HoudiniInstance {
    // todo(mac and linux default install dir)
    pub const INSTALL_DIR: &'static str = "C:/Program Files/Side Effects Software";

    pub fn from_version_string(version_string: &str) -> mischief::Result<Self> {
        let pattern = r"^(\d+)\.(\d+)\.(\d+)$";
        clerk::debug!("Houdini version string regex pattern: {}", pattern);
        let re: Regex = Regex::new(pattern).into_mischief()?;
        let caps = re.captures(version_string).ok_or_else(|| {
            mischief!(
                "Invalid version string: {}",
                version_string,
                help = "Try a string like: 20.5.123",
            )
        })?;
        let instance: HoudiniInstance = Self {
            major: caps.get(1).unwrap().as_str().parse::<u16>().unwrap(),
            minor: caps.get(2).unwrap().as_str().parse::<u16>().unwrap(),
            patch: caps.get(3).unwrap().as_str().parse::<u16>().unwrap(),
        };
        Ok(instance)
    }

    pub fn list_installed() -> mischief::Result<Vec<Self>> {
        // glob houdini install dir
        let glob_result = glob(
            &Path::new(Self::INSTALL_DIR)
                .join("Houdini *.*.*")
                .to_string_lossy(),
        )
        .into_mischief()?;

        // map path to instance
        let mut hinstances = glob_result
            .map(|f| {
                let version_vector = f
                    .unwrap()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .split(" ")
                    .collect::<Vec<&str>>()[1]
                    .split(".")
                    .map(|f| f.parse::<u16>().unwrap())
                    .collect::<Vec<u16>>();
                Self {
                    major: version_vector[0],
                    minor: version_vector[1],
                    patch: version_vector[2],
                }
            })
            .collect::<Vec<Self>>();

        //sort instance descend
        hinstances.sort_by(|a, b| {
            b.major
                .cmp(&a.major)
                .then_with(|| b.minor.cmp(&a.minor).then_with(|| b.patch.cmp(&a.patch)))
        });

        // err if no houdini found
        if hinstances.is_empty() {
            mischief::bail!(
                "No Houdini installed.",
                help = format!("Check your Houdini Install path: \"{}\"", Self::INSTALL_DIR)
            )
        }

        Ok(hinstances)
    }

    pub fn latest_installed_version() -> mischief::Result<HoudiniInstance> {
        Ok(Self::list_installed()?[0])
    }
    pub fn check_is_installed(&self) -> mischief::Result<()> {
        let p = Path::new(Self::INSTALL_DIR).join(format!(
            "Houdini {}.{}.{}",
            self.major, self.minor, self.patch
        ));
        if !p.exists() {
            mischief::bail!(
                "Houdini {}.{}.{} is not installed.",
                self.major,
                self.minor,
                self.patch
            )
        }
        Ok(())
    }
    pub fn version_string(&self, patch: bool) -> String {
        if patch {
            format!("{}.{}.{}", self.major, self.minor, self.patch)
        } else {
            format!("{}.{}", self.major, self.minor)
        }
    }

    pub fn hfs(&self) -> PathBuf {
        Path::new(Self::INSTALL_DIR).join(format!(
            "Houdini {}.{}.{}",
            self.major, self.minor, self.patch
        ))
    }

    pub fn cmake_prefix_path(&self) -> PathBuf {
        Path::new(Self::INSTALL_DIR)
            .join(format!(
                "Houdini {}.{}.{}",
                self.major, self.minor, self.patch
            ))
            .join("toolkit")
            .join("cmake")
    }

    pub fn generate_proto(
        &self,
        python_version_major: u8,
        python_version_minor: u8,
        infiles: &Vec<&Path>,
        outfile: &Path,
    ) -> mischief::Result<()> {
        //get hython path
        let hython = self.hfs().join("bin/hython.exe");
        clerk::debug!("hython: {}", hython.to_slash_lossy());
        if !hython.exists() {
            mischief::bail!("Hython is not found. Houdini{}", self.version_string(true))
        }

        // get script
        let script = self.hfs().join(format!(
            "houdini/python{}.{}libs/generate_proto.py",
            python_version_major, python_version_minor
        ));
        clerk::debug!("generate_proto.py path: {}", script.to_slash_lossy());
        if !script.exists() {
            mischief::bail!("Script is not found: {}", script.to_slash_lossy())
        }

        // prepare inputs
        let infiles_absolute = &infiles
            .iter()
            .map(|f| canonicalize(f).into_mischief())
            .collect::<mischief::Result<Vec<PathBuf>>>()?;

        // prepare output dir
        let outdir = outfile.parent().unwrap();
        if !outdir.exists() {
            clerk::info!("Proto output directory is not existed:{}", {
                let mut abs_outdir = std::env::current_dir().into_mischief()?;
                abs_outdir.push(outdir);
                abs_outdir.to_slash_lossy().to_string()
            });
            clerk::info!("Trying to create Proto output directory.");
            std::fs::create_dir_all(outdir).into_mischief()?;
        }
        clerk::debug!(
            "Proto output directory: {}",
            canonicalize(outdir).into_mischief()?.to_slash_lossy()
        );

        // execute
        let cmd_result = &process::Command::new(hython)
            .arg(script)
            .args(infiles_absolute)
            .arg(outfile)
            .output()
            .into_mischief()?;

        if cmd_result.status.success() {
            println!("Proto header generated successfully!");
            println!(
                "[ {}] -> {}",
                infiles_absolute
                    .iter()
                    .map(|f| f.to_slash_lossy().to_string())
                    .collect::<Vec<String>>()
                    .join(", "),
                canonicalize(outfile).into_mischief()?.to_slash_lossy()
            );
            // Print the command's stdout
            println!("{}", String::from_utf8_lossy(&cmd_result.stdout));
        } else {
            mischief::bail!("{}", String::from_utf8_lossy(&cmd_result.stderr))
        }
        Ok(())
    }
}
