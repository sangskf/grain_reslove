<template>
  <div class="updater-container">
    <div class="version-display">
      <span>当前版本: {{ appVersion }}</span>
    </div>
    <button 
      @click="checkForUpdates" 
      class="check-updates-btn"
      :disabled="isChecking || isDownloading"
    >
      {{ buttonText }}
    </button>
    
    <div v-if="updateProgress > 0 && updateProgress < 100" class="progress-bar-container">
      <div class="progress-bar" :style="{ width: `${updateProgress}%` }"></div>
      <span class="progress-text">{{ Math.floor(updateProgress) }}%</span>
    </div>
    
    <div v-if="updateMessage" class="update-message" :class="messageType">
      {{ updateMessage }}
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted } from 'vue';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import { getVersion } from '@tauri-apps/plugin-app';

export default {
  name: 'AppUpdater',
  setup() {
    const isChecking = ref(false);
    const isDownloading = ref(false);
    const updateAvailable = ref(false);
    const updateProgress = ref(0);
    const updateMessage = ref('');
    const messageType = ref('info');
    const appVersion = ref('');
    
    // 计算按钮文本
    const buttonText = computed(() => {
      if (isChecking.value) return '检查更新中...';
      if (isDownloading.value) return '正在下载更新...';
      if (updateAvailable.value) return '安装更新';
      return '检查更新';
    });
    
    // 获取应用版本
    const fetchAppVersion = async () => {
      try {
        appVersion.value = await getVersion();
      } catch (error) {
        console.error('获取版本号失败:', error);
        appVersion.value = '未知';
      }
    };
    
    // 组件挂载时获取版本号
    onMounted(() => {
      fetchAppVersion();
    });
    
    // 检查更新
    const checkForUpdates = async () => {
      if (isDownloading.value) return;
      
      if (updateAvailable.value) {
        await installUpdate();
        return;
      }
      
      isChecking.value = true;
      updateMessage.value = '正在检查更新...';
      messageType.value = 'info';
      
      try {
        const update = await check();
        
        if (update) {
          updateAvailable.value = true;
          updateMessage.value = `发现新版本 ${update.version}${update.body ? `\n${update.body}` : ''}`;
          messageType.value = 'success';
        } else {
          updateMessage.value = '当前已是最新版本';
          messageType.value = 'info';
        }
      } catch (error) {
        console.error('检查更新失败:', error);
        updateMessage.value = `检查更新失败: ${error.message || '未知错误'}`;
        messageType.value = 'error';
      } finally {
        isChecking.value = false;
      }
    };
    
    // 安装更新
    const installUpdate = async () => {
      if (!updateAvailable.value || isDownloading.value) return;
      
      try {
        const update = await check();
        
        if (!update) {
          updateMessage.value = '无可用更新';
          messageType.value = 'error';
          updateAvailable.value = false;
          return;
        }
        
        isDownloading.value = true;
        updateProgress.value = 0;
        let downloaded = 0;
        let contentLength = 0;
        
        await update.downloadAndInstall((event) => {
          switch (event.event) {
            case 'Started':
              contentLength = event.data.contentLength;
              updateMessage.value = `开始下载更新，总大小: ${formatFileSize(contentLength)}`;
              messageType.value = 'info';
              break;
            case 'Progress':
              downloaded += event.data.chunkLength;
              updateProgress.value = (downloaded / contentLength) * 100;
              break;
            case 'Finished':
              updateMessage.value = '下载完成，准备安装...';
              break;
            case 'Error':
              updateMessage.value = `下载错误: ${event.data.error}`;
              messageType.value = 'error';
              break;
          }
        });
        
        updateMessage.value = '更新已安装，准备重启应用...';
        messageType.value = 'success';
        
        // 等待一会再重启，让用户看到更新成功的消息
        setTimeout(async () => {
          await relaunch();
        }, 2000);
        
      } catch (error) {
        console.error('安装更新失败:', error);
        updateMessage.value = `安装更新失败: ${error.message || '未知错误'}`;
        messageType.value = 'error';
        isDownloading.value = false;
      }
    };
    
    // 格式化文件大小
    const formatFileSize = (bytes) => {
      if (bytes === 0) return '0 B';
      
      const k = 1024;
      const sizes = ['B', 'KB', 'MB', 'GB'];
      const i = Math.floor(Math.log(bytes) / Math.log(k));
      
      return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
    };
    
    return {
      isChecking,
      isDownloading,
      updateProgress,
      updateMessage,
      messageType,
      buttonText,
      appVersion,
      checkForUpdates
    };
  }
};
</script>

<style scoped>
.updater-container {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 12px;
  margin: 10px 0;
}

.version-display {
  font-size: 14px;
  color: #666;
  white-space: nowrap;
}

.check-updates-btn {
  background-color: #4CAF50;
  color: white;
  border: none;
  padding: 8px 16px;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  min-width: 120px;
}

.check-updates-btn:hover {
  background-color: #45a049;
}

.check-updates-btn:disabled {
  background-color: #cccccc;
  cursor: not-allowed;
}

.progress-bar-container {
  width: 100%;
  height: 20px;
  background-color: #f0f0f0;
  border-radius: 10px;
  margin-top: 10px;
  position: relative;
  overflow: hidden;
}

.progress-bar {
  height: 100%;
  background-color: #4CAF50;
  border-radius: 10px;
  transition: width 0.3s ease;
}

.progress-text {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  color: #333;
  font-size: 12px;
  font-weight: bold;
}

.update-message {
  margin-top: 10px;
  padding: 10px;
  border-radius: 4px;
  max-width: 100%;
  text-align: center;
  word-break: break-word;
}

.update-message.info {
  background-color: #e3f2fd;
  color: #0d47a1;
}

.update-message.success {
  background-color: #e8f5e9;
  color: #2e7d32;
}

.update-message.error {
  background-color: #ffebee;
  color: #c62828;
}

@media (prefers-color-scheme: dark) {
  .version-display {
    color: #aaa;
  }
}
</style> 