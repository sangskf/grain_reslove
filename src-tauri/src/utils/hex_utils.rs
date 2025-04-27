// 16进制数据工具模块
use log::{debug, trace, warn};

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
    trace!("HEX工具: 开始解析16进制字符串，长度: {}", hex_str.len());
    
    let hex_values: Vec<&str> = hex_str.split_whitespace().collect();
    let mut bytes = Vec::with_capacity(hex_values.len());
    
    debug!("HEX工具: 分割后的16进制值数量: {}", hex_values.len());

    for (index, hex_value) in hex_values.iter().enumerate() {
        match u8::from_str_radix(hex_value, 16) {
            Ok(byte) => {
                bytes.push(byte);
                if index < 10 || index >= hex_values.len() - 10 {
                    trace!("HEX工具: 位置[{}] '{}' -> {:02x}", index, hex_value, byte);
                } else if index == 10 {
                    trace!("HEX工具: ... 中间值省略 ...");
                }
            },
            Err(e) => {
                warn!("HEX工具: 解析失败 - 位置[{}] '{}' 不是有效的16进制值: {}", 
                      index, hex_value, e);
                return Err(format!("无效的16进制值: {}", hex_value));
            }
        }
    }

    debug!("HEX工具: 16进制解析成功，共转换 {} 个字节", bytes.len());
    
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
    trace!("HEX工具: 开始将字节数组转换为16进制字符串，字节数: {}", bytes.len());
    
    let result = bytes
        .iter()
        .enumerate()
        .map(|(i, byte)| {
            if i < 10 || i >= bytes.len() - 10 {
                trace!("HEX工具: 位置[{}] {:02x} -> '{:02x}'", i, byte, byte);
            } else if i == 10 {
                trace!("HEX工具: ... 中间值省略 ...");
            }
            format!("{:02x}", byte)
        })
        .collect::<Vec<String>>()
        .join(" ");
    
    debug!("HEX工具: 字节数组转换为16进制字符串完成，结果长度: {}", result.len());
    
    result
}
