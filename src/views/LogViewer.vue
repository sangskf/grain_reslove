<template>
  <div class="log-container">
    <h2>系统日志</h2>
    
    <div class="log-controls">
      <div class="log-filter">
        <label for="log-level">日志级别:</label>
        <select id="log-level" v-model="logLevel">
          <option value="all">全部</option>
          <option value="info">信息</option>
          <option value="warning">警告</option>
          <option value="error">错误</option>
        </select>
      </div>
      
      <div class="log-actions">
        <button class="refresh-btn" @click="refreshLogs">刷新</button>
        <button class="clear-btn" @click="clearLogs">清空</button>
      </div>
    </div>
    
    <div class="log-content">
      <div v-if="logs.length === 0" class="empty-log">
        暂无日志记录
      </div>
      
      <div v-else class="log-entries">
        <div v-for="(log, index) in filteredLogs" :key="index" :class="['log-entry', `log-${log.level}`]">
          <div class="log-time">{{ log.time }}</div>
          <div class="log-level-badge">{{ getLevelText(log.level) }}</div>
          <div class="log-message">{{ log.message }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed } from 'vue';

export default {
  name: 'LogViewer',
  setup() {
    const logLevel = ref('all');
    
    // Sample log data - in a real app, this would come from a backend service
    const logs = ref([
      { time: '2023-10-15 14:30:22', level: 'info', message: '应用启动成功' },
      { time: '2023-10-15 14:35:10', level: 'info', message: '用户连接到 192.168.3.231:2000' },
      { time: '2023-10-15 14:36:45', level: 'warning', message: '连接超时，正在重试...' },
      { time: '2023-10-15 14:37:12', level: 'error', message: '连接失败: 无法访问目标地址' },
      { time: '2023-10-15 14:40:05', level: 'info', message: '用户重新连接成功' },
      { time: '2023-10-15 14:42:30', level: 'info', message: '数据解析完成，共处理 512 个测点' },
    ]);
    
    const filteredLogs = computed(() => {
      if (logLevel.value === 'all') {
        return logs.value;
      }
      return logs.value.filter(log => log.level === logLevel.value);
    });
    
    const getLevelText = (level) => {
      switch (level) {
        case 'info': return '信息';
        case 'warning': return '警告';
        case 'error': return '错误';
        default: return level;
      }
    };
    
    const refreshLogs = () => {
      // In a real app, this would fetch new logs from the backend
      console.log('刷新日志');
    };
    
    const clearLogs = () => {
      if (confirm('确定要清空所有日志记录吗？')) {
        logs.value = [];
      }
    };
    
    return {
      logLevel,
      logs,
      filteredLogs,
      getLevelText,
      refreshLogs,
      clearLogs
    };
  }
};
</script>

<style scoped>
.log-container {
  padding: 0 20px;
  font-size: 0.7rem;
  max-width: 1200px;
  margin: 0 auto;
}

h2 {
  margin-top: 0;
  margin-bottom: 20px;
  color: #333;
  font-size: 1.5rem;
}

.log-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  background-color: #f3f3f3;
  padding: 12px 16px;
  border-radius: 6px;
}

.log-filter {
  display: flex;
  align-items: center;
}

.log-filter label {
  margin-right: 10px;
  font-weight: 500;
  color: #555;
  font-size: 0.85rem;
}

.log-filter select {
  padding: 6px 12px;
  border-radius: 4px;
  border: 1px solid #ddd;
  background-color: white;
  font-size: 0.85rem;
  color: #444;
}

.log-actions {
  display: flex;
  gap: 10px;
}

button {
  padding: 7px 14px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 500;
  font-size: 0.85rem;
  transition: all 0.2s;
}

.refresh-btn {
  background-color: #4CAF50;
  color: white;
}

.refresh-btn:hover {
  background-color: #45a049;
}

.clear-btn {
  background-color: #f0f0f0;
  color: #333;
}

.clear-btn:hover {
  background-color: #e0e0e0;
}

.log-content {
  background-color: #fcfcfc;
  border: 1px solid #eaeaea;
  border-radius: 6px;
  height: 550px;
  overflow-y: auto;
  box-shadow: 0 1px 3px rgba(0, 0, 0, 0.05);
}

.empty-log {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100%;
  color: #999;
  font-style: italic;
  font-size: 0.9rem;
}

.log-entries {
  padding: 8px;
}

.log-entry {
  display: flex;
  align-items: center;
  padding: 8px 12px;
  border-bottom: 1px solid #f0f0f0;
  font-size: 0.85rem;
  transition: background-color 0.1s;
}

.log-entry:hover {
  background-color: #f9f9f9;
}

.log-entry:last-child {
  border-bottom: none;
}

.log-time {
  flex: 0 0 160px;
  font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
  color: #666;
  font-size: 0.8rem;
}

.log-level-badge {
  flex: 0 0 40px;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.75rem;
  text-align: center;
  margin-right: 12px;
  font-weight: 500;
}

.log-message {
  flex: 1;
  line-height: 1.4;
  word-break: break-word;
}

.log-info .log-level-badge {
  background-color: #e3f2fd;
  color: #0d47a1;
}

.log-warning .log-level-badge {
  background-color: #fff8e1;
  color: #ff8f00;
}

.log-error .log-level-badge {
  background-color: #ffebee;
  color: #c62828;
}

/* Add subtle alternating row colors */
.log-entry:nth-child(even) {
  background-color: #fafafa;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .log-time {
    flex: 0 0 120px;
    font-size: 0.75rem;
  }
  
  .log-level-badge {
    flex: 0 0 35px;
    font-size: 0.7rem;
  }
  
  .log-container {
    padding: 0 10px;
  }
}
</style> 