<template>
  <div class="log-container">
    <h2>系统日志</h2>
    
    <div class="log-controls">
      <div class="log-filter">
        <label for="log-level">日志级别:</label>
        <select id="log-level" v-model="logLevel" @change="fetchLogs">
          <option value="all">全部</option>
          <option value="INFO">信息</option>
          <option value="WARN">警告</option>
          <option value="ERROR">错误</option>
        </select>
      </div>
      
      <div class="log-actions">
        <button class="refresh-btn" @click="fetchLogs">刷新</button>
        <button class="clear-btn" @click="clearLogs">清空</button>
        <div class="auto-refresh">
          <label>
            <input type="checkbox" v-model="autoRefresh">
            自动刷新
          </label>
        </div>
      </div>
    </div>
    
    <div class="log-content">
      <div v-if="logs.length === 0" class="empty-log">
        暂无日志记录
      </div>
      
      <div v-else class="log-entries">
        <div v-for="(log, index) in logs" :key="index" :class="['log-entry', `log-${log.level.toLowerCase()}`]">
          <div class="log-time">{{ log.time }}</div>
          <div class="log-level-badge">{{ getLevelText(log.level) }}</div>
          <div class="log-message">{{ log.message }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, watch, onMounted, onUnmounted } from 'vue';
import { getLogs, clearLogs } from '../utils/logger';

export default {
  name: 'LogViewer',
  setup() {
    const logLevel = ref('all');
    const logs = ref([]);
    const loading = ref(false);
    const error = ref(null);
    const autoRefresh = ref(true);
    let refreshInterval = null;
    
    // 获取日志
    const fetchLogs = async () => {
      loading.value = true;
      error.value = null;
      
      try {
        const level = logLevel.value === 'all' ? null : logLevel.value;
        logs.value = await getLogs(level, 200);
      } catch (err) {
        console.error('获取日志失败:', err);
        error.value = err.toString();
      } finally {
        loading.value = false;
      }
    };
    
    // 清空日志
    const clearLogsHandler = async () => {
      if (!confirm('确定要清空所有日志记录吗？')) {
        return;
      }
      
      try {
        await clearLogs();
        logs.value = [];
      } catch (err) {
        console.error('清空日志失败:', err);
        error.value = err.toString();
      }
    };
    
    // 获取日志级别文本
    const getLevelText = (level) => {
      switch (level) {
        case 'INFO': return '信息';
        case 'WARN': return '警告';
        case 'ERROR': return '错误';
        default: return level;
      }
    };
    
    // 设置自动刷新
    const setupAutoRefresh = () => {
      if (refreshInterval) {
        clearInterval(refreshInterval);
      }
      
      if (autoRefresh.value) {
        refreshInterval = setInterval(() => {
          fetchLogs();
        }, 5000); // 每5秒刷新一次
      }
    };
    
    // 监听自动刷新设置变化
    watch(autoRefresh, () => {
      setupAutoRefresh();
    });
    
    // 组件挂载时获取日志
    onMounted(() => {
      fetchLogs();
      setupAutoRefresh();
    });
    
    // 组件卸载时清理定时器
    onUnmounted(() => {
      if (refreshInterval) {
        clearInterval(refreshInterval);
      }
    });
    
    return {
      logLevel,
      logs,
      loading,
      error,
      autoRefresh,
      fetchLogs,
      clearLogs: clearLogsHandler,
      getLevelText
    };
  }
};
</script>

<style scoped>
.log-container {
  padding: 0 20px;
  font-size: 0.8rem;
  max-width: 1200px;
  margin: 0 auto;
}

h2 {
  margin-top: 0;
  margin-bottom: 20px;
  color: var(--text-color, #333);
  font-size: 1.5rem;
}

.log-controls {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  background-color: var(--sidebar-bg, #f3f3f3);
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
  color: var(--text-color, #555);
  font-size: 0.85rem;
}

.log-filter select {
  padding: 6px 12px;
  border-radius: 4px;
  border: 1px solid var(--sidebar-border, #ddd);
  background-color: var(--bg-color, white);
  font-size: 0.85rem;
  color: var(--text-color, #444);
}

.log-actions {
  display: flex;
  gap: 10px;
  align-items: center;
}

.auto-refresh {
  margin-left: 10px;
  font-size: 0.85rem;
  color: var(--text-color, #555);
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
  background-color: var(--active-color, #4CAF50);
  color: var(--active-text, white);
}

.refresh-btn:hover {
  background-color: var(--active-color, #45a049);
  opacity: 0.9;
}

.clear-btn {
  background-color: var(--sidebar-hover, #f0f0f0);
  color: var(--text-color, #333);
}

.clear-btn:hover {
  background-color: var(--sidebar-hover, #e0e0e0);
  opacity: 0.9;
}

.log-content {
  background-color: var(--bg-color, #fcfcfc);
  border: 1px solid var(--sidebar-border, #eaeaea);
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
  color: var(--version-color, #999);
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
  border-bottom: 1px solid var(--sidebar-border, #f0f0f0);
  font-size: 0.8rem;
  transition: background-color 0.1s;
}

.log-entry:hover {
  background-color: var(--sidebar-hover, #f9f9f9);
}

.log-entry:last-child {
  border-bottom: none;
}

.log-time {
  flex: 0 0 160px;
  font-family: 'Menlo', 'Monaco', 'Courier New', monospace;
  color: var(--version-color, #666);
  font-size: 0.7rem;
}

.log-level-badge {
  flex: 0 0 40px;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 0.65rem;
  text-align: center;
  margin-right: 12px;
  font-weight: 500;
}

.log-message {
  flex: 1;
  line-height: 1.4;
  word-break: break-word;
  color: var(--text-color, inherit);
}

/* 针对不同级别日志的暗黑模式适配 */
.log-info .log-level-badge {
  background-color: var(--info-badge-bg, #e3f2fd);
  color: var(--info-badge-text, #0d47a1);
}

.log-warn .log-level-badge {
  background-color: var(--warn-badge-bg, #fff8e1);
  color: var(--warn-badge-text, #ff8f00);
}

.log-error .log-level-badge {
  background-color: var(--error-badge-bg, #ffebee);
  color: var(--error-badge-text, #c62828);
}

/* Add subtle alternating row colors */
.log-entry:nth-child(even) {
  background-color: var(--sidebar-bg, #fafafa);
  opacity: 0.95;
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