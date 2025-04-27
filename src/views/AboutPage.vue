<template>
  <div class="about-container">
    <h2>关于本应用</h2>
    
    <div class="about-section">
      <h3>功能介绍</h3>
      <p>本软件是一款专门用于解析粮情监测数据的工具，旨在简化仓储粮食温度与湿度数据的获取与分析。</p>
      
      <div class="feature-list">
        <div class="feature-item">
          <div class="feature-icon">📡</div>
          <div class="feature-text">
            <h4>远程数据采集</h4>
            <p>通过网络远程连接到粮情数据采集设备，自动获取传感器数据</p>
          </div>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">📊</div>
          <div class="feature-text">
            <h4>数据解析与展示</h4>
            <p>解析粮情设备返回的16进制数据，以图表形式直观展示温度与湿度信息</p>
          </div>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">⚙️</div>
          <div class="feature-text">
            <h4>灵活配置</h4>
            <p>支持多种传感器布局，灵活设置层数、行数、列数等参数</p>
          </div>
        </div>
      </div>
    </div>
    
    <div class="about-section">
      <h3>使用帮助</h3>
      <p>如果您在使用过程中遇到任何问题，可以通过以下方式获取帮助：</p>
      <ul class="help-links">
        <li><a href="#">发送反馈邮件</a></li>
      </ul>
    </div>
  </div>
</template>

<script setup>
// This is using Vue 3 Composition API with <script setup>
// Components using <script setup> are automatically in Composition API mode
// No need to export anything - the entire component is compiled and exported by default
</script>

<style scoped>
.about-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 0 20px;
}

h2 {
  margin-top: 0;
  margin-bottom: 30px;
  color: var(--text-color, #333);
}

.about-section {
  margin-bottom: 40px;
}

h3 {
  font-size: 1.3rem;
  margin-bottom: 15px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--sidebar-border, #eee);
  color: var(--text-color, #333);
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 20px;
}

.info-item {
  display: flex;
  flex-direction: column;
  background-color: var(--sidebar-bg, #f8f8f8);
  border-radius: 8px;
  padding: 15px;
}

.info-label {
  font-size: 0.9rem;
  color: var(--version-color, #666);
  margin-bottom: 5px;
}

.info-value {
  font-size: 1.1rem;
  font-weight: 500;
  color: var(--text-color, inherit);
}

.feature-list {
  display: flex;
  flex-direction: column;
  gap: 20px;
  margin-top: 20px;
}

.feature-item {
  display: flex;
  background-color: var(--sidebar-bg, #f8f8f8);
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
  color: var(--text-color, inherit);
}

.feature-text p {
  margin: 0;
  color: var(--text-color, #555);
  opacity: 0.8;
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
  color: var(--active-color, #4CAF50);
  text-decoration: none;
  padding: 8px 0;
  font-weight: 500;
}

.help-links a:hover {
  text-decoration: underline;
}
</style> 