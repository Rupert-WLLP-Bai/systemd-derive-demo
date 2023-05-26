//! file: src/config.rs
//! 配置项解析模块
//!
//! 该模块提供了配置项解析的Trait和相关实现。

use std::collections::HashMap;

/// 配置项解析Trait
/// 
/// 这个trait定义了一个解析配置的方法，要求实现这个trait的类型需要实现`Default` trait，
/// 这样我们可以创建一个默认的实例然后根据提供的配置项更新这个实例的值。
pub trait Config: Default {
    /// 解析配置项和值
    ///
    /// # 参数
    ///
    /// * `config` - 配置项和值的HashMap
    ///
    /// # 返回值
    ///
    /// 返回解析后的配置项和值的Result，如果解析成功，返回Ok包含的解析后的类型，否则返回Err包含的错误信息。
    fn parse_config(config: &HashMap<String, String>) -> Result<Self, String>;
}

/// 自动生成解析配置项和值的代码的宏
/// 
/// 这个宏为提供的struct生成了Config trait的实现，实现了`parse_config`方法，
/// 该方法根据提供的配置项和值创建一个struct的实例，并填充它的字段值。
#[macro_export]
macro_rules! derive_config {
    ($struct_name:ident { $($field:ident : $ty:ty),* }) => {
        impl Config for $struct_name {
            fn parse_config(config: &HashMap<String, String>) -> Result<Self, String> {
                let mut result = Self::default();
                $(
                    if let Some(value) = config.get(stringify!($field)) {
                        match value.parse::<$ty>() {
                            Ok(parsed_value) => result.$field = parsed_value,
                            Err(_) => return Err(format!("Invalid value for {}: {}", stringify!($field), value)),
                        }
                    }
                )*
                Ok(result)
            }
        }
    };
}

