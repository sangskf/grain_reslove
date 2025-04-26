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
 * @param {Object} config - 配置参数
 * @returns {Object|null} - 解析后的头部信息对象
 */
export function parseProtocolHeader(hexArray, config = null) {
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
  
  // 从协议提供的数据长度中获取，但可能需要根据配置进行调整
  const rawDataLength = parseInt(hexArray[10] + hexArray[11], 16);
  
  // 数据长度校正，根据配置和数据量决定
  let adjustedDataLength = rawDataLength;
  
  // 如果提供了配置，则基于传感器点数调整数据长度
  if (config && config.totalPoints) {
    const totalPoints = config.totalPoints;
    // 如果传感器点数小于512，则使用1068字节
    // 否则，检查实际数据长度
    if (totalPoints < 512) {
      // 如果协议头声明的数据长度明显不足，但实际数据长度足够
      if (rawDataLength < 1068 && hexArray.length >= 1068 + 13) {
        adjustedDataLength = 1068;
      }
    } else {
      // 大量传感器点的情况，检查是否有足够的数据
      if (hexArray.length >= 2136 + 13) {
        adjustedDataLength = 2136;
      }
    }
  }
  
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
    rawDataLength: rawDataLength,
    dataLength: adjustedDataLength,
    commandCode: hexArray[12]
  };
}

/**
 * 解析温度数据
 * @param {string[]} hexArray - 16进制数据数组
 * @param {Object} config - 配置参数
 * @returns {Array} - 解析后的温度数据数组
 */
export function parseTemperatureData(hexArray, config = null) {
  const results = [];
  
  // 计算应该解析多少个温度值
  let dataBytesToParse = hexArray.length;
  
  // 如果有配置，并且知道数据长度
  if (config && config.dataSize) {
    // 只解析配置指定的数据长度
    dataBytesToParse = Math.min(10 + config.dataSize, hexArray.length);
  }
  
  // 从索引10开始解析温度数据，跳过头部
  for (let i = 10; i < dataBytesToParse - 1; i += 2) {
    // 遇到FF FF表示数据结束
    if (hexArray[i] === 'ff' && hexArray[i+1] === 'ff') {
      break;
    }
    
    try {
      // 解析两个字节的温度值
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
          sensorId: Math.floor((i - 10) / 2) + 1,  // 调整sensorId计算，从索引10开始
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

/**
 * 特定解析1034-1057位置的温湿度数据
 * @param {string[]} hexArray - 16进制数据数组
 * @returns {Object} - 解析后的温湿度数据
 */
export function parseSpecificRegion(hexArray) {
  // 检查数据长度是否足够
  if (hexArray.length < 1058) {
    console.warn('数据长度不足，无法解析1034-1057位置');
    return { temperatures: [], humidities: [] };
  }
  
  const temperatures = [];
  const humidities = [];
  
  // 解析1034-1057位置的数据
  // 假设每4个字节为一组数据，前2个字节为温度，后2个字节为湿度
  for (let i = 1034; i < 1058; i += 4) {
    if (i + 3 >= hexArray.length) break;
    
    try {
      // 温度数据解析 (注意: 低位在前，高位在后)
      const tempLowByte = parseInt(hexArray[i], 16);
      const tempHighByte = parseInt(hexArray[i+1], 16);
      
      // 湿度数据解析 (注意: 低位在前，高位在后)
      const humLowByte = parseInt(hexArray[i+2], 16);
      const humHighByte = parseInt(hexArray[i+3], 16);
      
      if (!isNaN(tempHighByte) && !isNaN(tempLowByte) && 
          !isNaN(humHighByte) && !isNaN(humLowByte)) {
        
        // 使用位运算构建温度的16位有符号整数
        const tempRaw = (tempHighByte << 8) | tempLowByte;
        
        // 判断温度符号位并正确计算
        let temperature;
        if ((tempRaw & 0x8000) === 0) {
          // 正温度: 直接乘以0.1 (假设温度精度为0.1℃)
          temperature = tempRaw * 0.1;
        } else {
          // 负温度: 需要进行二进制补码处理
          temperature = -((0xFFFF - tempRaw + 1) * 0.1);
        }
        
        // 湿度通常为无符号值，范围0-100%
        const humidity = ((humHighByte << 8) | humLowByte) * 0.1;
        
        // 保留1位小数
        temperature = parseFloat(temperature.toFixed(1));
        const humidityValue = parseFloat(humidity.toFixed(1));
        
        // 添加到结果数组
        temperatures.push({
          position: i,
          temperature: temperature
        });
        
        humidities.push({
          position: i,
          humidity: Math.min(humidityValue, 100) // 湿度上限为100%
        });
      }
    } catch (e) {
      console.error(`解析1034-1057位置数据错误 (位置 ${i}):`, e);
    }
  }
  
  return { temperatures, humidities };
} 