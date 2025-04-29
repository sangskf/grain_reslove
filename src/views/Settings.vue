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
      <button @click="resetToDefault" class="secondary reset-btn">
        <i class="fas fa-undo"></i> 恢复默认配置
      </button>
      <button @click="saveConfig" class="primary">保存配置</button>
    </div>
    
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { createLogger } from '../utils/logger';
import { confirm, message } from '@tauri-apps/plugin-dialog';
import { DEFAULT_CONFIG } from '../utils/dataSample';

// 日志记录器
const logger = createLogger('系统设置');

export default {
  name: 'Settings',
  
  setup() {
    const config = ref({ ...DEFAULT_CONFIG });
    
    // 从localStorage加载配置
    const loadConfig = () => {
      let savedConfig = null;
      try {
        savedConfig = localStorage.getItem('appConfig');
        if (savedConfig) {
          const parsedConfig = JSON.parse(savedConfig);
          // 验证解析后的配置
          if (typeof parsedConfig === 'object' && parsedConfig !== null) {
            // 确保所有必需的字段都存在，使用默认值填充缺失字段
            config.value = {
              ...DEFAULT_CONFIG,
              ...parsedConfig,
              // 确保数值类型字段为数字
              defaultPort: Number(parsedConfig.defaultPort) || DEFAULT_CONFIG.defaultPort,
              defaultSubDeviceAddr: Number(parsedConfig.defaultSubDeviceAddr) || DEFAULT_CONFIG.defaultSubDeviceAddr,
              timeout: Number(parsedConfig.timeout) || DEFAULT_CONFIG.timeout,
              defaultLayers: Number(parsedConfig.defaultLayers) || DEFAULT_CONFIG.defaultLayers,
              defaultRows: Number(parsedConfig.defaultRows) || DEFAULT_CONFIG.defaultRows,
              defaultColumns: Number(parsedConfig.defaultColumns) || DEFAULT_CONFIG.defaultColumns,
            };
          } else {
            throw new Error('Invalid config format');
          }
          logger.info('成功加载配置');
        } else {
          config.value = { ...DEFAULT_CONFIG };
          logger.info('未找到保存的配置，使用默认配置');
        }
      } catch (e) {
        console.error('加载配置失败:', e);
        logger.error(`加载配置失败: ${e.message}`);
        config.value = { ...DEFAULT_CONFIG };
        // 清除可能损坏的配置
        try {
          localStorage.removeItem('appConfig');
        } catch (clearError) {
          console.error('清除损坏的配置失败:', clearError);
        }
      }
    };
    
    // 保存配置到localStorage
    const saveConfig = () => {
      try {
        // 验证配置值
        const configToSave = {
          ...config.value,
          defaultPort: parseInt(config.value.defaultPort) || DEFAULT_CONFIG.defaultPort,
          defaultSubDeviceAddr: parseInt(config.value.defaultSubDeviceAddr) || DEFAULT_CONFIG.defaultSubDeviceAddr,
          timeout: parseInt(config.value.timeout) || DEFAULT_CONFIG.timeout,
          defaultLayers: parseInt(config.value.defaultLayers) || DEFAULT_CONFIG.defaultLayers,
          defaultRows: parseInt(config.value.defaultRows) || DEFAULT_CONFIG.defaultRows,
          defaultColumns: parseInt(config.value.defaultColumns) || DEFAULT_CONFIG.defaultColumns,
        };
        
        // 验证IP地址格式
        if (!/^(\d{1,3}\.){3}\d{1,3}$/.test(configToSave.defaultIp)) {
          configToSave.defaultIp = DEFAULT_CONFIG.defaultIp;
        }
        
        localStorage.setItem('appConfig', JSON.stringify(configToSave));
        logger.info('配置已保存');
      } catch (e) {
        console.error('保存配置失败:', e);
        logger.error(`保存配置失败: ${e.message}`);
      }
    };
    
    // 重置为默认配置
    const resetToDefault = async () => {
      const confirmed = await confirm('确定要恢复默认配置吗？这将覆盖所有当前设置。', {
        title: '确认操作',
        type: 'warning'
      });
      
      if (confirmed) {
        // 创建一个新的对象来替换整个配置
        Object.keys(config.value).forEach(key => {
          config.value[key] = DEFAULT_CONFIG[key];
        });
        
        // 强制DOM更新
        setTimeout(() => {
          // 确保输入框的值被更新
          document.querySelectorAll('input, textarea').forEach(input => {
            const event = new Event('change', { bubbles: true });
            input.dispatchEvent(event);
          });
        }, 0);
        
        saveConfig();
        logger.info('已重置为默认配置');
        await message('已恢复为默认配置', { title: '操作成功', type: 'info' });
      }
    };
    
    // 组件挂载时加载配置
    onMounted(() => {
      loadConfig();
    });
    
    return {
      config,
      saveConfig,
      resetToDefault,
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
  color: var(--text-color, #333);
}

h3 {
  margin-top: 0;
  margin-bottom: 20px;
  color: var(--text-color, #666);
  opacity: 0.8;
}

.settings-section {
  margin-bottom: 30px;
  padding: 20px;
  background-color: var(--sidebar-bg, #f8f8f8);
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
  color: var(--text-color, #555);
}

input, textarea {
  width: 100%;
  padding: 8px;
  border: 1px solid var(--sidebar-border, #ddd);
  border-radius: 4px;
  font-family: monospace;
  box-sizing: border-box;
  background-color: var(--bg-color, #fff);
  color: var(--text-color, #333);
}

input:focus, textarea:focus {
  border-color: var(--active-color, #4CAF50);
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
  font-size: 14px;
  transition: background-color 0.3s;
}

button.primary {
  background-color: var(--active-color, #4CAF50);
  color: var(--active-text, white);
}

button.primary:hover {
  background-color: var(--active-color, #45a049);
  opacity: 0.9;
}

button.secondary {
  background-color: var(--sidebar-hover, #f0f0f0);
  color: var(--text-color, #333);
}

button.secondary:hover {
  background-color: var(--sidebar-hover, #e0e0e0);
  opacity: 0.9;
}

button.reset-btn {
  display: flex;
  align-items: center;
  gap: 6px;
}

button.reset-btn i {
  font-size: 0.9em;
}

.feature-text h4 {
  margin: 0 0 10px 0;
  font-size: 1.1rem;
}

.feature-text p {
  margin: 0;
  color: var(--text-color, #555);
}

.help-links li {
  margin-bottom: 10px;
}

.help-links a {
  display: inline-block;
  color: var(--active-color, #4CAF50);
  text-decoration: none;
  padding: 8px 0;
  font-weight: 500;
}

.help-links a:hover {
  text-decoration: underline;
}
</style> 