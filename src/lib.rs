use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn accessory(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("accessory")?
        .flox()?
        .with_exec(vec!["kamal", "accessory", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn app(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("app")?
        .flox()?
        .with_exec(vec!["kamal", "app", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn audit(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("audit")?
        .flox()?
        .with_exec(vec!["kamal", "audit", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn build(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("build")?
        .flox()?
        .with_exec(vec!["kamal", "build", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn config(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("config")?
        .flox()?
        .with_exec(vec!["kamal", "config", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn deploy(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("deploy")?
        .flox()?
        .with_exec(vec!["kamal", "deploy", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn details(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("details")?
        .flox()?
        .with_exec(vec!["kamal", "details", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn docs(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("docs")?
        .flox()?
        .with_exec(vec!["kamal", "docs", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn env(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("env")?
        .flox()?
        .with_exec(vec!["kamal", "env", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn envify(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("envify")?
        .flox()?
        .with_exec(vec!["kamal", "envify", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn help(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("help")?
        .flox()?
        .with_exec(vec!["kamal", "help", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn init(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("init")?
        .flox()?
        .with_exec(vec!["kamal", "init", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn install() -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("install")?
        .flox()?
        .with_exec(vec!["echo", "Successfully installed Kamal 🎉"])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn lock(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("lock")?
        .flox()?
        .with_exec(vec!["kamal", "lock", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn prune(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("prune")?
        .flox()?
        .with_exec(vec!["kamal", "prune", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn redeploy(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("redeploy")?
        .flox()?
        .with_exec(vec!["kamal", "redeploy", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn registry(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("registry")?
        .flox()?
        .with_exec(vec!["kamal", "registry", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn remove(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("remove")?
        .flox()?
        .with_exec(vec!["kamal", "remove", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn rollback(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("rollback")?
        .flox()?
        .with_exec(vec!["kamal", "rollback", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn server(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("server")?
        .flox()?
        .with_exec(vec!["kamal", "server", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn setup(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("setup")?
        .flox()?
        .with_exec(vec!["kamal", "setup", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn traefik(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("traefik")?
        .flox()?
        .with_exec(vec!["kamal", "traefik", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn version(args: String) -> FnResult<String> {
    helpers::setup()?;
    let stdout = dag()
        .pipeline("version")?
        .flox()?
        .with_exec(vec!["kamal", "version", &args])?
        .stdout()?;
    Ok(stdout)
}
