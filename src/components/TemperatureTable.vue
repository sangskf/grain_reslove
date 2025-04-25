<template>
  <div class="temperature-table">
    <h3>温度数据解析结果:</h3>
    <table>
      <thead>
        <tr>
          <th>传感器编号</th>
          <th>温度值 (℃)</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="(item, index) in temperatures" :key="index">
          <td>{{ item.sensorId }}</td>
          <td>{{ item.temperature.toFixed(1) }}</td>
        </tr>
      </tbody>
    </table>
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

tbody tr:nth-child(even) {
  background-color: #f9f9f9;
}

tbody tr:hover {
  background-color: #f0f0f0;
}
</style> 