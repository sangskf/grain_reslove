<template>
  <div class="settings-container">
    <h2>系统设置</h2>
    
    <div class="settings-section">
      <h3>网络设置</h3>
      <div class="network-settings">
        <div class="settings-row">
          <div class="form-group">
            <label for="defaultIp">默认IP地址:</label>
            <input 
              id="defaultIp" 
              v-model="config.defaultIp" 
              type="text" 
              @change="saveConfig"
            />
          </div>
          
          <div class="form-group">
            <label for="defaultPort">默认端口号:</label>
            <input 
              id="defaultPort" 
              v-model.number="config.defaultPort" 
              type="number" 
              @change="saveConfig"
            />
          </div>
        </div>

        <div class="settings-row">
          <div class="form-group">
            <label for="defaultSubDeviceAddr">默认分机地址:</label>
            <input 
              id="defaultSubDeviceAddr" 
              v-model.number="config.defaultSubDeviceAddr" 
              type="number" 
              min="1"
              max="99"
              @change="saveConfig"
            />
          </div>
          
          <div class="form-group">
            <label for="timeout">连接超时(毫秒):</label>
            <input 
              id="timeout" 
              v-model.number="config.timeout" 
              type="number" 
              min="1000"
              step="1000"
              @change="saveConfig"
            />
          </div>
        </div>
      </div>
    </div>
    
    <div class="settings-section">
      <h3>行列层配置</h3>
      <div class="sensor-settings">
        <div class="settings-row">
          <div class="form-group">
            <label for="defaultLayers">默认层数:</label>
            <input 
              id="defaultLayers" 
              v-model.number="config.defaultLayers" 
              type="number" 
              min="1"
              @change="saveConfig"
            />
          </div>
          
          <div class="form-group">
            <label for="defaultRows">默认行数:</label>
            <input 
              id="defaultRows" 
              v-model.number="config.defaultRows" 
              type="number" 
              min="1"
              @change="saveConfig"
            />
          </div>
          
          <div class="form-group">
            <label for="defaultColumns">默认列数:</label>
            <input 
              id="defaultColumns" 
              v-model.number="config.defaultColumns" 
              type="number" 
              min="1"
              @change="saveConfig"
            />
          </div>
        </div>
      </div>
    </div>
    
    <div class="settings-section">
      <h3>样例数据</h3>
      <div class="form-group">
        <label for="defaultSendData">默认发送数据:</label>
        <textarea 
          id="defaultSendData" 
          v-model="config.defaultSendData" 
          rows="3"
          @change="saveConfig"
        ></textarea>
      </div>
      
      <div class="form-group">
        <label for="sampleResponseData">样例响应数据:</label>
        <textarea 
          id="sampleResponseData" 
          v-model="config.sampleResponseData" 
          rows="6"
          @change="saveConfig"
        ></textarea>
      </div>
    </div>
    
    <div class="button-group">
      <button @click="resetToDefault" class="secondary">恢复默认配置</button>
      <button @click="saveConfig" class="primary">保存配置</button>
    </div>
    
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { createLogger } from '../utils/logger';

// 日志记录器
const logger = createLogger('系统设置');

// 默认配置
const DEFAULT_CONFIG = {
  defaultIp: '127.0.0.1',
  defaultPort: 2000,
  defaultSubDeviceAddr: 1,
  timeout: 10000,
  defaultLayers: 8,
  defaultRows: 8,
  defaultColumns: 8,
  defaultSendData: 'AA A0 18 08 23 16 55 36 00 01 A0 01 FF FF FF FF FF FF FF FF FF FF FF FF FF FF C3 EF EF',
  sampleResponseData: 'AA B0 18 08 23 16 55 36 00 01 2F 01 E9 00 DB 00 FD 00 79 01 2F 01 DF 00 78 00 73 00 B9 00 33 01 FB 00 70 00 69 00 A9 00 33 01 E1 00 72 00 72 00 B6 00 2F 01 09 01 6A 00 63 00 95 00 3A 01 35 01 EC 00 16 01 1A 01 33 01 38 01 76 00 50 00 58 00 37 01 14 01 60 00 4B 00 9C 00 35 01 C4 00 54 00 5B 00 23 01 34 01 F6 00 89 00 7C 00 BB 00 25 01 2A 01 0A 01 08 01 3F 01 3E 01 3E 01 63 00 4B 00 74 00 35 01 33 01 63 00 4D 00 6F 00 2C 01 32 01 67 00 49 00 6D 00 35 01 33 01 DC 00 C9 00 CC 00 2D 01 34 01 A1 00 97 00 B1 00 33 01 27 01 6A 00 50 00 74 00 41 01 1E 01 60 00 50 00 81 00 35 01 D3 00 54 00 46 00 78 00 2C 01 F6 00 98 00 94 00 BC 00 3F 01 35 01 94 00 6B 00 79 00 37 01 36 01 6A 00 48 00 54 00 2A 01 2D 01 72 00 4F 00 60 00 34 01 3B 01 76 00 4C 00 60 00 3E 01 27 01 6E 00 51 00 5A 00 3B 01 44 01 3B 01 42 01 3E 01 40 01 3E 01 75 00 54 00 6C 00 39 01 45 01 74 00 52 00 64 00 38 01 35 01 37 01 3A 01 45 01 3D 01 2E 01 7B 00 5A 00 7C 00 32 01 3E 01 A2 00 74 00 79 00 29 01 33 01 90 00 50 00 57 00 29 01 32 01 80 00 4D 00 59 00 35 01 29 01 84 00 4B 00 59 00 42 01 36 01 73 00 4B 00 61 00 40 01 AA 00 8A 00 AA 00 75 01 42 01 DC 00 58 00 4E 00 74 00 3A 01 42 01 75 00 4E 00 52 00 2E 01 98 00 54 00 55 00 A7 00 31 01 2F 01 75 00 50 00 5A 00 FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF 3B 00 CA FF FF FF FF FF FF FF FF FF FF FF FF FF FF FF 19 00 FA FF FF FF FF FF FF FF FF FF 7A B7 EF EF'
};

