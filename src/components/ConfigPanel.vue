<template>
  <div class="config-panel">
    <h3>行列层配置:</h3>
    <div class="config-items">
      <div class="config-item">
        <label for="layers">层数:</label>
        <input id="layers" v-model.number="layers" type="number" min="1" max="100" @change="updateConfig" />
      </div>
      <div class="config-item">
        <label for="rows">行数:</label>
        <input id="rows" v-model.number="rows" type="number" min="1" max="100" @change="updateConfig" />
      </div>
      <div class="config-item">
        <label for="columns">列数:</label>
        <input id="columns" v-model.number="columns" type="number" min="1" max="100" @change="updateConfig" />
      </div>
    </div>
    <div class="config-info">
      <p>当前配置: {{ totalPoints }}个传感器点</p>
      <p>数据长度: {{ dataSize }}字节</p>
    </div>
  </div>
</template>

<script>
import { ref, computed, watch } from 'vue';

export default {
  name: 'ConfigPanel',
  props: {
    initialLayers: {
      type: Number,
      default: 8
    },
    initialRows: {
      type: Number,
      default: 8
    },
    initialColumns: {
      type: Number,
      default: 8
    }
  },
  emits: ['config-updated'],
  setup(props, { emit }) {
    const layers = ref(props.initialLayers);
    const rows = ref(props.initialRows);
    const columns = ref(props.initialColumns);

    const totalPoints = computed(() => {
      return layers.value * rows.value * columns.value;
    });

    const dataSize = computed(() => {
      const points = totalPoints.value;
      // 每个传感器点2字节
      // 如果小于512点，使用1068字节
      // 否则可能是2136字节
      return points < 512 ? 1068 : 2136;
    });

    const updateConfig = () => {
      emit('config-updated', {
        layers: layers.value,
        rows: rows.value,
        columns: columns.value,
        totalPoints: totalPoints.value,
        dataSize: dataSize.value
      });
    };

    // 初始化时触发一次更新
    watch(() => [props.initialLayers, props.initialRows, props.initialColumns], 
      ([newLayers, newRows, newColumns]) => {
        layers.value = newLayers;
        rows.value = newRows;
        columns.value = newColumns;
        updateConfig();
      }, 
      { immediate: true }
    );

    return {
      layers,
      rows,
      columns,
      totalPoints,
      dataSize,
      updateConfig
    };
  }
};
</script>

<style scoped>
.config-panel {
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

.config-items {
  display: flex;
  gap: 15px;
  margin-bottom: 15px;
}

.config-item {
  display: flex;
  flex-direction: column;
  flex: 1;
}

label {
  margin-bottom: 5px;
  font-weight: bold;
  color: var(--text-color, inherit);
}

input {
  padding: 8px;
  border: 1px solid var(--sidebar-border, #ddd);
  border-radius: 4px;
  font-size: 14px;
  background-color: var(--bg-color, white);
  color: var(--text-color, inherit);
}

.config-info {
  background-color: var(--sidebar-hover, #e0e0e0);
  padding: 10px;
  border-radius: 4px;
  font-size: 14px;
  color: var(--text-color, inherit);
}

.config-info p {
  margin: 5px 0;
}
</style> 