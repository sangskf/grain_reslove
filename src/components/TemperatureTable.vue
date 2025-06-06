<template>
  <div class="temperature-table">
    <h3>温度数据解析结果:</h3>
    
    <div class="temperature-grid">
      <div v-for="(item, index) in temperatures" :key="index" class="grid-cell">
        <div class="sensor-id">传感器 #{{ item.sensorId }}</div>
        <div class="temperature-value" :style="getTemperatureStyle(item.temperature)">
          {{ item.temperature.toFixed(1) }} ℃
        </div>
      </div>
    </div>
    
    <div class="temperature-stats" v-if="temperatures.length > 0">
      <p>共 {{ temperatures.length }} 个温度点 | 
         最高: <span class="temp-high">{{ maxTemp.toFixed(1) }}℃</span> | 
         最低: <span class="temp-low">{{ minTemp.toFixed(1) }}℃</span> | 
         平均: {{ avgTemp.toFixed(1) }}℃
      </p>
    </div>
  </div>
</template>

<script>
import { DEFAULT_CONFIG } from '../utils/dataSample';
import { useConfigStore } from '../stores/config';

export default {
  name: 'TemperatureTable',
  setup() {
    const configStore = useConfigStore();
    return { configStore };
  },
  props: {
    temperatures: {
      type: Array,
      required: true
    },
    config: {
      type: Object,
      default: () => DEFAULT_CONFIG
    }
  },
  computed: {
    // 计算最高温度
    maxTemp() {
      if (this.temperatures.length === 0) return 0;
      return Math.max(...this.temperatures.map(item => item.temperature));
    },
    
    // 计算最低温度
    minTemp() {
      if (this.temperatures.length === 0) return 0;
      return Math.min(...this.temperatures.map(item => item.temperature));
    },
    
    // 计算平均温度
    avgTemp() {
      if (this.temperatures.length === 0) return 0;
      const sum = this.temperatures.reduce((acc, item) => acc + item.temperature, 0);
      return sum / this.temperatures.length;
    }
  },
  methods: {
    // 根据温度值获取样式
    getTemperatureStyle(temp) {
      if (temp === null || temp === undefined || temp === -100) {
        const invalid = this.config.temperatureConfig.invalid;
        return {
          backgroundColor: invalid.color,
          color: invalid.textColor
        };
      }

      for (const range of this.config.temperatureConfig.ranges) {
        if (temp >= range.min && temp < range.max) {
          return {
            backgroundColor: range.color,
            color: range.textColor
          };
        }
      }

      return {
        backgroundColor: this.config.temperatureConfig.invalid.color,
        color: this.config.temperatureConfig.invalid.textColor
      };
    }
  }
};
</script>

<style scoped>
.temperature-table {
  margin-top: 20px;
  padding: 15px;
  background-color: var(--sidebar-bg, #f5f5f5);
  border-radius: 4px;
}

h3 {
  margin-top: 0;
  margin-bottom: 10px;
  color: var(--text-color, inherit);
}

.temperature-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(120px, 1fr));
  gap: 10px;
}

.grid-cell {
  min-width: 120px;
  background-color: var(--bg-color, white);
  border-radius: 4px;
  padding: 10px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  transition: transform 0.2s;
}

.grid-cell:hover {
  transform: translateY(-2px);
  box-shadow: 0 3px 6px rgba(0,0,0,0.15);
}

.sensor-id {
  font-size: 0.9em;
  color: var(--version-color, #666);
  margin-bottom: 5px;
}

.temperature-value {
  font-size: 1.2em;
  font-weight: bold;
  text-align: center;
  padding: 5px;
  border-radius: 3px;
}

/* 温度级别颜色 */
.temperature-value {
  padding: 8px;
  border-radius: 4px;
  transition: all 0.3s ease;
}

.temperature-stats {
  margin-top: 15px;
  padding: 10px;
  background-color: var(--bg-color, #ffffff);
  border-radius: 4px;
  text-align: center;
  font-weight: bold;
  color: var(--text-color, inherit);
}

.temp-high {
  color: var(--temp-high-text, #8B4513);
}

.temp-low {
  color: var(--temp-low-text, #008000);
}
</style>