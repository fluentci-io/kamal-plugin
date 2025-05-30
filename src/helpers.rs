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
    let version = dag().get_env("KAMAL_VERSION")?;
    let version = if version.is_empty() {
        "".to_string()
    } else {
        format!("-v {}", version)
    };

    setup_flox()?;
    dag()
        .pipeline("setup kamal")?
        .flox()?
        .with_exec(vec!["flox install ruby@3.3.4"])?
        .with_exec(vec![&format!("gem install kamal {}", version)])?
        .with_exec(vec!["[ -d $HOME/.local/bin ] || mkdir -p $HOME/.local/bin"])?
        .with_exec(vec!["ln -s `flox activate -- gem environment gemhome`/bin/kamal $HOME/.local/bin/kamal || true"])?
        .with_exec(vec!["PATH=$HOME/.local/bin:$PATH", "type", "kamal"])?
        .with_exec(vec!["type", "ruby"])?
        .stdout()?;

    dag().set_envs(vec![(
        "PATH".into(),
        format!("{}/.local/bin:{}", home, path),
    )])?;

    Ok(())
}
