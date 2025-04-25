// 16进制数据工具模块

/// 将16进制字符串转换为字节数组
/// 
/// # 参数
/// 
/// * `hex_str` - 空格分隔的16进制字符串，如 "AA B0 01 02"
/// 
/// # 返回值
/// 
/// 成功时返回字节数组，失败时返回错误信息
pub fn hex_string_to_bytes(hex_str: &str) -> Result<Vec<u8>, String> {
    let hex_values: Vec<&str> = hex_str.split_whitespace().collect();
    let mut bytes = Vec::with_capacity(hex_values.len());
    
    for hex_value in hex_values {
        match u8::from_str_radix(hex_value, 16) {
            Ok(byte) => bytes.push(byte),
            Err(_) => return Err(format!("无效的16进制值: {}", hex_value)),
        }
    }
    
    Ok(bytes)
}

/// 将字节数组格式化为空格分隔的16进制字符串
/// 
/// # 参数
/// 
/// * `bytes` - 字节数组
/// 
/// # 返回值
/// 
/// 格式化后的16进制字符串，如 "aa b0 01 02"
pub fn format_bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<Vec<String>>()
        .join(" ")
} 