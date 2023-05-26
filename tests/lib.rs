//! file: tests/lib.rs
//! 配置解析库的单元测试
//!
//! 这个模块包含了配置解析库的单元测试。

// 导入宏
#[macro_use]
extern crate systemd_config_parser;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    // 导入Config trait
    use systemd_config_parser::config::Config;

    // 测试用的配置结构
    #[derive(Default)]
    struct TestConfig {
        value1: u32,
        value2: String,
    }

    // 为测试用的配置结构生成Config trait的实现
    derive_config!(TestConfig { value1: u32, value2: String });

    // 测试解析配置项和值的函数
    #[test]
    fn test_parse_config() {
        let mut config = HashMap::new();
        config.insert("value1".to_string(), "10".to_string());
        config.insert("value2".to_string(), "test".to_string());

        let result = TestConfig::parse_config(&config);

        // 断言结果应该是Ok
        assert!(result.is_ok());

        // 解析出的配置项和值
        let test_config = result.unwrap();
        assert_eq!(test_config.value1, 10);
        assert_eq!(test_config.value2, "test");
    }

    // 测试解析无效配置项值的情况
    #[test]
    fn test_parse_config_invalid_value() {
        let mut config = HashMap::new();
        config.insert("value1".to_string(), "invalid".to_string());

        let result = TestConfig::parse_config(&config);

        // 断言结果应该是Err，因为"value1"的值无法转换为u32类型
        assert!(result.is_err());
    }
}
