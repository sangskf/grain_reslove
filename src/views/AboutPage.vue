<template>
  <div class="about-container">
    <h2>关于应用</h2>
    
    <div class="about-section version-section">
      <div class="version-layout">
        <div class="version-info">
          <div class="app-logo">🌾</div>
          <div class="version-details">
            <h3>粮情数据解析工具</h3>
            <div class="version-meta">
              <div class="version-number">V{{ appVersion }}</div>
              <div class="version-updater">
                <AppUpdater />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <div class="about-section">
      <h3>功能介绍</h3>
      <p>本软件是一款专门用于解析粮情监测数据的工具，旨在简化仓储粮食温度与湿度数据的获取、解析与分析，提高粮情监测效率。</p>
      
      <div class="feature-list">
        <div class="feature-item">
          <div class="feature-icon">📡</div>
          <div class="feature-text">
            <h4>远程数据采集</h4>
            <p>通过TCP/IP网络远程连接到粮情数据采集设备，实时获取传感器数据，支持多种通信协议</p>
          </div>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">📊</div>
          <div class="feature-text">
            <h4>数据解析与展示</h4>
            <p>解析粮情设备返回的16进制数据，以图表和表格形式直观展示温度与湿度信息，方便数据分析</p>
          </div>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">⚙️</div>
          <div class="feature-text">
            <h4>灵活配置</h4>
            <p>支持多种传感器布局，灵活设置层数、行数、列数等参数，适配不同规模的粮库环境</p>
          </div>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">📝</div>
          <div class="feature-text">
            <h4>日志记录</h4>
            <p>自动记录系统日志和通信数据，便于追踪问题和分析历史数据</p>
          </div>
        </div>
        
        <div class="feature-item">
          <div class="feature-icon">🔄</div>
          <div class="feature-text">
            <h4>自动更新</h4>
            <p>支持应用自动检查更新，确保您始终使用最新版本的工具</p>
          </div>
        </div>
      </div>
    </div>
    
    <div class="about-section">
      <h3>使用说明</h3>
      <div class="instructions">
        <div class="instruction-item">
          <div class="instruction-number">1</div>
          <div class="instruction-content">
            <h4>基本设置</h4>
            <p>在"系统设置"页面配置默认IP地址、端口号、行列层数等参数</p>
          </div>
        </div>
        
        <div class="instruction-item">
          <div class="instruction-number">2</div>
          <div class="instruction-content">
            <h4>设备连接</h4>
            <p>在主页面输入设备IP地址和端口，点击"连接设备"按钮建立连接</p>
          </div>
        </div>
        
        <div class="instruction-item">
          <div class="instruction-number">3</div>
          <div class="instruction-content">
            <h4>数据采集</h4>
            <p>连接成功后，点击"获取数据"按钮发送查询命令，系统将自动解析返回数据</p>
          </div>
        </div>
        
        <div class="instruction-item">
          <div class="instruction-number">4</div>
          <div class="instruction-content">
            <h4>数据查看</h4>
            <p>在图表区域查看温度分布，支持切换不同层数和视图模式</p>
          </div>
        </div>
        
        <div class="instruction-item">
          <div class="instruction-number">5</div>
          <div class="instruction-content">
            <h4>数据导出</h4>
            <p>可以将获取的数据导出为Excel等格式，便于后续分析和报表生成</p>
          </div>
        </div>
      </div>
    </div>
    
    <div class="about-section">
      <h3>常见问题</h3>
      <div class="faq-list">
        <div class="faq-item">
          <div class="faq-question">连接设备失败怎么办？</div>
          <div class="faq-answer">
            <p>请检查：</p>
            <ul>
              <li>设备IP地址和端口号是否正确</li>
              <li>设备是否已开机并正常工作</li>
              <li>网络连接是否稳定</li>
              <li>是否有防火墙阻止连接</li>
            </ul>
          </div>
        </div>
        
        <div class="faq-item">
          <div class="faq-question">数据解析不正确怎么处理？</div>
          <div class="faq-answer">
            <p>可能原因：</p>
            <ul>
              <li>设备通信协议不兼容，请更新软件版本</li>
              <li>请确认设置的行列层数与实际设备匹配</li>
              <li>检查设备固件版本是否需要更新</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
    
    <div class="about-section">
      <h3>邮箱联系</h3>
      <p>如有问题或建议，请发送邮件至：</p>
      <div class="contact-info">
        <div class="contact-item">
          <div class="contact-icon">📧</div>
          <div class="contact-text">support@example.com</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue';
import { getVersion } from '@tauri-apps/api/app';
import AppUpdater from '../components/AppUpdater.vue';

export default {
  name: 'AboutPage',
  components: {
    AppUpdater
  },
  
  setup() {
    const appVersion = ref('0.0.0');
    
    // 获取应用版本
    const fetchAppVersion = async () => {
      try {
        const version = await getVersion();
        appVersion.value = version;
      } catch (error) {
        console.error('获取版本信息失败:', error);
        // 如果获取失败，使用默认版本
        appVersion.value = '0.1.0';
      }
    };
    
    onMounted(async () => {
      await fetchAppVersion();
    });
    
    return {
      appVersion
    };
  }
};
</script>

