<template>
  <div class="environment-data" v-if="hasData">
    <h3>环境数据:</h3>
    <div class="env-grid">
      <div class="env-card indoor">
        <div class="env-title">仓内</div>
        <div class="env-values">
          <div class="temp-value" v-if="envData.indoorTemp !== null">
            <span class="label">温度:</span>
            <span class="value" :style="getTemperatureStyle(envData.indoorTemp)">
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
            <span class="value" :style="getTemperatureStyle(envData.outdoorTemp)">
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
import { DEFAULT_CONFIG } from '../utils/dataSample';
import { useConfigStore } from '../stores/config';

export default {
  name: 'EnvironmentData',
  setup() {
    const configStore = useConfigStore();
    return { configStore };
  },
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
    },
    config: {
      type: Object,
      default: () => DEFAULT_CONFIG
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
.environment-data {
  margin-top: 20px;
  padding: 15px;
  background-color: var(--sidebar-bg, #f5f5f5);
  border-radius: 4px;
}

h3 {
  margin-top: 0;
  margin-bottom: 15px;
  color: var(--text-color, inherit);
}

.env-grid {
  display: flex;
  gap: 20px;
}

.env-card {
  flex: 1;
  background-color: var(--bg-color, white);
  border-radius: 4px;
  padding: 15px;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.env-title {
  font-weight: bold;
  font-size: 1.1em;
  margin-bottom: 10px;
  padding-bottom: 5px;
  border-bottom: 1px solid var(--sidebar-border, #eee);
}

.indoor .env-title {
  color: var(--info-badge-text, #2196F3);
}

.outdoor .env-title {
  color: var(--warn-badge-text, #FF9800);
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
  color: var(--text-color, inherit);
}

.value {
  font-weight: bold;
  padding: 3px 8px;
  border-radius: 3px;
}

.value {
  font-weight: bold;
  padding: 3px 8px;
  border-radius: 3px;
  transition: all 0.3s ease;
}

.no-data {
  color: var(--version-color, #999);
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