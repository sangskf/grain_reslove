/**
 * 解析16进制字符串为数组
 * @param {string} hexString - 空格分隔的16进制字符串
 * @returns {string[]} - 16进制值数组
 */
export function parseHexString(hexString) {
  return hexString.trim().split(/\s+/);
}

/**
 * 解析协议头部信息
 * @param {string[]} hexArray - 16进制数据数组
 * @returns {Object|null} - 解析后的头部信息对象
 */
export function parseProtocolHeader(hexArray) {
  if (hexArray.length < 13) {
    return null;
  }
  
  return {
    header: hexArray[0] + ' ' + hexArray[1],
    deviceId: hexArray.slice(2, 10).join(' '),
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
  
  // 从索引13开始解析温度数据
  for (let i = 13; i < hexArray.length - 1; i += 2) {
    // 遇到FF FF表示数据结束
    if (hexArray[i] === 'ff' && hexArray[i+1] === 'ff') {
      break;
    }
    
    try {
      // 解析两个字节的温度值
      const highByte = parseInt(hexArray[i], 16);
      const lowByte = parseInt(hexArray[i+1], 16);
      
      if (!isNaN(highByte) && !isNaN(lowByte)) {
        const tempRaw = (highByte * 256 + lowByte);
        const temperature = tempRaw / 10.0;
        
        results.push({
          sensorId: Math.floor((i - 13) / 2) + 1,
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