<style scoped>
.about-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 20px;
}

h2 {
  margin-top: 0;
  margin-bottom: 30px;
  color: var(--text-color, #333);
}

.about-section {
  margin-bottom: 40px;
  background-color: var(--sidebar-bg, #f8f8f8);
  border-radius: 8px;
  padding: 20px;
}

h3 {
  font-size: 1.3rem;
  margin-top: 0;
  margin-bottom: 15px;
  padding-bottom: 10px;
  border-bottom: 1px solid var(--sidebar-border, #eee);
  color: var(--text-color, #333);
}

.version-section {
  background-color: var(--active-color, #4CAF50);
  color: var(--active-text, white);
  position: relative;
  padding: 20px;
}

.version-layout {
  display: flex;
}

.version-info {
  display: flex;
  width: 100%;
}

.app-logo {
  font-size: 2.5rem;
  margin-right: 15px;
  display: flex;
  align-items: center;
}

.version-details {
  display: flex;
  flex-direction: column;
  flex-grow: 1;
}

.version-details h3 {
  margin: 0 0 10px 0;
  padding: 0;
  border: none;
  font-size: 1.4rem;
  color: white;
}

.version-meta {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.version-number {
  font-size: 1.1rem;
  font-weight: bold;
  color: #fff;
}

/* 为版本区域中的更新器组件添加自定义样式 */
.version-section :deep(.updater-container) {
  margin: 0;
  flex-direction: column;
  align-items: flex-end;
  gap: 5px;
}

.version-section :deep(.version-display) {
  display: none; /* 隐藏重复的版本号显示 */
}

.version-section :deep(.check-updates-btn) {
  background-color: rgba(255, 255, 255, 0.15);
  color: white;
  border: 1px solid rgba(255, 255, 255, 0.3);
  padding: 4px 10px;
  font-size: 12px;
  min-width: 0;
}

.version-section :deep(.check-updates-btn:hover) {
  background-color: rgba(255, 255, 255, 0.25);
}

.version-section :deep(.update-message) {
  color: white;
  background-color: rgba(0, 0, 0, 0.1);
  max-width: 200px;
  font-size: 12px;
  padding: 6px;
  text-align: right;
}

.version-section :deep(.progress-bar-container) {
  max-width: 150px;
  height: 15px;
  margin-top: 5px;
}

.version-section :deep(.update-message.info),
.version-section :deep(.update-message.success),
.version-section :deep(.update-message.error) {
  background-color: rgba(0, 0, 0, 0.2);
  color: white;
}

.feature-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
  margin-top: 20px;
}

.feature-item {
  display: flex;
  background-color: var(--bg-color, white);
  border-radius: 8px;
  padding: 15px;
}

.feature-icon {
  font-size: 2rem;
  margin-right: 20px;
  display: flex;
  align-items: center;
  flex-shrink: 0;
}

.feature-text h4 {
  margin: 0 0 8px 0;
  font-size: 1.1rem;
  color: var(--text-color, inherit);
}

.feature-text p {
  margin: 0;
  color: var(--text-color, #555);
  opacity: 0.8;
}

.instructions {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.instruction-item {
  display: flex;
  align-items: flex-start;
  background-color: var(--bg-color, white);
  border-radius: 8px;
  padding: 15px;
}

.instruction-number {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 30px;
  height: 30px;
  background-color: var(--active-color, #4CAF50);
  color: var(--active-text, white);
  border-radius: 50%;
  font-weight: bold;
  margin-right: 15px;
  flex-shrink: 0;
}

.instruction-content h4 {
  margin: 0 0 8px 0;
  font-size: 1.1rem;
}

.instruction-content p {
  margin: 0;
  color: var(--text-color, #555);
}

.faq-list {
  display: flex;
  flex-direction: column;
  gap: 15px;
}

.faq-item {
  background-color: var(--bg-color, white);
  border-radius: 8px;
  padding: 15px;
}

.faq-question {
  font-weight: bold;
  margin-bottom: 10px;
  color: var(--text-color, #333);
}

.faq-answer {
  color: var(--text-color, #555);
}

.faq-answer p {
  margin-top: 0;
  margin-bottom: 8px;
}

.faq-answer ul {
  margin-top: 0;
  padding-left: 20px;
}

.contact-info {
  display: flex;
  flex-wrap: wrap;
  gap: 15px;
  margin-top: 15px;
}

.contact-item {
  display: flex;
  align-items: center;
  background-color: var(--bg-color, white);
  padding: 10px 15px;
  border-radius: 8px;
  margin-right: 15px;
}

.contact-icon {
  margin-right: 10px;
  font-size: 1.2rem;
}

.contact-text {
  font-weight: 500;
}
</style> 