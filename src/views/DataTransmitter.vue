<template>
  <div class="container">
    <!-- 移除配置菜单 -->
    <div class="main-content">
      <div class="tabs">
        <button 
          @click="activeTab = 'send'" 
          :class="{ 'active': activeTab === 'send' }"
        >
          远程解析
        </button>
        <button 
          @click="activeTab = 'parse'" 
          :class="{ 'active': activeTab === 'parse' }"
        >
          本地解析
        </button>
      </div>

      <!-- 传感器配置面板，共享于两个选项卡 -->
      <config-panel 
        :initial-layers="configLayers"
        :initial-rows="configRows"
        :initial-columns="configColumns"
        @config-updated="handleSensorConfigUpdate"
      />

      <!-- 发送数据面板 -->
      <div v-if="activeTab === 'send'">
        <div class="connection-panel">
          <div class="connection-row">
            <div class="form-group ip-field">
              <label for="ip">IP地址:</label>
              <input id="ip" v-model="ipAddress" type="text" placeholder="例如: 192.168.1.1" />
            </div>

            <div class="form-group port-field">
              <label for="port">端口号:</label>
              <input id="port" v-model="port" type="number" placeholder="例如: 8080" />
            </div>
            
            <div class="form-group subdevice-field">
              <label for="subDeviceAddr">分机地址:</label>
              <input 
                id="subDeviceAddr" 
                v-model.number="subDeviceAddr" 
                type="number" 
                min="1"
                max="99"
                placeholder="大于0的整数" 
              />
            </div>
          </div>
          
          <!-- 添加时间选择器 -->
          <div class="time-row">
            <div class="form-group datetime-field">
              <label for="commandDateTime">命令时间:</label>
              <div class="datetime-container">
                <input
                  id="commandDateTime"
                  v-model="commandDateTime"
                  type="datetime-local"
                  step="1"
                  @change="updateCommandWithTime"
                />
                <button
                  class="reset-time-btn"
                  @click="resetToCurrentTime"
                  title="重置为当前时间"
                >
                  <span>⟳</span>
                </button>
              </div>
            </div>
          </div>
        </div>

        <div class="data-panel">
          <div class="form-group">
            <label for="hexData">16进制发送数据:</label>
            <div class="textarea-container">
              <textarea
                  id="hexData"
                  v-model="hexData"
                  placeholder="输入16进制数据，例如: AA A0 18 08 23 16 55 36 00 01 A0 01 FF FF FF FF FF FF FF FF FF FF FF FF FF C3 EF EF"
                  rows="3"
              ></textarea>
              <div class="checkbox-container">
                <input type="checkbox" id="convertToUppercase" v-model="convertToUppercase">
                <label for="convertToUppercase">自动转换为大写</label>
              </div>
            </div>
            <small>格式: 使用空格分隔每个16进制值</small>
          </div>

          <div class="button-group">
            <button @click="loadDefaultData" class="secondary">加载默认数据</button>
            <button @click="sendData" :disabled="isConnecting" class="primary">发送数据</button>
          </div>
        </div>

        <div v-if="isConnecting" class="status connecting">连接中...</div>
        <div v-if="error" class="status error">{{ error }}</div>
      </div>

      <!-- 本地解析面板 -->
      <div v-if="activeTab === 'parse'">
        <div class="data-panel">
          <div class="form-group">
            <label for="responseData">16进制响应数据:</label>
            <div class="textarea-container">
              <textarea
                  id="responseData"
                  v-model="localResponseData"
                  placeholder="输入要解析的16进制数据，例如: AA B0 18 08 23 16 55 36 00 01 28 01 DF 00 D2 00 EF 00 73 01..."
                  rows="5"
              ></textarea>
              <div class="checkbox-container">
                <input type="checkbox" id="convertToUppercaseLocal" v-model="convertToUppercase">
                <label for="convertToUppercaseLocal">自动转换为大写</label>
              </div>
            </div>
            <small>格式: 使用空格分隔每个16进制值，必须符合协议格式（AA B0开头）</small>
          </div>

          <div class="button-group">
            <button @click="loadSampleResponse" class="secondary">加载示例数据</button>
            <button @click="parseLocalData" class="primary">解析数据</button>
          </div>
        </div>

        <div v-if="error" class="status error">{{ error }}</div>
      </div>

      <!-- 解析结果显示，两个选项卡共用 -->
      <hex-data-display v-if="response" :hexData="response" />
      <protocol-header v-if="headerInfo" :headerInfo="headerInfo" />
      <temperature-table v-if="parsedData.length > 0" :temperatures="parsedData" />
      <environment-data v-if="headerInfo" :env-data="environmentData" />
    </div>
  </div>
