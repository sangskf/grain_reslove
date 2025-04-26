<template>
  <div class="environment-data" v-if="hasData">
    <h3>环境数据:</h3>
    <div class="env-grid">
      <div class="env-card indoor">
        <div class="env-title">仓内</div>
        <div class="env-values">
          <div class="temp-value" v-if="envData.indoorTemp !== null">
            <span class="label">温度:</span>
            <span class="value" :class="getTemperatureClass(envData.indoorTemp)">
              {{ envData.indoorTemp }}℃
            </span>
          </div>
          <div class="humidity-value" v-if="envData.indoorHumidity !== null">
            <span class="label">湿度:</span>
            <span class="value">{{ envData.indoorHumidity }}%</span>
          </div>
          <div class="no-data" v-if="!hasIndoorData">暂无内温内湿数据</div>
        </div>
      </div>

      <div class="env-card outdoor">
        <div class="env-title">仓外</div>
        <div class="env-values">
          <div class="temp-value" v-if="envData.outdoorTemp !== null">
            <span class="label">温度:</span>
            <span class="value" :class="getTemperatureClass(envData.outdoorTemp)">
              {{ envData.outdoorTemp }}℃
            </span>
          </div>
          <div class="humidity-value" v-if="envData.outdoorHumidity !== null">
            <span class="label">湿度:</span>
            <span class="value">{{ envData.outdoorHumidity }}%</span>
          </div>
          <div class="no-data" v-if="!hasOutdoorData">暂无外温外湿数据</div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: 'EnvironmentData',
  props: {
    envData: {
      type: Object,
      required: true,
      default: () => ({
        indoorTemp: null,
        indoorHumidity: null,
        outdoorTemp: null,
        outdoorHumidity: null
      })
    }
  },
  computed: {
    hasIndoorData() {
      return this.envData.indoorTemp !== null || this.envData.indoorHumidity !== null;
    },
    hasOutdoorData() {
      return this.envData.outdoorTemp !== null || this.envData.outdoorHumidity !== null;
    },
    hasData() {
      return this.hasIndoorData || this.hasOutdoorData;
    }
  },
  methods: {
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
.environment-data {
  margin-top: 20px;
  padding: 15px;
  background-color: #f5f5f5;
  border-radius: 4px;
}

h3 {
  margin-top: 0;
  margin-bottom: 15px;
}

.env-grid {
  display: flex;
  gap: 20px;
}

.env-card {
  flex: 1;
  background-color: white;
  border-radius: 4px;
  padding: 15px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.env-title {
  font-weight: bold;
  font-size: 1.1em;
  margin-bottom: 10px;
  padding-bottom: 5px;
  border-bottom: 1px solid #eee;
}

.indoor .env-title {
  color: #2196F3;
}

.outdoor .env-title {
  color: #FF9800;
}

.env-values {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.temp-value, .humidity-value {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.label {
  font-weight: 500;
}

.value {
  font-weight: bold;
  padding: 3px 8px;
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

.no-data {
  color: #999;
  font-style: italic;
  text-align: center;
  padding: 10px;
}

@media (max-width: 600px) {
  .env-grid {
    flex-direction: column;
  }
}
</style> 