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
  if (hexArray.length < 1068) {
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
  const deviceId = parseInt(hexArray[9], 16);  // 将第9位的16进制转换为10进制
  
  // 从协议提供的数据长度中获取，但可能需要根据配置进行调整
  const rawDataLength = hexArray.length;

  // 数据长度校正，根据配置和数据量决定
  let adjustedDataLength = rawDataLength;

  // 如果提供了配置，则基于传感器点数调整数据长度
  if (config && config.totalPoints) {
    const totalPoints = config.totalPoints;
    // 如果传感器点数小于512，则使用1068字节
    // 否则，检查实际数据长度
    if (totalPoints <= 512) {
      // 如果协议头声明的数据长度明显不足，但实际数据长度足够
      adjustedDataLength = 1068;
    } else {
      // 大量传感器点的情况，检查是否有足够的数据
      adjustedDataLength = 2136;
    }
  }

  // 解析内温、内湿、外温、外湿数据
  let indoorTemp = null;
  let indoorHumidity = null;
  let outdoorTemp = null;
  let outdoorHumidity = null;

  // 确保数据长度足够
  if (hexArray.length == 1068) {
    // 解析内湿 (1034位置) - 单字节数据
    const indoorHumidityIndex = 1034;
    if (hexArray[indoorHumidityIndex] !== 'FF') {
      indoorHumidity = parseInt(hexArray[indoorHumidityIndex], 16);
    }

    // 解析内温 (1035-1036位置) - 双字节数据，注意高低位
    const indoorTempLowIndex = 1036;
    const indoorTempHighIndex = 1035;
    if (hexArray[indoorTempLowIndex] !== 'FF' || hexArray[indoorTempHighIndex] !== 'FF') {
      const lowByte = parseInt(hexArray[indoorTempLowIndex], 16);
      const highByte = parseInt(hexArray[indoorTempHighIndex], 16);
      // 构建16位有符号整数并计算温度值
      const tempRaw = (highByte << 8) | lowByte;
      // 判断是否为有效温度值
      if (!isNaN(tempRaw)) {
        if ((tempRaw & 0x8000) === 0) {
          // 正温度
          indoorTemp = parseFloat((tempRaw * 0.1).toFixed(1));
        } else {
          // 负温度
          indoorTemp = parseFloat((-((0xFFFF - tempRaw + 1) * 0.1)).toFixed(1));
        }
      }
    }

    // 解析外湿 (1052位置) - 单字节数据
    const outdoorHumidityIndex = 1052;
    if (hexArray[outdoorHumidityIndex] !== 'FF') {
      outdoorHumidity = parseInt(hexArray[outdoorHumidityIndex], 16);
    }

    // 解析外温 (1053-1054位置) - 双字节数据，注意高低位
    const outdoorTempLowIndex = 1054;
    const outdoorTempHighIndex = 1053;
    if (hexArray[outdoorTempLowIndex] !== 'FF' || hexArray[outdoorTempHighIndex] !== 'FF') {
      const lowByte = parseInt(hexArray[outdoorTempLowIndex], 16);
      const highByte = parseInt(hexArray[outdoorTempHighIndex], 16);
      // 构建16位有符号整数并计算温度值
      const tempRaw = (highByte << 8) | lowByte;
      // 判断是否为有效温度值
      if (!isNaN(tempRaw)) {
        if ((tempRaw & 0x8000) === 0) {
          // 正温度
          outdoorTemp = parseFloat((tempRaw * 0.1).toFixed(1));
        } else {
          // 负温度
          outdoorTemp = parseFloat((-((0xFFFF - tempRaw + 1) * 0.1)).toFixed(1));
        }
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
    // 添加环境数据
    indoorTemp: indoorTemp,
    indoorHumidity: indoorHumidity,
    outdoorTemp: outdoorTemp,
    outdoorHumidity: outdoorHumidity
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
    if (hexArray[i] === 'FF' && hexArray[i+1] === 'FF') {
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