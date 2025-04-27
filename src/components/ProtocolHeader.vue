<template>
  <div class="protocol-header">
    <h3>协议头部信息:</h3>
    <table>
      <thead>
        <tr>
          <th>字段</th>
          <th>值</th>
          <th>说明</th>
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>包头</td>
          <td>{{ headerInfo.header || 'AA B0' }}</td>
          <td>固定为 AA B0</td>
        </tr>
        <tr>
          <td>采集时间</td>
          <td>{{ headerInfo.timestamp || '未解析' }}</td>
          <td>数据采集的时间戳</td>
        </tr>
        <tr>
          <td>分机地址</td>
          <td>{{ headerInfo.deviceId !== null ? headerInfo.deviceId : '未知' }}</td>
          <td>分机地址</td>
        </tr>
        <tr>
          <td>原始数据长度</td>
          <td>{{ headerInfo.rawDataLength || 0 }}</td>
          <td>协议中的数据区字节长度</td>
        </tr>
        <tr>
          <td>调整后数据长度</td>
          <td>{{ headerInfo.dataLength || 0 }}</td>
          <td v-if="headerInfo.dataLength === headerInfo.rawDataLength">数据区字节长度</td>
          <td v-else class="adjusted-length">基于配置调整后的数据长度</td>
        </tr>
      </tbody>
    </table>
  </div>
</template>

<script>
export default {
  name: 'ProtocolHeader',
  props: {
    headerInfo: {
      type: Object,
      required: true
    }
  }
};
</script>

<style scoped>
.protocol-header {
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

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  text-align: left;
  padding: 8px;
  border: 1px solid var(--sidebar-border, #ddd);
  color: var(--text-color, inherit);
}

th {
  background-color: var(--sidebar-hover, #f0f0f0);
  font-weight: bold;
}

tr:nth-child(even) {
  background-color: var(--sidebar-bg, #f9f9f9);
  opacity: 0.9;
}

.adjusted-length {
  color: var(--error-badge-text, #e53935);
  font-weight: bold;
}
</style> 