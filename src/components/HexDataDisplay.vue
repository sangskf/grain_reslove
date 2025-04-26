<template>
  <div class="hex-display">
    <div class="header">
    <h3>接收数据:</h3>
      <button class="toggle-btn" @click="toggleDisplay">
        {{ isVisible ? '隐藏' : '显示' }}
      </button>
    </div>
    <div class="hex-data" v-if="isVisible">{{ formattedHexData }}</div>
  </div>
</template>

<script>
import { computed, ref } from 'vue';

export default {
  name: 'HexDataDisplay',
  props: {
    hexData: {
      type: String,
      required: true
    }
  },
  setup(props) {
    const isVisible = ref(false);
    
    const toggleDisplay = () => {
      isVisible.value = !isVisible.value;
    };
    
    // 格式化16进制响应数据以便于显示
    const formattedHexData = computed(() => {
      if (!props.hexData) return '';
      
      const hexArray = props.hexData.trim().split(/\s+/);
      let formattedHex = '';
      
      for (let i = 0; i < hexArray.length; i++) {
        formattedHex += hexArray[i] + ' ';
        if ((i + 1) % 16 === 0) {
          formattedHex += '\n';
        }
      }
      
      return formattedHex;
    });
    
    return {
      formattedHexData,
      isVisible,
      toggleDisplay
    };
  }
};
</script>

<style scoped>
.hex-display {
  margin-top: 20px;
  padding: 15px;
  background-color: #f5f5f5;
  border-radius: 4px;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 10px;
}

h3 {
  margin: 0;
}

.toggle-btn {
  padding: 4px 8px;
  background-color: #4CAF50;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 12px;
  transition: background-color 0.2s;
}

.toggle-btn:hover {
  background-color: #45a049;
}

.hex-data {
  font-family: monospace;
  white-space: pre-wrap;
  background-color: #e0e0e0;
  padding: 10px;
  border-radius: 4px;
  overflow-x: auto;
  line-height: 1.5;
  max-height: 300px;
  overflow-y: auto;
  font-size: 14px;
  border: 1px solid #ccc;
}

.hex-data::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}

.hex-data::-webkit-scrollbar-track {
  background: #f1f1f1;
  border-radius: 4px;
}

.hex-data::-webkit-scrollbar-thumb {
  background: #888;
  border-radius: 4px;
}

.hex-data::-webkit-scrollbar-thumb:hover {
  background: #555;
}
</style> 