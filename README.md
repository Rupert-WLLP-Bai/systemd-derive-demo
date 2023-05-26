# systemd-config-parser

这是一个用Rust语言编写的，解析systemd格式配置文件的库。

## 功能特性

1. 提供了一个 `Config` trait，以便你可以为自己的类型实现配置项的解析。
2. 提供了一个 `derive_config` 宏，用于自动为给定的结构体生成配置解析的代码。

## 使用方法

首先，为你的结构体实现 `Default` trait，然后使用 `derive_config` 宏，像这样：

```rust
use systemd_config_parser::config::Config;
use std::collections::HashMap;

#[derive(Default)]
struct MyConfig {
    field1: u32,
    field2: String,
}

derive_config!(MyConfig { field1: u32, field2: String });
```

然后，你就可以使用 `parse_config` 方法解析一个 `HashMap<String, String>` 来得到你的配置项了。

```rust
let mut config = HashMap::new();
config.insert("field1".to_string(), "10".to_string());
config.insert("field2".to_string(), "test".to_string());

let my_config = MyConfig::parse_config(&config).unwrap();
```

## 测试

运行以下命令以执行单元测试：

```shell
cargo test
```