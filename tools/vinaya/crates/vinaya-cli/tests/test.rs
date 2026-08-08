use mischief::IntoMischief;
use path_slash::PathExt;
use rstest::rstest;

#[rstest]
#[case(&["houdini","hfs","--version","20.5.123"])]
#[case(&["package","20.5","dir"])]
#[case(&["preference","20.5"])]
fn test_cli(#[case] args: &[&str]) -> mischief::Result<()> {
    let cmd = assert_cmd::Command::cargo_bin("vinaya")
        .into_mischief()?
        .env_clear()
        .args(args)
        .assert()
        .success();
    let mut name = String::from(std::env::consts::OS);
    name.push('-');
    name.push_str(&args.join("-"));
    let home = dirs::home_dir()
        .ok_or_else(|| mischief::mischief!(""))?
        .to_slash_lossy()
        .to_string();
    insta::with_settings!({filters => [(home.as_str(),"[HOME]")]
    }, { insta::assert_snapshot!(
        name,
        String::from_utf8_lossy(cmd.get_output().stdout.as_slice())
    )});
    Ok(())
}