export default {
  name: 'Settings',
  
  setup() {
    const config = ref({ ...DEFAULT_CONFIG });
    
    // 从localStorage加载配置
    const loadConfig = () => {
      const savedConfig = localStorage.getItem('appConfig');
      if (savedConfig) {
        try {
          const parsedConfig = JSON.parse(savedConfig);
          config.value = { ...DEFAULT_CONFIG, ...parsedConfig };
          logger.info('成功加载配置');
        } catch (e) {
          console.error('加载配置失败:', e);
          logger.error(`加载配置失败: ${e.message}`);
          config.value = { ...DEFAULT_CONFIG };
        }
      } else {
        logger.info('未找到保存的配置，使用默认配置');
      }
    };
    
    // 保存配置到localStorage
    const saveConfig = () => {
      try {
        localStorage.setItem('appConfig', JSON.stringify(config.value));
        logger.info('配置已保存');
      } catch (e) {
        console.error('保存配置失败:', e);
        logger.error(`保存配置失败: ${e.message}`);
      }
    };
    
    // 重置为默认配置
    const resetToDefault = () => {
      if (confirm('确定要恢复默认配置吗？这将覆盖所有当前设置。')) {
        config.value = { ...DEFAULT_CONFIG };
        saveConfig();
        logger.info('已重置为默认配置');
      }
    };
    
    // 组件挂载时加载配置
    onMounted(() => {
      loadConfig();
    });
    
    return {
      config,
      saveConfig,
      resetToDefault
    };
  }
};
</script>

<style scoped>
.settings-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

h2 {
  margin-top: 0;
  margin-bottom: 30px;
  color: #333;
}

h3 {
  margin-top: 0;
  margin-bottom: 20px;
  color: #666;
}

.settings-section {
  margin-bottom: 30px;
  padding: 20px;
  background-color: #f8f8f8;
  border-radius: 8px;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.1);
}

.settings-row {
  display: flex;
  gap: 20px;
  margin-bottom: 15px;
}

.network-settings .settings-row .form-group {
  flex: 1;
  width: 50%;
}

.sensor-settings .settings-row .form-group {
  flex: 1;
  width: 33.33%;
}

.form-group {
  margin-bottom: 15px;
}

label {
  display: block;
  margin-bottom: 5px;
  font-weight: bold;
  color: #555;
}

input, textarea {
  width: 100%;
  padding: 8px;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-family: monospace;
  box-sizing: border-box;
}

input:focus, textarea:focus {
  border-color: #4CAF50;
  outline: none;
}

textarea {
  resize: vertical;
}

.button-group {
  display: flex;
  gap: 10px;
  justify-content: flex-end;
  margin-bottom: 30px;
}

button {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  transition: background-color 0.3s;
}

button.primary {
  background-color: #4CAF50;
  color: white;
}

button.primary:hover {
  background-color: #45a049;
}

button.secondary {
  background-color: #f0f0f0;
  color: #333;
}

button.secondary:hover {
  background-color: #e0e0e0;
}

.feature-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin-top: 20px;
}

.feature-item {
  display: flex;
  background-color: white;
  border-radius: 8px;
  padding: 20px;
}

.feature-icon {
  font-size: 2rem;
  margin-right: 20px;
  display: flex;
  align-items: center;
}

.feature-text h4 {
  margin: 0 0 10px 0;
  font-size: 1.1rem;
}

.feature-text p {
  margin: 0;
  color: #555;
}

.help-links {
  list-style-type: none;
  padding: 0;
  margin-top: 15px;
}

.help-links li {
  margin-bottom: 10px;
}

.help-links a {
  display: inline-block;
  color: #4CAF50;
  text-decoration: none;
  padding: 8px 0;
  font-weight: 500;
}

.help-links a:hover {
  text-decoration: underline;
}
</style> 