</template>

<script>
import { ref, computed, watch, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// 导入子组件
import HexDataDisplay from '../components/HexDataDisplay.vue';
import ProtocolHeader from '../components/ProtocolHeader.vue';
import TemperatureTable from '../components/TemperatureTable.vue';
import ConfigPanel from '../components/ConfigPanel.vue';
import EnvironmentData from '../components/EnvironmentData.vue';

// 导入工具函数
import { parseHexString, parseProtocolHeader, parseTemperatureData } from '../utils/dataParser';

export default {
  name: 'DataTransmitter',
  components: {
    HexDataDisplay,
    ProtocolHeader,
    TemperatureTable,
    ConfigPanel,
    EnvironmentData,
  },
  setup() {
    // 选项卡切换
    const activeTab = ref('send');
    
    // 是否将十六进制值转换为大写
    const convertToUppercase = ref(true);
    
    // 发送数据相关
    const ipAddress = ref('');
    const port = ref(null);
    const subDeviceAddr = ref(null);
    const hexData = ref('');
    const isConnecting = ref(false);
    const timeout = ref(5000);
    
    // 命令时间相关
    const commandDateTime = ref('');
    
    // 本地解析相关
    const localResponseData = ref('');
    
    // 传感器配置
    const configLayers = ref(8);
    const configRows = ref(8);
    const configColumns = ref(8);
    const sensorConfig = ref({
      layers: 8,
      rows: 8,
      columns: 8,
      totalPoints: 512,
      dataSize: 1068
    });
    
    // 共用状态
    const error = ref('');
    const response = ref('');
    const parsedData = ref([]);
    const headerInfo = ref(null);
    
    // 环境数据对象
    const environmentData = computed(() => {
      if (!headerInfo.value) return {};
      
      return {
        indoorTemp: headerInfo.value.indoorTemp,
        indoorHumidity: headerInfo.value.indoorHumidity,
        outdoorTemp: headerInfo.value.outdoorTemp,
        outdoorHumidity: headerInfo.value.outdoorHumidity
      };
    });

    // 从localStorage加载配置
    const loadConfig = () => {
      try {
        const savedConfig = localStorage.getItem('appConfig');
        if (savedConfig) {
          const config = JSON.parse(savedConfig);
          
          // 加载网络设置
          ipAddress.value = config.defaultIp || '192.168.3.231';
          port.value = config.defaultPort || 2000;
          subDeviceAddr.value = config.defaultSubDeviceAddr || 1;
          timeout.value = config.timeout || 5000;
          
          // 加载传感器配置
          configLayers.value = config.defaultLayers || 8;
          configRows.value = config.defaultRows || 8;
          configColumns.value = config.defaultColumns || 8;
          
          // 更新传感器配置对象
          sensorConfig.value = {
            layers: configLayers.value,
            rows: configRows.value,
            columns: configColumns.value,
            totalPoints: configLayers.value * configRows.value * configColumns.value,
            dataSize: 1068
          };
          
          // 加载默认数据
          hexData.value = config.defaultSendData || 'AA A0 18 08 23 16 55 36 00 01 A0 01 FF FF FF FF FF FF FF FF FF FF FF FF FF C3 EF EF';
          localResponseData.value = config.sampleResponseData || '';
        }
      } catch (error) {
        console.error('加载配置失败:', error);
      }
    };

    // 在组件挂载时加载配置和设置默认时间
    onMounted(() => {
      loadConfig();
      setDefaultDateTime();
      // 如果有分机地址，更新命令
      if (subDeviceAddr.value) {
        try {
          hexData.value = generateCommand();
        } catch (err) {
          console.error('生成初始命令失败:', err);
        }
      }
    });

    // 设置默认时间为当前时间
    const setDefaultDateTime = () => {
      const now = new Date();
      // 格式化为datetime-local输入框需要的格式 (YYYY-MM-DDTHH:MM:SS)
      const year = now.getFullYear();
      const month = String(now.getMonth() + 1).padStart(2, '0');
      const day = String(now.getDate()).padStart(2, '0');
      const hours = String(now.getHours()).padStart(2, '0');
      const minutes = String(now.getMinutes()).padStart(2, '0');
      const seconds = String(now.getSeconds()).padStart(2, '0');
      
      commandDateTime.value = `${year}-${month}-${day}T${hours}:${minutes}:${seconds}`;
    };
    
    // 重置为当前时间
    const resetToCurrentTime = () => {
      setDefaultDateTime();
      updateCommandWithTime();
    };
    
    // 处理配置更新
    const handleConfigUpdate = (newConfig) => {
      // 更新网络配置
      ipAddress.value = newConfig.defaultIp;
      port.value = newConfig.defaultPort;
      subDeviceAddr.value = newConfig.defaultSubDeviceAddr;
      timeout.value = newConfig.timeout;
      
      // 更新传感器配置
      configLayers.value = newConfig.defaultLayers;
      configRows.value = newConfig.defaultRows;
      configColumns.value = newConfig.defaultColumns;
      
      // 更新传感器配置对象
      sensorConfig.value = {
        layers: newConfig.defaultLayers,
        rows: newConfig.defaultRows,
        columns: newConfig.defaultColumns,
        totalPoints: newConfig.defaultLayers * newConfig.defaultRows * newConfig.defaultColumns,
        dataSize: 1068
      };
      
      // 更新默认数据
      if (!hexData.value) {
        hexData.value = newConfig.defaultSendData;
      }
      if (!localResponseData.value) {
        localResponseData.value = newConfig.sampleResponseData;
      }
    };

    // 处理传感器配置更新
    const handleSensorConfigUpdate = (newConfig) => {
      sensorConfig.value = newConfig;
      console.log('传感器配置已更新:', newConfig);
    };
    
    // 将10进制数转换为BCD码
    const decimalToBCD = (decimal) => {
      return ((Math.floor(decimal / 10) << 4) | (decimal % 10)) & 0xFF;
    };
    
    // 根据选择的时间更新命令
    const updateCommandWithTime = () => {
      try {
        hexData.value = generateCommand();
      } catch (err) {
        console.log('更新时间时出现错误:', err.message);
      }
    };

    // 生成发送指令，根据分机地址和时间修改命令
    const generateCommand = () => {
      // 检查分机地址是否合法
      if (!subDeviceAddr.value || subDeviceAddr.value <= 0) {
        throw new Error('分机地址必须是大于0的正整数');
      }
      
      // 解析当前hexData，以便修改
      const hexValues = hexData.value.trim().split(/\s+/);
      
      // 确保有效的命令格式
      if (hexValues.length < 12) {
        throw new Error('命令格式无效，无法更新分机地址和时间');
      }
      
      // 解析选择的时间
      const selectedDate = new Date(commandDateTime.value);
      if (isNaN(selectedDate.getTime())) {
        throw new Error('请选择有效的日期和时间');
      }
      
      // 设置命令中的时间（位置2-7，年月日时分秒）
      // 注意：年份只使用后两位，并转换为BCD码
      const year = selectedDate.getFullYear() % 100; // 获取年份的后两位
      const month = selectedDate.getMonth() + 1;     // 月份是从0开始的
      const day = selectedDate.getDate();
      const hour = selectedDate.getHours();
      const minute = selectedDate.getMinutes();
      const second = selectedDate.getSeconds();
      
      // 将时间转换为BCD码并设置到命令中
      hexValues[2] = decimalToBCD(year).toString(16).padStart(2, '0').toUpperCase();
      hexValues[3] = decimalToBCD(month).toString(16).padStart(2, '0').toUpperCase();
      hexValues[4] = decimalToBCD(day).toString(16).padStart(2, '0').toUpperCase();
      hexValues[5] = decimalToBCD(hour).toString(16).padStart(2, '0').toUpperCase();
      hexValues[6] = decimalToBCD(minute).toString(16).padStart(2, '0').toUpperCase();
      hexValues[7] = decimalToBCD(second).toString(16).padStart(2, '0').toUpperCase();
      
      // 在协议的第9位设置分机地址
      let subDeviceAddrValue = subDeviceAddr.value;
      // 转换为16进制字符串，并确保两位数（前面补0）
      let hexSubDeviceAddr = subDeviceAddrValue.toString(16).padStart(2, '0').toUpperCase();
      hexValues[9] = hexSubDeviceAddr;
      hexValues[11] = hexSubDeviceAddr;

      // 重新组合命令
      return hexValues.join(' ');
    };

    // 加载默认的16进制发送数据
    const loadDefaultData = () => {
      try {
        const savedConfig = localStorage.getItem('appConfig');
        if (savedConfig) {
          const config = JSON.parse(savedConfig);
          hexData.value = config.defaultSendData || 'AA A0 18 08 23 16 55 36 00 01 A0 01 FF FF FF FF FF FF FF FF FF FF FF FF FF C3 EF EF';
        }
      } catch (error) {
        console.error('加载默认数据失败:', error);
        hexData.value = 'AA A0 18 08 23 16 55 36 00 01 A0 01 FF FF FF FF FF FF FF FF FF FF FF FF FF C3 EF EF';
      }
    };
    
    // 监听分机地址变化，自动更新发送数据
    watch(subDeviceAddr, (newValue) => {
      try {
        if (newValue && newValue >= 1 && newValue < 100) {
          hexData.value = generateCommand();
        }
      } catch (err) {
        // 分机地址变化过程中可能出现暂时性错误，不在此处显示
        console.log('地址更新时出现错误:', err.message);
      }
    });
    
    // 加载示例的16进制响应数据
    const loadSampleResponse = () => {
      try {
        const savedConfig = localStorage.getItem('appConfig');
        if (savedConfig) {
          const config = JSON.parse(savedConfig);
          localResponseData.value = config.sampleResponseData || 'AA B0 18 08 23 16 55 36 00 01 28 01 DF 00 D2 00 EF 00 73 01 27 01 D1 00 74 00 70 00 B7 00 2E 01 EE 00 6E 00 67 00 A5 00 2F 01 D4 00 70 00 70 00 B2 00 2B 01 FD 00 66 00 60 00 91 00 37 01 2C 01 E2 00 10 01 15 01 2E 01 35 01 73 00 4E 00 57 00 35 01 09 01 5E 00 4A 00 98 00 32 01 B8 00 53 00 59 00 1A 01 31 01 E8 00 82 00 76 00 B4 00';
        }
      } catch (error) {
        console.error('加载样例响应数据失败:', error);
        localResponseData.value = 'AA B0 18 08 23 16 55 36 00 01 28 01 DF 00 D2 00 EF 00 73 01 27 01 D1 00 74 00 70 00 B7 00 2E 01 EE 00 6E 00 67 00 A5 00 2F 01 D4 00 70 00 70 00 B2 00 2B 01 FD 00 66 00 60 00 91 00 37 01 2C 01 E2 00 10 01 15 01 2E 01 35 01 73 00 4E 00 57 00 35 01 09 01 5E 00 4A 00 98 00 32 01 B8 00 53 00 59 00 1A 01 31 01 E8 00 82 00 76 00 B4 00';
      }
    };

    // 验证16进制数据格式
    const validateHexData = (data) => {
      if (!data.trim()) {
        return { valid: false, message: '请输入16进制数据' };
      }

      const hexValues = data.trim().split(/\s+/);
      const hexRegex = /^[0-9A-Fa-f]{2}$/;

      for (const hex of hexValues) {
        if (!hexRegex.test(hex)) {
          return {
            valid: false,
            message: `无效的16进制值: ${hex}。每个值必须是两位16进制数字(00-FF)`
          };
        }
      }

      return { valid: true };
    };

    // 如果需要，将十六进制数据转换为大写
    const convertHexToUppercaseIfNeeded = (hexData) => {
      if (convertToUppercase.value) {
        return hexData.toUpperCase();
      }
      return hexData;
    };

    // 发送数据并接收响应
    const sendData = async () => {
      if (!ipAddress.value || !port.value) {
        error.value = '请输入IP地址和端口号';
        return;
      }
      
      // 验证分机地址
      if (!subDeviceAddr.value || subDeviceAddr.value <= 0) {
        error.value = '分机地址必须是大于0的正整数';
        return;
      }

      // 尝试生成带有分机地址和时间的命令
      let commandToSend;
      try {
        commandToSend = generateCommand();
      } catch (err) {
        error.value = err.message;
        return;
      }

      // 转换为大写（如果选项启用）
      commandToSend = convertHexToUppercaseIfNeeded(commandToSend);

      // 验证16进制数据
      const validation = validateHexData(commandToSend);
      if (!validation.valid) {
        error.value = validation.message;
        return;
      }

      error.value = '';
      isConnecting.value = true;
      response.value = '';
      parsedData.value = [];
      headerInfo.value = null;

      try {
        // 使用 Promise.race 实现超时控制
        const timeoutPromise = new Promise((_, reject) => {
          setTimeout(() => reject(new Error('连接超时，请检查设备是否在线')), timeout.value);
        });

        // 发送16进制数据并接收响应
        const sendPromise = invoke('send_hex_data', {
          ip: ipAddress.value,
          port: parseInt(port.value),
          data: commandToSend,
          timeout_ms: timeout.value
        });

        const result = await Promise.race([sendPromise, timeoutPromise]);

        if (result) {
          response.value = result;
          processResponse(result);
        }
      } catch (err) {
        if (err.message.includes('timeout') || err.message.includes('超时')) {
          error.value = '连接超时，请检查设备是否在线';
        } else if (err.message.includes('refused')) {
          error.value = '连接被拒绝，请检查IP地址和端口是否正确';
        } else if (err.message.includes('network')) {
          error.value = '网络错误，请检查网络连接';
        } else {
          error.value = `发送数据失败: ${err.message}`;
        }
      } finally {
        isConnecting.value = false;
      }
    };
    
    // 本地解析输入的16进制响应数据
    const parseLocalData = () => {
      // 清空之前的数据
      error.value = '';
      response.value = '';
      parsedData.value = [];
      headerInfo.value = null;
      
      // 转换为大写（如果选项启用）
      const processedData = convertHexToUppercaseIfNeeded(localResponseData.value);
      
      // 验证16进制数据
      const validation = validateHexData(processedData);
      if (!validation.valid) {
        error.value = validation.message;
        return;
      }
      
      // 设置响应数据并进行解析
      response.value = processedData;
      processResponse(processedData);
    };

    // 处理响应数据
    const processResponse = (hexResponse) => {
      const hexArray = hexResponse.split(/\s+/);

      // 检查包头是否符合协议（aa b0）
      if (hexArray[0].toUpperCase() !== 'AA' || hexArray[1].toUpperCase() !== 'B0') {
        error.value = '接收数据格式错误，无效的包头';
        return;
      }

      // 解析头部信息
      headerInfo.value = parseProtocolHeader(hexArray, sensorConfig.value);

      // 解析温度数据
      parsedData.value = parseTemperatureData(hexArray, sensorConfig.value);
    };

    return {
      activeTab,
      ipAddress,
      port,
      subDeviceAddr,
      commandDateTime,
      hexData,
      localResponseData,
      isConnecting,
      error,
      response,
      parsedData,
      headerInfo,
      environmentData,
      convertToUppercase,
      configLayers,
      configRows,
      configColumns,
      loadDefaultData,
      loadSampleResponse,
      sendData,
      parseLocalData,
      updateCommandWithTime,
      resetToCurrentTime,
      handleConfigUpdate,
      handleSensorConfigUpdate
    };
  }
};
</script>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
  display: grid;
  grid-template-columns: 300px 1fr;
  gap: 20px;
}

