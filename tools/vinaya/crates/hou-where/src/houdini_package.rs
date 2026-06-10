use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use glob::glob;
use mischief::IntoMischief;
use path_slash::PathExt;
use serde_json::{Value, json};
use validator::Validate;

use crate::{
    HOUDINI_VERSION_MAJOR_MAX, HOUDINI_VERSION_MAJOR_MIN, HOUDINI_VERSION_MINOR_MAX,
    HoudiniPreference,
};

#[derive(Debug, Clone)]
pub struct HoudiniPackage {
    pub enable: bool,
    pub name: String,
    pub json_file: PathBuf,
}

impl HoudiniPackage {
    fn json_object(json_file: &Path) -> mischief::Result<serde_json::Value> {
        let mut file: File = File::open(json_file).into_mischief()?;

        let mut contents: String = String::new();
        file.read_to_string(&mut contents).into_mischief()?;
        let json_value: Value = serde_json::from_str(&contents).into_mischief()?;
        Ok(json_value)
    }
    fn read_json(json_file: &Path) -> mischief::Result<Self> {
        let json_content: Value = Self::json_object(json_file)?;
        let pkg: HoudiniPackage = Self {
            enable: json_content["enable"].as_bool().unwrap(),
            name: json_file
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .into_owned(),
            json_file: json_file.to_path_buf(),
        };
        Ok(pkg)
    }
    fn switch_package(&mut self, enable: bool) -> mischief::Result<()> {
        self.enable = enable;
        let mut json_content: Value = Self::json_object(&self.json_file)?;
        json_content["enable"] = json!(enable);
        let mut file: File = File::create(&self.json_file).unwrap();
        serde_json::to_writer(&mut file, &json_content).unwrap();
        Ok(())
    }
}

#[derive(Debug, Clone, Validate)]
pub struct HoudiniPackageManager {
    #[validate(range(min=HOUDINI_VERSION_MAJOR_MIN,max=HOUDINI_VERSION_MAJOR_MAX))]
    pub major: u16,
    #[validate(range(max=HOUDINI_VERSION_MINOR_MAX))]
    pub minor: u16,
    pub package_dir: PathBuf,
    pub packages: Vec<HoudiniPackage>,
}
impl HoudiniPackageManager {
    pub fn from_houdini_preference(
        houdini_preference: HoudiniPreference,
    ) -> mischief::Result<Self> {
        let package_dir: PathBuf = houdini_preference.directory.join("packages");

        let packages: Vec<HoudiniPackage> = glob(
            &houdini_preference
                .directory
                .join("packages/*.json")
                .to_string_lossy(),
        )
        .expect("Failed to read glob pattern")
        .map(|f| HoudiniPackage::read_json(&f.unwrap()))
        .collect::<mischief::Result<Vec<HoudiniPackage>>>()?;

        let manager: HoudiniPackageManager = Self {
            major: houdini_preference.major,
            minor: houdini_preference.minor,
            package_dir,
            packages,
        };
        Ok(manager)
    }
    pub fn check_is_existed(&self) -> mischief::Result<&Self> {
        if !&self.package_dir.exists() {
            mischief::bail!(
                "Houdini package directory is not existed: {}",
                self.package_dir.to_slash_lossy(),
            )
        }
        Ok(self)
    }
    pub fn from_version(major: u16, minor: u16) -> mischief::Result<Self> {
        let pref = HoudiniPreference::from_version(major, minor)?;
        let manager = Self::from_houdini_preference(pref)?;
        Ok(manager)
    }
    pub fn switch_packages(&mut self, names: &[String], enable: bool) -> mischief::Result<()> {
        for p in self.packages.iter_mut() {
            clerk::debug!("Trying to switch `{}` enable to: {}", p.name, enable);
            if names.contains(&p.name) {
                clerk::debug!("Found package: {}", p.name);
                p.switch_package(enable)?;
            }
        }
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::TempDir;

    use super::*;

    fn setup_fake_pref(dir: &TempDir) -> HoudiniPreference {
        let pref_dir = dir.path().join("houdini20.5");
        let pkg_dir = pref_dir.join("packages");
        fs::create_dir_all(&pkg_dir).unwrap();

        fs::write(
            pkg_dir.join("mypackage.json"),
            r#"{"enable": true, "path": "/some/path"}"#,
        )
        .unwrap();
        fs::write(
            pkg_dir.join("otherpackage.json"),
            r#"{"enable": false, "path": "/other/path"}"#,
        )
        .unwrap();

        HoudiniPreference {
            major: 20,
            minor: 5,
            directory: pref_dir,
        }
    }

    #[test]
    fn test_from_houdini_preference() {
        let tmp = TempDir::new().unwrap();
        let pref = setup_fake_pref(&tmp);
        let manager = HoudiniPackageManager::from_houdini_preference(pref).unwrap();

        assert_eq!(manager.major, 20);
        assert_eq!(manager.minor, 5);
        assert_eq!(manager.packages.len(), 2);

        let mypackage = manager
            .packages
            .iter()
            .find(|p| p.name == "mypackage")
            .unwrap();
        assert!(mypackage.enable);

        let otherpackage = manager
            .packages
            .iter()
            .find(|p| p.name == "otherpackage")
            .unwrap();
        assert!(!otherpackage.enable);
    }

    #[test]
    fn test_check_is_existed() {
        let tmp = TempDir::new().unwrap();
        let pref = setup_fake_pref(&tmp);
        let manager = HoudiniPackageManager::from_houdini_preference(pref).unwrap();
        assert!(manager.check_is_existed().is_ok());
    }

    #[test]
    fn test_check_is_existed_missing() {
        let tmp = TempDir::new().unwrap();
        let pref = setup_fake_pref(&tmp);
        let manager = HoudiniPackageManager::from_houdini_preference(pref).unwrap();
        fs::remove_dir_all(&manager.package_dir).unwrap();
        assert!(manager.check_is_existed().is_err());
    }

    #[test]
    fn test_switch_packages() {
        let tmp = TempDir::new().unwrap();
        let pref = setup_fake_pref(&tmp);
        let mut manager = HoudiniPackageManager::from_houdini_preference(pref).unwrap();

        manager
            .switch_packages(&["mypackage".to_string()], false)
            .unwrap();
        let pkg = manager
            .packages
            .iter()
            .find(|p| p.name == "mypackage")
            .unwrap();
        assert!(!pkg.enable);

        // verify written to disk
        let content = fs::read_to_string(&pkg.json_file).unwrap();
        let json: Value = serde_json::from_str(&content).unwrap();
        assert_eq!(json["enable"], json!(false));
    }

    #[test]
    fn test_switch_packages_unmatched() {
        let tmp = TempDir::new().unwrap();
        let pref = setup_fake_pref(&tmp);
        let mut manager = HoudiniPackageManager::from_houdini_preference(pref).unwrap();

        manager
            .switch_packages(&["nonexistent".to_string()], true)
            .unwrap();
        // nothing should have changed
        let pkg = manager
            .packages
            .iter()
            .find(|p| p.name == "mypackage")
            .unwrap();
        assert!(pkg.enable);
    }
}
