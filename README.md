# Config

Config.toml
```toml
text = "Found text"
```

main.rs
```rust
use anyhow::Error;
use config::Load;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    text: Option<String>
}

fn main() -> Result<(), Error> {
    let config: Config = Load::from("Config.toml")?;
    let text = config.text.unwrap_or(
        "Found no text".to_string()
    );

    println!("{}", text);

    Ok(())
}
```
