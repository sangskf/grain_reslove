<template>
  <div class="protocol-header">
    <h3>协议头部信息:</h3>
    <table>
      <tr>
        <th>字段</th>
        <th>值</th>
        <th>说明</th>
      </tr>
      <tr>
        <td>包头</td>
        <td>{{ headerInfo.header }}</td>
        <td>固定为 AA B0</td>
      </tr>
      <tr>
        <td>采集时间</td>
        <td>{{ headerInfo.timestamp || '未解析' }}</td>
        <td>数据采集的时间戳</td>
      </tr>
      <tr>
        <td>日期</td>
        <td>{{ headerInfo.date || '未解析' }}</td>
        <td>年-月-日</td>
      </tr>
      <tr>
        <td>时间</td>
        <td>{{ headerInfo.time || '未解析' }}</td>
        <td>时:分:秒</td>
      </tr>
      <tr>
        <td>设备编号</td>
        <td>{{ headerInfo.deviceId }}</td>
        <td>设备唯一标识</td>
      </tr>
      <tr>
        <td>原始数据长度</td>
        <td>{{ headerInfo.rawDataLength }}</td>
        <td>协议中的数据区字节长度</td>
      </tr>
      <tr>
        <td>调整后数据长度</td>
        <td>{{ headerInfo.dataLength }}</td>
        <td v-if="headerInfo.dataLength === headerInfo.rawDataLength">数据区字节长度</td>
        <td v-else class="adjusted-length">基于配置调整后的数据长度</td>
      </tr>
      <tr>
        <td>命令码</td>
        <td>{{ headerInfo.commandCode }}</td>
        <td>表示数据类型</td>
      </tr>
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
  background-color: #f5f5f5;
  border-radius: 4px;
}

h3 {
  margin-top: 0;
  margin-bottom: 10px;
}

table {
  width: 100%;
  border-collapse: collapse;
}

th, td {
  text-align: left;
  padding: 8px;
  border: 1px solid #ddd;
}

th {
  background-color: #f0f0f0;
  font-weight: bold;
}

tr:nth-child(even) {
  background-color: #f9f9f9;
}

.adjusted-length {
  color: #e53935;
  font-weight: bold;
}
</style> 