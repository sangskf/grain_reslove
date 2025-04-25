<template>
  <div class="container">
    <h2>粮情数据传输</h2>

    <div class="tabs">
      <button 
        @click="activeTab = 'send'" 
        :class="{ 'active': activeTab === 'send' }"
      >
        数据发送
      </button>
      <button 
        @click="activeTab = 'parse'" 
        :class="{ 'active': activeTab === 'parse' }"
      >
        本地解析
      </button>
    </div>

    <!-- 发送数据面板 -->
    <div v-if="activeTab === 'send'">
      <div class="connection-panel">
        <div class="form-group">
          <label for="ip">IP地址:</label>
          <input id="ip" v-model="ipAddress" type="text" placeholder="例如: 192.168.1.1" />
        </div>

        <div class="form-group">
          <label for="port">端口号:</label>
          <input id="port" v-model="port" type="number" placeholder="例如: 8080" />
        </div>
      </div>

      <div class="data-panel">
        <div class="form-group">
          <label for="hexData">16进制发送数据:</label>
          <textarea
              id="hexData"
              v-model="hexData"
              placeholder="输入16进制数据，例如: AA A0 18 08 23 16 55 36 00 01 A0 01 FF FF FF FF FF FF FF FF FF FF FF FF FF C3 EF EF"
              rows="3"
          ></textarea>
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
          <textarea
              id="responseData"
              v-model="localResponseData"
              placeholder="输入要解析的16进制数据，例如: aa b0 18 08 23 16 55 36 00 01 28 01 df 00 d2 00 ef 00 73 01..."
              rows="5"
          ></textarea>
          <small>格式: 使用空格分隔每个16进制值，必须符合协议格式（aa b0开头）</small>
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
  </div>
</template>

<script>
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// 导入子组件
import HexDataDisplay from '../components/HexDataDisplay.vue';
import ProtocolHeader from '../components/ProtocolHeader.vue';
import TemperatureTable from '../components/TemperatureTable.vue';

// 导入工具函数
import { parseHexString, parseProtocolHeader, parseTemperatureData } from '../utils/dataParser';

export default {
  name: 'DataTransmitter',
  components: {
    HexDataDisplay,
    ProtocolHeader,
    TemperatureTable
  },
  setup() {
    // 选项卡切换
    const activeTab = ref('send');
    
    // 发送数据相关
    const ipAddress = ref('127.0.0.1');
    const port = ref(8080);
    const hexData = ref('AA A0 18 08 23 16 55 36 00 01 A0 01 FF FF FF FF FF FF FF FF FF FF FF FF FF C3 EF EF');
    const isConnecting = ref(false);
    
    // 本地解析相关
    const localResponseData = ref('');
    
    // 共用状态
    const error = ref('');
    const response = ref('');
    const parsedData = ref([]);
    const headerInfo = ref(null);

    // 加载默认的16进制发送数据
    const loadDefaultData = () => {
      hexData.value = 'AA A0 18 08 23 16 55 36 00 01 A0 01 FF FF FF FF FF FF FF FF FF FF FF FF FF C3 EF EF';
    };
    
    // 加载示例的16进制响应数据
    const loadSampleResponse = () => {
      localResponseData.value = 'aa b0 18 08 23 16 55 36 00 01 28 01 df 00 d2 00 ef 00 73 01 27 01 d1 00 74 00 70 00 b7 00 2e 01 ee 00 6e 00 67 00 a5 00 2f 01 d4 00 70 00 70 00 b2 00 2b 01 fd 00 66 00 60 00 91 00 37 01 2c 01 e2 00 10 01 15 01 2e 01 35 01 73 00 4e 00 57 00 35 01 09 01 5e 00 4a 00 98 00 32 01 b8 00 53 00 59 00 1a 01 31 01 e8 00 82 00 76 00 b4 00 1d 01 23 01 fd 00 fe 00 2a 01 3c 01 3c 01';
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

    // 发送数据并接收响应
    const sendData = async () => {
      if (!ipAddress.value || !port.value) {
        error.value = '请输入IP地址和端口号';
        return;
      }

      // 验证16进制数据
      const validation = validateHexData(hexData.value);
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
        // 发送16进制数据并接收响应
        const result = await invoke('send_hex_data', {
          ip: ipAddress.value,
          port: parseInt(port.value),
          data: hexData.value
        });

        if (result) {
          response.value = result;
          processResponse(result);
        }
      } catch (err) {
        error.value = `发送数据失败: ${err}`;
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
      
      // 验证16进制数据
      const validation = validateHexData(localResponseData.value);
      if (!validation.valid) {
        error.value = validation.message;
        return;
      }
      
      // 设置响应数据并进行解析
      response.value = localResponseData.value;
      processResponse(localResponseData.value);
    };

    // 处理响应数据
    const processResponse = (hexResponse) => {
      const hexArray = hexResponse.split(/\s+/);

      // 检查包头是否符合协议（aa b0）
      if (hexArray[0] !== 'aa' || hexArray[1] !== 'b0') {
        error.value = '接收数据格式错误，无效的包头';
        return;
      }

      // 解析头部信息
      headerInfo.value = parseProtocolHeader(hexArray);

      // 解析温度数据
      parsedData.value = parseTemperatureData(hexArray);
    };

    return {
      activeTab,
      ipAddress,
      port,
      hexData,
      localResponseData,
      isConnecting,
      error,
      response,
      parsedData,
      headerInfo,
      loadDefaultData,
      loadSampleResponse,
      sendData,
      parseLocalData
    };
  }
};
</script>

<style scoped>
.container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
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
  margin-bottom: 20px;
  padding: 15px;
  border-radius: 8px;
  background-color: #f8f8f8;
}

.form-group {
  margin-bottom: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
}

input, textarea {
  width: 100%;
  padding: 8px;
  border: 1px solid #ccc;
  border-radius: 4px;
  font-family: monospace;
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
</style> 