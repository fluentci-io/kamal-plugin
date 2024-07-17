# Kamal Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/kamal)](https://pkg.fluentci.io/kamal)
[![ci](https://github.com/fluentci-io/kamal-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/kamal-plugin/actions/workflows/ci.yml)

This plugin install and run [kamal](https://kamal-deploy.org/) on your CI/CD pipelines.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm kamal help
```

## Functions

| Name      | Description                                |
| --------- | ------------------------------------------ |
| accessory | Manage accessories (db/redis/search)       |
| app       | Manage application                         |
| audit     | Show audit log from servers                |
| build     | Build application image                    |
| config    | Show combined config (including secrets!)  |
| deploy    | Deploy app to servers                      |
| details   | Show details about all containers          |
| docs      | Show Kamal documentation for configuration setting | 
| env       | Manage environment files |
| envify    | Create .env by evaluating .env.erb (or .env.staging.erb -> .env.staging when using -d staging) |
| help      | Describe available commands or one specific command |
| init      | Create config stub in config/deploy.yml and env stub in .env |
| install   | Install Kamal CLI |
| lock      | Manage the deploy lock |
| prune     | Prune old application images and containers |
| redeploy  | Deploy app to servers without bootstrapping servers, starting Traefik, pruning, and registry login |
| registry | Login and -out of the image registry |
| remove   | Remove Traefik, app, accessories, and registry session from servers |
| rolback | Rollback app to VERSION |
| server  | Bootstrap servers with curl and Docker |
| setup   | Setup all accessories, push the env, and deploy app to servers |
| traefik | Manage Traefik load balancer |
| version | Show Kamal version |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/kamal@v0.1.0?wasm=1", "setup", vec!["latest"])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: kamal
    args: |
      install
- name: Show kamal help
  run: |
    flox activate -- type kamal
    fluentci run --wasm kamal version
    fluentci run --wasm kamal help
```
