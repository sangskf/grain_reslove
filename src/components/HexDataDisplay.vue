<template>
  <div class="hex-display">
    <h3>接收数据:</h3>
    <div class="hex-data">{{ formattedHexData }}</div>
  </div>
</template>

<script>
import { computed } from 'vue';

export default {
  name: 'HexDataDisplay',
  props: {
    hexData: {
      type: String,
      required: true
    }
  },
  setup(props) {
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
      formattedHexData
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

h3 {
  margin-top: 0;
  margin-bottom: 10px;
}

.hex-data {
  font-family: monospace;
  white-space: pre-wrap;
  background-color: #e0e0e0;
  padding: 10px;
  border-radius: 4px;
  overflow-x: auto;
  line-height: 1.5;
}
</style> 