.main-content {
  grid-column: 2;
}

.tabs {
  display: flex;
  margin-bottom: 20px;
  border-bottom: 1px solid #ddd;
}

.tabs button {
  padding: 10px 20px;
  background-color: #f0f0f0;
  border: none;
  cursor: pointer;
  margin-right: 5px;
  border-radius: 4px 4px 0 0;
  font-weight: 500;
}

.tabs button.active {
  background-color: #4CAF50;
  color: white;
}

.connection-panel, .data-panel {
  margin-bottom: 10px;
  padding: 15px;
  border-radius: 8px;
  background-color: #f8f8f8;
}

.connection-row, .time-row {
  display: flex;
  gap: 20px;
  margin-bottom: 10px;
  align-items: flex-start;
}

.ip-field {
  flex: 1;
}

.port-field {
  flex: 1;
}

.subdevice-field {
  flex: 1;
}

.datetime-field {
  flex: 1;
}

.datetime-container {
  display: flex;
  align-items: center;
}

.reset-time-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-left: 10px;
  height: 36px;
  width: 36px;
  padding: 0;
  font-size: 18px;
  background-color: #4CAF50;
  color: white;
  border-radius: 50%;
  cursor: pointer;
  transition: background-color 0.3s;
}

.reset-time-btn:hover {
  background-color: #4CAF50;
}

