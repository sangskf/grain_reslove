// 网络通信模块
use log::{debug, error, info, warn};
use std::io::{Read, Write};
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::time::Duration;

use crate::utils::hex_utils::{format_bytes_to_hex, hex_string_to_bytes};

/// 发送16进制数据并接收响应
///
/// # 参数
///
/// * `ip` - 目标IP地址
/// * `port` - 目标端口号
/// * `data` - 16进制数据字符串，以空格分隔
/// * `timeout_ms` - 超时时间（毫秒）
///
/// # 返回值
///
/// 成功时返回响应的16进制字符串，失败时返回错误信息
pub fn send_hex_data(
    ip: &str,
    port: u16,
    data: &str,
    timeout_ms: Option<u64>,
) -> Result<String, String> {
    debug!("网络模块: 开始处理发送请求 -> {}:{}", ip, port);
    let actual_timeout = timeout_ms.unwrap_or(5000);
    debug!("网络模块: 使用超时时间: {}ms", actual_timeout);

    // 将用户输入的16进制字符串转换为字节数组
    let parse_start = std::time::Instant::now();
    let hex_data = match hex_string_to_bytes(data) {
        Ok(bytes) => {
            let elapsed = parse_start.elapsed();
            debug!(
                "网络模块: 16进制字符串解析成功, 字节数: {}, 耗时: {:?}",
                bytes.len(),
                elapsed
            );
            bytes
        }
        Err(e) => {
            let elapsed = parse_start.elapsed();
            error!("网络模块: 解析16进制数据失败: {}, 耗时: {:?}", e, elapsed);
            return Err(format!("解析16进制数据失败: {}", e));
        }
    };

    // 连接到指定的IP和端口
    let address = format!("{}:{}", ip, port);
    info!("网络模块: 开始连接到 {}", address);
    let connect_start = std::time::Instant::now();

    // 获取本地网络信息
    let local_addr_info = get_local_network_info();
    debug!("网络模块: 本地网络信息: {}", local_addr_info);

    // DNS解析检查 - 尝试将IP解析为确保IP格式正确
    let ip_parse_result = ip.parse::<IpAddr>();
    if ip_parse_result.is_err() {
        debug!("网络模块: IP地址'{}' 不是一个标准IP格式，尝试DNS解析", ip);
    } else {
        debug!("网络模块: IP地址'{}' 有效", ip);
    }

    let socket_addr = match address.parse::<SocketAddr>() {
        Ok(addr) => {
            debug!("网络模块: 地址解析成功: {}", addr);
            addr
        },
        Err(e) => {
            error!("网络模块: 解析地址失败 ({}): {}", address, e);
            return Err(format!("无效的地址格式: {} - 请检查IP地址和端口格式是否正确", e));
        }
    };

    let mut stream = match TcpStream::connect_timeout(
        &socket_addr,
        Duration::from_millis(actual_timeout),
    ) {
        Ok(stream) => {
            let elapsed = connect_start.elapsed();
            let local_addr = match stream.local_addr() {
                Ok(addr) => addr.to_string(),
                Err(_) => "未知".to_string(),
            };
            let peer_addr = match stream.peer_addr() {
                Ok(addr) => addr.to_string(),
                Err(_) => "未知".to_string(),
            };
            info!(
                "网络模块: 连接 {} 成功, 本地端口: {}, 远程端: {}, 耗时: {:?}",
                address, local_addr, peer_addr, elapsed
            );
            stream
        }
        Err(e) => {
            let elapsed = connect_start.elapsed();
            let error_msg = e.to_string().to_lowercase();
            let error_kind = std::io::Error::kind(&e);
            
            // 测试目标端口是否开放
            let is_port_common = match port {
                80 => "HTTP",
                443 => "HTTPS",
                21 => "FTP",
                22 => "SSH",
                25 => "SMTP",
                110 => "POP3",
                143 => "IMAP",
                3306 => "MySQL",
                5432 => "PostgreSQL",
                1433 => "MSSQL",
                _ => "",
            };
            let port_info = if !is_port_common.is_empty() {
                format!("{}（常见端口: {}）", port, is_port_common)
            } else {
                port.to_string()
            };

            if error_msg.contains("connection refused") {
                error!(
                    "网络模块: 连接被拒绝 {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("连接被拒绝（错误代码: {:?}）\n\n可能原因:\n1. 目标设备 {} 上的 {} 端口没有程序在监听\n2. 防火墙阻止了连接\n3. 目标设备在线但服务未启动\n\n建议:\n- 检查设备是否启动并运行服务\n- 验证端口号是否正确\n- 检查防火墙设置", error_kind, ip, port_info));
            } else if error_msg.contains("timed out") {
                error!(
                    "网络模块: 连接超时 {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("连接超时（错误代码: {:?}）\n\n可能原因:\n1. 目标设备 {} 不在线或网络拥塞\n2. 目标设备防火墙默认丢弃连接请求（不响应）\n3. 网络路径中有路由器/设备阻止了连接\n\n建议:\n- 检查设备是否开机并连接到网络\n- 尝试 ping {} 测试基本连通性\n- 检查网络设置和防火墙", error_kind, ip, ip));
            } else if error_msg.contains("network is unreachable") {
                error!(
                    "网络模块: 网络不可达 {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("网络不可达（错误代码: {:?}）\n\n可能原因:\n1. 本机网络配置问题（如无有效IP地址）\n2. 尝试连接到与本机不在同一网络的地址\n3. 路由表配置错误\n\n建议:\n- 检查本机网络连接状态\n- 确认IP地址 {} 是否在您的网络范围内\n- 检查网关和路由设置", error_kind, ip));
            } else if error_msg.contains("no route to host") {
                error!(
                    "网络模块: 无法路由到主机 {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("无法路由到主机（错误代码: {:?}）\n\n可能原因:\n1. 目标IP {} 存在但无法到达（中间路由器阻止）\n2. 路由器上的ACL或防火墙规则阻止了连接\n3. 目标主机禁用或配置错误\n\n建议:\n- 使用 traceroute {} 查看网络路径\n- 检查网络设备上的防火墙和ACL设置\n- 确认目标设备的网络配置", error_kind, ip, ip));
            } else if error_msg.contains("connection reset") {
                error!(
                    "网络模块: 连接被重置 {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("连接被重置（错误代码: {:?}）\n\n可能原因:\n1. 目标设备 {} 主动拒绝了连接\n2. 目标设备上的服务崩溃或未正确响应\n3. 防火墙或安全软件中断了连接\n\n建议:\n- 检查目标设备上的应用是否正常运行\n- 确认端口 {} 配置正确\n- 查看设备日志获取更多信息", error_kind, ip, port));
            } else if error_msg.contains("host unreachable") {
                error!(
                    "网络模块: 主机不可达 {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("主机不可达（错误代码: {:?}）\n\n可能原因:\n1. 目标IP地址 {} 不存在或未分配\n2. 本地网络设备收到ICMP主机不可达消息\n3. 目标设备已关闭或网络接口已禁用\n\n建议:\n- 确认IP地址是否正确\n- 检查目标设备的网络状态\n- 尝试 ping {} 看是否有回应", error_kind, ip, ip));
            } else {
                error!(
                    "网络模块: 连接失败 {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("连接失败: {}（错误代码: {:?}）\n\n建议:\n- 检查网络连接\n- 验证IP地址和端口\n- 确认目标设备状态", e, error_kind));
            }
        }
    };

    // 设置读写超时
    debug!("网络模块: 设置socket读写超时: {}ms", actual_timeout);
    let timeout = Duration::from_millis(actual_timeout);
    stream.set_read_timeout(Some(timeout)).map_err(|e| {
        error!("网络模块: 设置读取超时失败: {}", e);
        format!("设置读取超时失败: {}", e)
    })?;
    stream.set_write_timeout(Some(timeout)).map_err(|e| {
        error!("网络模块: 设置写入超时失败: {}", e);
        format!("设置写入超时失败: {}", e)
    })?;

    // 发送16进制数据
    debug!("网络模块: 开始发送数据, 字节数: {}", hex_data.len());
    let send_start = std::time::Instant::now();

    if let Err(e) = stream.write_all(&hex_data) {
        let elapsed = send_start.elapsed();
        let error_msg = e.to_string().to_lowercase();
        let error_kind = std::io::Error::kind(&e);

        if error_msg.contains("broken pipe") {
            error!(
                "网络模块: 连接已断开: {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                address, local_addr_info, elapsed, e, error_kind
            );
            return Err(format!("连接已断开（错误代码: {:?}）\n\n可能原因:\n1. 数据发送过程中设备断开了连接\n2. 网络连接突然中断\n3. 设备检测到无效数据并关闭了连接\n\n建议:\n- 检查设备状态和网络稳定性\n- 验证发送的数据格式是否正确", error_kind));
        } else if error_msg.contains("timed out") {
            error!(
                "网络模块: 发送数据超时: {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                address, local_addr_info, elapsed, e, error_kind
            );
            return Err(format!("发送数据超时（错误代码: {:?}）\n\n可能原因:\n1. 网络拥塞或不稳定\n2. 设备处理能力有限，无法及时接收数据\n3. 设备无响应或处于忙碌状态\n\n建议:\n- 增加超时时间\n- 检查设备状态\n- 减小发送数据量", error_kind));
        } else {
            error!(
                "网络模块: 发送数据失败: {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                address, local_addr_info, elapsed, e, error_kind
            );
            return Err(format!("发送数据失败: {}（错误代码: {:?}）\n\n建议:\n- 检查网络连接状态\n- 确认设备是否正常运行\n- 验证数据格式", e, error_kind));
        }
    }

    let send_elapsed = send_start.elapsed();
    info!(
        "网络模块: 数据发送成功, 字节数: {}, 耗时: {:?}",
        hex_data.len(),
        send_elapsed
    );

    // 读取响应
    debug!("网络模块: 等待接收响应");
    let recv_start = std::time::Instant::now();
    let mut buffer = vec![0u8; 4096]; // 增大缓冲区以处理大型响应
    let mut response_data = Vec::new();

    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            let elapsed = recv_start.elapsed();
            if bytes_read > 0 {
                response_data.extend_from_slice(&buffer[0..bytes_read]);
                info!(
                    "网络模块: 成功接收数据, 字节数: {}, 耗时: {:?}",
                    bytes_read, elapsed
                );

                // 尝试读取更多数据（如果有）
                let mut total_bytes = bytes_read;
                while let Ok(more_bytes) = stream.read(&mut buffer) {
                    if more_bytes == 0 {
                        break;
                    }
                    response_data.extend_from_slice(&buffer[0..more_bytes]);
                    total_bytes += more_bytes;
                    debug!(
                        "网络模块: 额外接收到 {} 字节数据, 总计: {}",
                        more_bytes, total_bytes
                    );
                }
            } else {
                warn!(
                    "网络模块: 设备返回了0字节数据 (连接: {} -> {}), 耗时: {:?}",
                    local_addr_info, address, elapsed
                );
                return Err(String::from("设备未返回数据\n\n可能原因:\n1. 设备收到请求但没有数据需要返回\n2. 设备协议要求特定格式的请求\n3. 设备正在处理请求但需要更长时间\n\n建议:\n- 检查发送的命令格式是否正确\n- 验证设备是否支持该命令\n- 考虑增加超时时间"));
            }
        }
        Err(e) => {
            let elapsed = recv_start.elapsed();
            let error_msg = e.to_string().to_lowercase();
            let error_kind = std::io::Error::kind(&e);

            if error_msg.contains("timed out") {
                error!(
                    "网络模块: 接收数据超时: {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("接收数据超时（错误代码: {:?}）\n\n可能原因:\n1. 设备响应时间超过了设定的超时时间({}ms)\n2. 设备处理请求需要较长时间\n3. 设备收到请求但未能完成处理\n\n建议:\n- 增加超时时间\n- 检查设备状态\n- 简化请求命令", error_kind, actual_timeout));
            } else if error_msg.contains("connection reset") {
                error!(
                    "网络模块: 接收数据时连接被重置: {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("接收数据时连接被重置（错误代码: {:?}）\n\n可能原因:\n1. 设备在处理请求过程中崩溃\n2. 设备主动关闭了连接\n3. 设备检测到异常并中断了通信\n\n建议:\n- 检查设备日志\n- 确认发送的命令格式正确\n- 验证设备固件是否需要更新", error_kind));
            } else {
                error!(
                    "网络模块: 接收数据失败: {} (本地: {}), 耗时: {:?}, 错误详情: {}, 错误类型: {:?}",
                    address, local_addr_info, elapsed, e, error_kind
                );
                return Err(format!("接收数据失败: {}（错误代码: {:?}）\n\n建议:\n- 检查网络连接状态\n- 确认设备是否仍在线\n- 尝试重新连接", e, error_kind));
            }
        }
    }

    // 将响应转换为16进制字符串
    debug!(
        "网络模块: 开始将响应转换为16进制字符串, 原始字节数: {}",
        response_data.len()
    );
    let format_start = std::time::Instant::now();
    let hex_response = format_bytes_to_hex(&response_data);
    let format_elapsed = format_start.elapsed();

    debug!(
        "网络模块: 转换为16进制完成, 字符串长度: {}, 耗时: {:?}",
        hex_response.len(),
        format_elapsed
    );

    // 计算总共耗时
    let total_elapsed = connect_start.elapsed();
    info!(
        "网络模块: 整个网络交互过程完成, 发送: {} 字节, 接收: {} 字节, 总耗时: {:?}",
        hex_data.len(),
        response_data.len(),
        total_elapsed
    );

    Ok(hex_response)
}

/// 获取本地网络信息，用于日志记录和错误诊断
fn get_local_network_info() -> String {
    let mut info = String::new();
    
    // 尝试获取本地IP地址列表
    if let Ok(interfaces) = get_if_addrs::get_if_addrs() {
        let active_interfaces: Vec<_> = interfaces
            .iter()
            .filter(|iface| {
                // 只关注IPv4地址和非回环接口
                matches!(iface.addr.ip(), IpAddr::V4(_)) && !iface.is_loopback()
            })
            .collect();
            
        if !active_interfaces.is_empty() {
            for iface in active_interfaces {
                info.push_str(&format!("{}={}, ", iface.name, iface.addr.ip()));
            }
        } else {
            // 如果没有找到活跃的网络接口，至少提供本地回环地址
            info.push_str("无活跃网络接口, 本地=127.0.0.1");
        }
    } else {
        // 如果无法获取网络接口信息，提供一个基本信息
        info.push_str("无法获取网络接口信息");
    }
    
    info
}
