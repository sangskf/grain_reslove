<template>
  <div>
    <header class="app-header">
      <h1>粮情数据解析</h1>
      <AppUpdater />
    </header>
    <main class="container">
      <DataTransmitter />
    </main>
  </div>
</template>

<script setup>
import DataTransmitter from './views/DataTransmitter.vue';
import AppUpdater from './components/AppUpdater.vue';
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

.app-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  background-color: #f0f0f0;
  border-bottom: 1px solid #ddd;
}

.app-header h1 {
  margin: 0;
  font-size: 1.5rem;
}

.container {
  margin: 0;
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }
  
  .app-header {
    background-color: #1f1f1f;
    border-bottom-color: #444;
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
