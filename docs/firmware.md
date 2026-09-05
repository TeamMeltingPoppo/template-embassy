# Firmware Development

## mavlink-rs

MAVLinkのメッセージを送受信するためのRustライブラリです。MAVLinkメッセージの定義、wire-formatのparser/packerが実装されています。[mavlink-dialect](https://github.com/TeamMeltingPoppo/mavlink-dialect)のMAVLinkメッセージ定義から自動生成されています。`Cargo.toml`にmavlink-rsのリポジトリを追加することで使えるようになります。


```toml

[dependencies.mavlink]
git = "https://github.com/TeamMeltingPoppo/mavlink-rs"
tag = "0.1.0"
features = ["dialect-swingby", "embedded"]
default-features = false
```

no_std環境でも使うことができます。