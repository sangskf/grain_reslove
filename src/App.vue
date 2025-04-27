<template>
  <div class="app-container" :class="{ 'dark-mode': isDarkMode }">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="sidebar-header">
        <h1 class="sidebar-title">粮情数据解析</h1>
        <div class="version-number">V{{ appVersion }}</div>
      </div>
      
      <nav class="sidebar-nav">
        <div class="nav-item" :class="{ 'active': currentPage === 'data' }" @click="currentPage = 'data'">
          <span>首页</span>
        </div>
        <div class="nav-item" :class="{ 'active': currentPage === 'settings' }" @click="currentPage = 'settings'">
          <span>配置</span>
        </div>
        <div class="nav-item" :class="{ 'active': currentPage === 'log' }" @click="currentPage = 'log'">
          <span>日志</span>
        </div>
        <div class="nav-item" :class="{ 'active': currentPage === 'about' }" @click="currentPage = 'about'">
          <span>关于</span>
        </div>
      </nav>
      
      <!-- Theme toggle button -->
      <div class="theme-toggle">
        <button @click="toggleDarkMode">
          {{ isDarkMode ? '🌞 亮色模式' : '🌙 暗色模式' }}
        </button>
      </div>
    </aside>
    
    <!-- Main Content -->
    <main class="main-content">
      <DataTransmitter v-if="currentPage === 'data'" />
      <LogViewer v-if="currentPage === 'log'" />
      <AboutPage v-if="currentPage === 'about'" />
      <Settings v-if="currentPage === 'settings'" />
    </main>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import DataTransmitter from './views/DataTransmitter.vue';
import LogViewer from './views/LogViewer.vue';
import AboutPage from './views/AboutPage.vue';
import Settings from './views/Settings.vue';

const currentPage = ref('data');
const isDarkMode = ref(false);
const appVersion = ref('0.0.0');

// 从Tauri获取应用版本
const fetchAppVersion = async () => {
  try {
    const version = await getVersion();
    appVersion.value = version;
  } catch (error) {
    console.error('获取应用版本失败:', error);
    // 如果获取失败，使用默认版本
    appVersion.value = '0.1.0';
  }
};

// 组件挂载时
onMounted(async () => {
  // 获取应用版本
  await fetchAppVersion();
  
  // 从localStorage中获取主题偏好
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'dark') {
    isDarkMode.value = true;
  } else if (savedTheme === null && window.matchMedia('(prefers-color-scheme: dark)').matches) {
    // 如果没有保存的主题且系统偏好暗色模式，则使用暗色模式
    isDarkMode.value = true;
  }
});

// 切换深色模式
const toggleDarkMode = () => {
  isDarkMode.value = !isDarkMode.value;
  localStorage.setItem('theme', isDarkMode.value ? 'dark' : 'light');
};
</script>

<style>
:root {
  --text-color: #0f0f0f;
  --bg-color: #f6f6f6;
  --sidebar-bg: #f0f0f0;
  --sidebar-text: #333;
  --sidebar-border: #ddd;
  --sidebar-hover: #e0e0e0;
  --scrollbar-thumb: rgba(0, 0, 0, 0.2);
  --version-color: #666;
  --active-color: #4CAF50;
  --active-text: white;
  
  /* 日志级别徽章的亮色模式颜色 */
  --info-badge-bg: #e3f2fd;
  --info-badge-text: #0d47a1;
  --warn-badge-bg: #fff8e1;
  --warn-badge-text: #ff8f00;
  --error-badge-bg: #ffebee;
  --error-badge-text: #c62828;
  
  /* 温度显示的亮色模式颜色 */
  --temp-very-high-bg: #ffcccc;
  --temp-very-high-text: #cc0000;
  --temp-high-bg: #fff0cc;
  --temp-high-text: #cc6600;
  --temp-normal-bg: #e8f5e9;
  --temp-normal-text: #2e7d32;
  --temp-low-bg: #e3f2fd;
  --temp-low-text: #0d47a1;
  --temp-very-low-bg: #e0f7fa;
  --temp-very-low-text: #006064;
  
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: var(--text-color);
  background-color: var(--bg-color);

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.dark-mode {
  --text-color: #f6f6f6;
  --bg-color: #1e1e1e;
  --sidebar-bg: #252526;
  --sidebar-text: #e0e0e0;
  --sidebar-border: #3e3e42;
  --sidebar-hover: #37373d;
  --scrollbar-thumb: rgba(255, 255, 255, 0.2);
  --version-color: #bbbbbb;
  --active-color: #388e3c;
  --active-text: white;
  
  /* 日志级别徽章的暗黑模式颜色 */
  --info-badge-bg: #1a3a5e;
  --info-badge-text: #90caf9;
  --warn-badge-bg: #4d3a00;
  --warn-badge-text: #ffd54f;
  --error-badge-bg: #4e1c1c;
  --error-badge-text: #ef9a9a;
  
  /* 温度显示的暗黑模式颜色 */
  --temp-very-high-bg: #4d0000;
  --temp-very-high-text: #ff8080;
  --temp-high-bg: #3d2600;
  --temp-high-text: #ffcc80;
  --temp-normal-bg: #133d19;
  --temp-normal-text: #81c784;
  --temp-low-bg: #0a2d66;
  --temp-low-text: #64b5f6;
  --temp-very-low-bg: #003133;
  --temp-very-low-text: #80deea;
}

body {
  margin: 0;
  padding: 0;
  color: var(--text-color);
  background-color: var(--bg-color);
}

.app-container {
  display: flex;
  height: 100vh;
  width: 100%;
}

.sidebar {
  width: 250px;
  background-color: var(--sidebar-bg);
  color: var(--sidebar-text);
  display: flex;
  flex-direction: column;
  padding: 20px 0;
  border-right: 1px solid var(--sidebar-border);
}

.sidebar-header {
  padding: 0 20px;
  margin-bottom: 20px;
}

.sidebar-title {
  font-size: 1.5rem;
  margin: 0 0 5px 0;
  color: var(--sidebar-text);
}

.version-number {
  font-size: 0.8rem;
  color: var(--version-color);
}

.sidebar-nav {
  flex: 1;
}

.nav-item {
  padding: 12px 20px;
  cursor: pointer;
  transition: background-color 0.2s;
  color: var(--sidebar-text);
}

.nav-item:hover {
  background-color: var(--sidebar-hover);
}

.nav-item.active {
  background-color: var(--active-color);
  color: var(--active-text);
}

.theme-toggle {
  padding: 10px 20px;
  margin-top: auto;
  display: block;
}

.theme-toggle button {
  width: 100%;
  padding: 10px;
  border: none;
  border-radius: 4px;
  background-color: var(--sidebar-hover);
  color: var(--sidebar-text);
  cursor: pointer;
  font-size: 14px;
  transition: background-color 0.3s;
}

.theme-toggle button:hover {
  opacity: 0.9;
}

.main-content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  scrollbar-width: thin;  /* Firefox */
  color: var(--text-color);
  background-color: var(--bg-color);
}

/* 隐藏滚动条但保留功能 - Chrome, Safari, Edge */
.main-content::-webkit-scrollbar {
  width: 6px;
}

.main-content::-webkit-scrollbar-track {
  background: transparent;
}

.main-content::-webkit-scrollbar-thumb {
  background-color: var(--scrollbar-thumb);
  border-radius: 20px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  
  button:active {
    background-color: #0f0f0f69;
  }
}
</style>