.form-group {
  margin-bottom: 15px;
  width: 100%;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

input, textarea, select {
  width: 100%;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-family: monospace;
  box-sizing: border-box;
}

.textarea-container {
  position: relative;
}

.checkbox-container {
  display: flex;
  align-items: center;
  margin-top: 8px;
}

.checkbox-container input[type="checkbox"] {
  width: auto;
  margin-right: 8px;
}

.checkbox-container label {
  display: inline;
  margin-bottom: 0;
  font-weight: normal;
  font-size: 0.9rem;
  color: #555;
}

small {
  display: block;
  margin-top: 5px;
  color: #666;
}

.button-group {
  display: flex;
  gap: 10px;
  margin-top: 15px;
}

button {
  padding: 10px 15px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

button.primary {
  background-color: #4CAF50;
  color: white;
}

button.secondary {
  background-color: #f0f0f0;
  color: #333;
}

button:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.status {
  margin-top: 10px;
  padding: 10px;
  border-radius: 4px;
}

.connecting {
  background-color: #f0f0f0;
}

.error {
  background-color: #ffebee;
  color: #c62828;
}

/* 响应式布局 */
@media (max-width: 1024px) {
  .container {
    grid-template-columns: 1fr;
    gap: 10px;
  }
  
  .main-content {
    grid-column: 1;
  }
}
</style> 