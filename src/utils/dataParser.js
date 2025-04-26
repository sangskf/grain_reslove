/**
 * 解析16进制字符串为数组
 * @param {string} hexString - 空格分隔的16进制字符串
 * @returns {string[]} - 16进制值数组
 */
export function parseHexString(hexString) {
  return hexString.trim().split(/\s+/);
}

/**
 * BCD码转十进制
 * @param {number} bcd - BCD编码的值
 * @returns {number} - 十进制值
 */
function bcdToDecimal(bcd) {
  return Math.floor(bcd / 16) * 10 + (bcd % 16);
}

/**
 * 解析协议头部信息
 * @param {string[]} hexArray - 16进制数据数组
 * @returns {Object|null} - 解析后的头部信息对象
 */
export function parseProtocolHeader(hexArray) {
  console.log(hexArray)
  if (hexArray.length < 13) {
    return null;
  }
  
  // 解析时间信息 (第2-7位对应年月日时分秒)
  // 先将16进制字符串转换为数值
  const yearHex = parseInt(hexArray[2], 16);
  const monthHex = parseInt(hexArray[3], 16);
  const dayHex = parseInt(hexArray[4], 16);
  const hourHex = parseInt(hexArray[5], 16);
  const minuteHex = parseInt(hexArray[6], 16);
  const secondHex = parseInt(hexArray[7], 16);
  
  // 粮情设备通常使用BCD编码传输时间
  const year = bcdToDecimal(yearHex);
  const month = bcdToDecimal(monthHex);
  const day = bcdToDecimal(dayHex);
  const hour = bcdToDecimal(hourHex);
  const minute = bcdToDecimal(minuteHex);
  const second = bcdToDecimal(secondHex);
  
  // 格式化时间字符串 (20xx年)
  const formattedDate = `20${year.toString().padStart(2, '0')}-${month.toString().padStart(2, '0')}-${day.toString().padStart(2, '0')}`;
  const formattedTime = `${hour.toString().padStart(2, '0')}:${minute.toString().padStart(2, '0')}:${second.toString().padStart(2, '0')}`;
  const timestamp = `${formattedDate} ${formattedTime}`;
  
  // 设备ID是从第8位开始的两个字节
  const deviceId = hexArray.slice(8, 10).join(' ');
  
  return {
    header: hexArray[0] + ' ' + hexArray[1],
    timestamp: timestamp,
    date: formattedDate,
    time: formattedTime,
    year: 2000 + year,
    month: month,
    day: day,
    hour: hour,
    minute: minute,
    second: second,
    deviceId: deviceId,
    dataLength: parseInt(hexArray[10] + hexArray[11], 16),
    commandCode: hexArray[12]
  };
}

/**
 * 解析温度数据
 * @param {string[]} hexArray - 16进制数据数组
 * @returns {Array} - 解析后的温度数据数组
 */
export function parseTemperatureData(hexArray) {
  const results = [];
  
  // 从索引10开始解析温度数据（按用户要求修改）
  for (let i = 10; i < hexArray.length - 1; i += 2) {
    // 遇到FF FF表示数据结束
    if (hexArray[i] === 'ff' && hexArray[i+1] === 'ff') {
      break;
    }
    
    try {
      // 解析两个字节的温度值 (注意: 修正了高低位处理)
      const lowByte = parseInt(hexArray[i], 16);
      const highByte = parseInt(hexArray[i+1], 16);
      
      if (!isNaN(highByte) && !isNaN(lowByte)) {
        // 使用位运算构建16位有符号整数
        const tempRaw = (highByte << 8) | lowByte;
        
        // 判断符号位并正确计算温度值
        let temperature;
        if ((tempRaw & 0x8000) === 0) {
          // 正温度: 直接乘以0.0625
          temperature = tempRaw * 0.0625;
        } else {
          // 负温度: 需要进行二进制补码处理
          temperature = -((0xFFFF - tempRaw + 1) * 0.0625);
        }
        
        // 保留3位小数
        temperature = parseFloat(temperature.toFixed(3));
        
        results.push({
          sensorId: Math.floor((i - 10) / 2) + 1,  // 调整sensorId计算，基于索引10
          temperature: temperature
        });
      }
    } catch (e) {
      console.error('解析温度数据错误:', e);
    }
  }
  
  return results;
}

/**
 * 将16进制字符串转换为字节数组
 * @param {string} hexString - 空格分隔的16进制字符串
 * @returns {Uint8Array} - 字节数组
 */
export function hexToBytes(hexString) {
  const hexArray = parseHexString(hexString);
  const bytes = new Uint8Array(hexArray.length);
  
  for (let i = 0; i < hexArray.length; i++) {
    bytes[i] = parseInt(hexArray[i], 16);
  }
  
  return bytes;
} 