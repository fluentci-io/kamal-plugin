use std::vec;

use anyhow::Error;
use fluentci_pdk::dag;

pub fn setup_flox() -> Result<(), Error> {
    let os = dag().get_os()?;
    if os == "macos" {
        dag()
      .pipeline("setup-flox")?
      .with_exec(vec![r#"type brew > /dev/null 2> /dev/null || /bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)""#])?
      .with_exec(vec!["type flox > /dev/null 2> /dev/null || brew install flox"])?
      .stdout()?;
    }
    Ok(())
}

pub fn setup() -> Result<(), Error> {
    let home = dag().get_env("HOME")?;
    let path = dag().get_env("PATH")?;

    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

    setup_flox()?;
    dag()
        .pipeline("setup kamal")?
        .flox()?
        .with_exec(vec!["flox", "install", "ruby"])?
        .with_exec(vec!["gem", "install", "kamal"])?
        .with_exec(vec!["[ -d $HOME/.local/bin ] || mkdir -p $HOME/.local/bin"])?
        .with_exec(vec![
            "ln",
            "-s",
            "`gem environment gemhome`/bin/kamal",
            "$HOME/.local/bin/kamal",
        ])?
        .stdout()?;
    Ok(())
}
