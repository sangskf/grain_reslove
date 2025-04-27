<template>
  <div class="app-container">
    <!-- Sidebar -->
    <aside class="sidebar">
      <div class="sidebar-header">
        <h1 class="sidebar-title">粮情数据解析</h1>
        <div class="version-number">v1.0.0</div>
      </div>
      
      <nav class="sidebar-nav">
        <div class="nav-item" :class="{ 'active': currentPage === 'data' }" @click="currentPage = 'data'">
          <span>数据解析</span>
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
      
      <!-- Theme toggle button is now hidden -->
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
import { ref } from 'vue';
import DataTransmitter from './views/DataTransmitter.vue';
import LogViewer from './views/LogViewer.vue';
import AboutPage from './views/AboutPage.vue';
import Settings from './views/Settings.vue';

const currentPage = ref('data');
</script>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

body {
  margin: 0;
  padding: 0;
}

.app-container {
  display: flex;
  height: 100vh;
  width: 100%;
}

.sidebar {
  width: 250px;
  background-color: #f0f0f0;
  color: #333;
  display: flex;
  flex-direction: column;
  padding: 20px 0;
  border-right: 1px solid #ddd;
}

.sidebar-header {
  padding: 0 20px;
  margin-bottom: 20px;
}

.sidebar-title {
  font-size: 1.5rem;
  margin: 0 0 5px 0;
  color: #333;
}

.version-number {
  font-size: 0.8rem;
  color: #666;
}

.sidebar-nav {
  flex: 1;
}

.nav-item {
  padding: 12px 20px;
  cursor: pointer;
  transition: background-color 0.2s;
  color: #333;
}

.nav-item:hover {
  background-color: #e0e0e0;
}

.nav-item.active {
  background-color: #4CAF50;
  color: white;
}

/* Theme toggle is now hidden */
.theme-toggle {
  display: none;
}

.main-content {
  flex: 1;
  padding: 20px;
  overflow-y: auto;
  scrollbar-width: thin;  /* Firefox */
}

/* 隐藏滚动条但保留功能 - Chrome, Safari, Edge */
.main-content::-webkit-scrollbar {
  width: 6px;
}

.main-content::-webkit-scrollbar-track {
  background: transparent;
}

.main-content::-webkit-scrollbar-thumb {
  background-color: rgba(0, 0, 0, 0.2);
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
