<template>
  <div class="temperature-table">
    <h3>温度数据解析结果:</h3>
    
    <div class="temperature-grid">
      <div v-for="(chunk, chunkIndex) in temperatureChunks" :key="'chunk-'+chunkIndex" class="grid-row">
        <div v-for="(item, itemIndex) in chunk" :key="'item-'+itemIndex" class="grid-cell">
          <div class="sensor-id">传感器 #{{ item.sensorId }}</div>
          <div class="temperature-value" :class="getTemperatureClass(item.temperature)">
            {{ item.temperature.toFixed(1) }} ℃
          </div>
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
export default {
  name: 'TemperatureTable',
  props: {
    temperatures: {
      type: Array,
      required: true
    }
  },
  computed: {
    // 将温度数据按每行5个分组
    temperatureChunks() {
      const chunks = [];
      const itemsPerRow = 4;
      
      for (let i = 0; i < this.temperatures.length; i += itemsPerRow) {
        chunks.push(this.temperatures.slice(i, i + itemsPerRow));
      }
      
      return chunks;
    },
    
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
    // 根据温度值获取样式类
    getTemperatureClass(temp) {
      if (temp > 35) return 'temp-very-high';
      if (temp > 25) return 'temp-high';
      if (temp < 0) return 'temp-very-low';
      if (temp < 10) return 'temp-low';
      return 'temp-normal';
    }
  }
};
</script>

<style scoped>
.temperature-table {
  margin-top: 20px;
  padding: 15px;
  background-color: #f5f5f5;
  border-radius: 4px;
}

h3 {
  margin-top: 0;
  margin-bottom: 10px;
}

.temperature-grid {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.grid-row {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.grid-cell {
  flex: 1;
  min-width: 120px;
  background-color: white;
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
  color: #666;
  margin-bottom: 5px;
}

.temperature-value {
  font-size: 1.2em;
  font-weight: bold;
  text-align: center;
  padding: 5px;
  border-radius: 3px;
}

.temp-very-high {
  background-color: #ffcccc;
  color: #cc0000;
}

.temp-high {
  background-color: #fff0cc;
  color: #cc6600;
}

.temp-normal {
  background-color: #e8f5e9;
  color: #2e7d32;
}

.temp-low {
  background-color: #e3f2fd;
  color: #0d47a1;
}

.temp-very-low {
  background-color: #e0f7fa;
  color: #006064;
}

.temperature-stats {
  margin-top: 15px;
  padding: 10px;
  background-color: #ffffff;
  border-radius: 4px;
  text-align: center;
  font-weight: bold;
}

.temp-high {
  color: #cc6600;
}

.temp-low {
  color: #0d47a1;
}
</style> 