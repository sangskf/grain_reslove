<template>
  <div class="container">
    <div class="container-bottom">
      <!-- 按层统计   -->
      <div class="table">
        <div class="total">
          <span class="block">
            {{countInfo.badCount}}
            <span class="desc">故障点</span>
          </span>
          <span class="block">
            {{countInfo.highCount}}
            <span class="desc">高温点</span>
          </span>
          <span class="block">
            {{countInfo.weatherCount}}
            <span class="desc">升温过快点</span>
          </span>
          <span class="block">
            {{countInfo.unNormalCount}}
            <span class="desc">异常点</span>
          </span>
        </div>
        <div class="line" v-for="(item, index) in storeyInfos" :key="'storeyInfos_' + index">
          <div class="cell1">第{{item.ch}}层</div>
          <div class="cell cell2">
            <div class="desc">最高温</div>
            <div class="num">{{rounding(item.zgw)}}</div>
          </div>
          <div class="cell cell3">
            <div class="desc">最低温</div>
            <div class="num">{{rounding(item.zdw)}}</div>
          </div>
          <div class="cell cell4">
            <div class="desc">平均温</div>
            <div class="num">{{rounding(item.pjw)}}</div>
          </div>
        </div>
      </div>
      <!-- 行列层数据展示   -->
      <div class="layer-con">
        <div class="layers">
          <!-- 左侧复选框 -->
          <div class="left-checkboxes">
            <div
              v-for="(matrix, matrixIndex) in layerArrs"
              :key="`left-checkbox-${matrixIndex}`"
              class="left-checkbox-item"
              :style="{
                'top': `${120 + (matrixIndex * (320/layerArrs.length))}px`
              }"
            >
              <span class="layer-text">第{{ matrix }}层</span>
              <input
                :title="`第${ matrix }层`"
                type="checkbox"
                class="custom-checkbox"
                v-model="layerVisibility[matrixIndex]"
                @change="updateVisibility"
              />
            </div>
          </div>
          <!-- 右侧行列层展示 -->
          <div
            class="layer"
            v-for="(matrix,index) in layerDatas"
            :key="`layer-${index}`"
            :style="{
              'top': `${index * (320/layerDatas.length)}px`,
              'z-index': layerDatas.length - index,
              'opacity': layerVisibility[index] ? 1 : 0,
              '--layer-index': index,
              'transform': 'skewX(-45deg)'
            }"
          >
            <div
              class="row"
              v-for="(row,rowIndex) in matrix"
              :key="`row-${rowIndex}`"
              :style="{'opacity': rowVisibility[rowIndex] ? 1 : 0}"
            >
              <div
                class="col"
                v-for="(num,colIndex) in row"
                :key="`col-${colIndex}`"
                :style="{
                  'background-color': colVisibility[colIndex] ? getColor(num) : 'transparent',
                  'opacity': colVisibility[colIndex] ? 1 : 0
                }"
              >
                <!--添加背景层-->
                <div class="col-bg"
                     :style="{
                      'background-color': colVisibility[colIndex] ? getColor(num) : 'transparent',
                      'opacity': colVisibility[colIndex] ? 1 : 0
                    }">
                  <span class="num">{{rounding(num)}}</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted } from 'vue'

// define props
const props = defineProps({
  temperatures: { type: Array, required: false, default: () => [] },
  layers: { type: Number, required: false, default: 8 },
  rows: { type: Number, required: false, default: 8 },
  columns: { type: Number, required: false, default: 8 },
  darkMode: { type: Boolean, required: false, default: false }
})

// reactive state
const colors = [
  'RGBA(70, 147, 207, 1)',
  'RGBA(93, 207, 70, 1)',
  'RGBA(207, 187, 70, 1)',
  'RGBA(207, 123, 70, 1)',
  'RGBA(207, 70, 70, 1)',
  'RGBA(207, 207, 207, 1)'
]
const darkModeColors = [
  'RGBA(70, 147, 207, 1)',
  'RGBA(93, 207, 70, 1)',
  'RGBA(207, 187, 70, 1)',
  'RGBA(207, 123, 70, 1)',
  'RGBA(207, 70, 70, 1)',
  'RGBA(150, 150, 150, 1)'
]
const countInfo = ref({})
const layerDatas = ref([])
const storeyInfos = ref([])
const layerArrs = ref([])
const rowArrs = ref([])
const colsArrs = ref([])
const layerVisibility = ref([])
const colVisibility = ref([])
const rowVisibility = ref([])

// utility functions
function getColor(num) {
  const colorSet = props.darkMode ? darkModeColors : colors
  if (num === -100) return colorSet[5]
  if (num < 10) return colorSet[0]
  if (num < 20) return colorSet[1]
  if (num < 25) return colorSet[2]
  if (num < 30) return colorSet[3]
  return colorSet[4]
}

// keep one decimal place when formatting numbers
function rounding(val) {
  return Number(val).toFixed(1)
}

function initVisibilityArrays() {
  if (layerDatas.value.length > 0) {
    layerVisibility.value = Array(layerDatas.value.length).fill(true)
    if (layerDatas.value[0]?.[0]) {
      colVisibility.value = Array(layerDatas.value[0][0].length).fill(true)
    }
    rowVisibility.value = Array(layerDatas.value[0].length).fill(true)
  }
}

function updateVisibility() {
  // optional: update stats or re-render
}

// Function to process temperatures from props into 3D grid
function processTemperatures() {
  if (!props.temperatures || props.temperatures.length === 0) return;

  // Create empty 3D array based on layers, rows, columns
  const totalLayers = props.layers || 8;
  const totalRows = props.rows || 8;
  const totalColumns = props.columns || 8;

  // Initialize the 3D grid with default values
  const tempGrid = Array(totalLayers)
    .fill()
    .map(() => 
      Array(totalRows)
        .fill()
        .map(() => Array(totalColumns).fill(-100))
    );
  
  // Fill in temperature values
  props.temperatures.forEach(temp => {
    // Calculate the position in the 3D grid
    const sensorId = temp.sensorId - 1; // Adjust for 0-based indexing
    const layer = Math.floor(sensorId / (totalRows * totalColumns));
    const remainder = sensorId % (totalRows * totalColumns);
    const row = Math.floor(remainder / totalColumns);
    const column = remainder % totalColumns;
    
    // Ensure we're within bounds
    if (layer >= 0 && layer < totalLayers && 
        row >= 0 && row < totalRows &&
        column >= 0 && column < totalColumns) {
      tempGrid[layer][row][column] = temp.temperature;
    }
  });
  
  // Update component state
  layerDatas.value = tempGrid;
  
  // Generate layer, row, column labels
  layerArrs.value = Array.from({ length: totalLayers }, (_, i) => i + 1);
  rowArrs.value = Array.from({ length: totalRows }, (_, i) => i + 1);
  colsArrs.value = Array.from({ length: totalColumns }, (_, i) => i + 1);
  
  // Calculate statistics for each layer
  storeyInfos.value = layerDatas.value.map((layer, idx) => {
    const temps = layer.flat().filter(t => t !== -100);
    const sum = temps.reduce((acc, val) => acc + val, 0);
    const avg = temps.length > 0 ? sum / temps.length : 0;
    
    return {
      ch: idx + 1,
      zgw: temps.length > 0 ? Math.max(...temps) : 0,
      zdw: temps.length > 0 ? Math.min(...temps) : 0,
      pjw: avg
    };
  });
  
  // Count abnormal points
  countInfo.value = {
    badCount: props.temperatures.filter(t => t.temperature === -100).length,
    highCount: props.temperatures.filter(t => t.temperature > 30).length,
    weatherCount: 0, // Need more info to calculate
    unNormalCount: props.temperatures.filter(t => t.temperature < 0 || t.temperature > 35).length
  };
  
  // Initialize visibility arrays
  initVisibilityArrays();
}

// Watch for temperature data changes from parent
watch(
  () => [props.temperatures, props.layers, props.rows, props.columns, props.darkMode],
  () => {
    processTemperatures();
  },
  { deep: true, immediate: true }
)

// initialize on mount
onMounted(() => {
  if (props.temperatures && props.temperatures.length > 0) {
    processTemperatures();
  } else {
    // Initialize empty arrays for visibility
    layerArrs.value = Array.from({ length: props.layers }, (_, i) => i + 1);
    rowArrs.value = Array.from({ length: props.rows }, (_, i) => i + 1);
    colsArrs.value = Array.from({ length: props.columns }, (_, i) => i + 1);
    initVisibilityArrays();
  }
})
</script>

<style scoped>
/* Container styles */
.container {
  width: 100%;
  height: 100%;
  padding: 15px;
  background: var(--bg-color, #f6f6f6);
  transition: background-color 0.3s ease;
}
:root {
  --text-color: #333;
  --text-color-light: #ffffff;
  --bg-color: #f6f6f6;
  --card-bg: linear-gradient(180deg, rgba(16, 50, 115, 0.8) 0%, rgba(0, 10, 37, 0.2) 50%, rgba(2, 144, 200, 0.6) 100%);
  --card-bg-dark: linear-gradient(180deg, rgba(10, 30, 70, 0.8) 0%, rgba(0, 5, 20, 0.5) 50%, rgba(1, 70, 100, 0.6) 100%);
  --highlight-color: #5df1ff;
  --shadow-color: rgba(0, 0, 0, 0.15);
  --cell-bg: rgba(20, 106, 197, 0.5);
  --cell-bg-dark: rgba(15, 80, 150, 0.6);
}
.dark-mode {
  --text-color: #eee;
  --text-color-light: #ffffff;
  --bg-color: #121212;
  --card-bg: var(--card-bg-dark);
  --highlight-color: #5df1ff;
  --shadow-color: rgba(0, 0, 0, 0.3);
  --cell-bg: var(--cell-bg-dark);
}
.container-bottom {
  display: flex;
  flex-wrap: wrap;
  width: 100%;
  height: 100%;
  gap: 20px;
}
.container-top {
  font-family: 'Microsoft YaHei', sans-serif;
  font-weight: bold;
  font-size: 16px;
  color: var(--text-color);
  padding: 20px 0 20px 40px;
  text-shadow: 0 0 10px rgba(93, 241, 255, 0.5);
}
.container-bottom .table {
  flex: 1;
  min-width: 300px;
  height: auto;
  min-height: 400px;
  background: var(--card-bg);
  border-radius: 20px;
  margin: 0 1.5% 0 1.5%;
  padding: 25px;
  box-shadow: 0 10px 30px var(--shadow-color), inset 0 0 15px rgba(93, 241, 255, 0.3);
  backdrop-filter: blur(5px);
  transition: all 0.3s ease;
}
.container-bottom .table:hover {
  box-shadow: 0 15px 35px rgba(0, 0, 0, 0.25), inset 0 0 20px rgba(93, 241, 255, 0.4);
  transform: translateY(-5px);
}
.container-bottom .table .total {
  margin-bottom: 29px;
  font-family: Impact, sans-serif;
  font-size: 28px;
  color: var(--highlight-color);
  line-height: 29px;
  text-align: center;
  display: flex;
  //text-shadow: 0 0 8px rgba(93, 241, 255, 0.7);
}
.container-bottom .table .total .block {
  flex: 1;
  transition: all 0.2s ease;
  padding: 10px 5px;
  border-radius: 10px;
}
.container-bottom .table .total .block:hover {
  background: rgba(93, 241, 255, 0.1);
  transform: scale(1.05);
}
.container-bottom .table .total .block .desc {
  display: block;
  font-family: 'Microsoft YaHei', sans-serif;
  font-weight: bold;
  font-size: 16px;
  color: var(--text-color-light);
  line-height: 21px;
  text-align: center;
  margin-top: 5px;
  opacity: 1;
}
.container-bottom .table .line {
  width: 100%;
  height: 70px;
  background: var(--cell-bg);
  border-radius: 9px;
  margin-bottom: 15px;
  font-family: 'Microsoft YaHei', sans-serif;
  font-weight: bold;
  font-size: 16px;
  color: var(--text-color-light);
  display: flex;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.1);
  transition: all 0.3s ease;
}
.container-bottom .table .line:hover {
  transform: translateX(5px);
  box-shadow: 0 8px 20px rgba(0, 0, 0, 0.15);
  background: linear-gradient(90deg, rgba(20, 106, 197, 0.4), rgba(20, 106, 197, 0.8), rgba(20, 106, 197, 0.4));
}
.container-bottom .table .line .cell1 {
  line-height: 70px;
  font-family: 'Microsoft YaHei', sans-serif;
  font-weight: bold;
  font-size: 16px;
  color: var(--text-color-light);
  padding: 0 20px;
  text-shadow: 0 0 5px rgba(255, 255, 255, 0.5);
}
.container-bottom .table .line .cell {
  flex: 1;
  margin-top: 7px;
  text-align: center;
  display: flex;
  flex-direction: column;
  justify-content: center;
}
.container-bottom .table .line .cell .desc {
  font-family: 'Microsoft YaHei', sans-serif;
  font-size: 16px;
  color: var(--text-color-light);
  line-height: 21px;
  opacity: 0.7;
}
.container-bottom .table .line .cell .num {
  display: inline-block;
  width: 100%;
  text-align: center;
  font-size: 18px;
  margin-top: 5px;
  //text-shadow: 0 0 8px rgba(93, 241, 255, 0.7);
}
/* Layer container */
.layer-con {
  flex: 2;
  min-width: 500px;
  max-width: 900px;
  height: auto;
  min-height: 500px;
  background: var(--card-bg);
  border-radius: 20px;
  position: relative;
  //padding: 60px 40px 40px 100px;
  padding: 0px 0px 0px 20px;
  margin: 30px;
  box-shadow: 0 10px 20px var(--shadow-color), inset 0 0 10px rgba(93, 241, 255, 0.2);
  backdrop-filter: blur(5px);
  transition: all 0.3s ease;
  will-change: transform, box-shadow;
  overflow: visible;
}
.layer-con:hover {
  box-shadow: 0 15px 25px rgba(0, 0, 0, 0.2), inset 0 0 15px rgba(93, 241, 255, 0.3);
}
.layer-con .layers {
  width: 500px;
  height: 300px;
  position: relative;
  transform-style: preserve-3d;
  perspective: 1200px;
}
/* Left checkboxes */
.layer-con .layers .left-checkboxes {
  position: absolute;
  left: -158px;
  top: 0;
  width: 150px;
  height: 100%;
  z-index: 100;
}
.layer-con .layers .left-checkboxes .left-checkbox-item {
  position: absolute;
  transition: transform 0.2s ease;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  padding: 8px 12px;
  background: transparent;
  //border-radius: 8px;
  //box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
}
.layer-con .layers .left-checkboxes .left-checkbox-item:hover {
  transform: translateX(-5px);
  background: transparent;
}
.layer-con .layers .left-checkboxes .left-checkbox-item .layer-text {
  color: var(--text-color);
  margin-right: 10px;
  font-family: 'Microsoft YaHei', sans-serif;
  font-size: 16px;
  font-weight: bold;
  //text-shadow: 0 0 6px rgba(93, 241, 255, 0.8);
  white-space: nowrap;
}
/* Custom checkbox styling */
.custom-checkbox {
  appearance: none;
  -webkit-appearance: none;
  width: 24px;
  height: 24px;
  border: 3px solid #0078d4;
  border-radius: 4px;
  color: var(--text-color);
  box-shadow: 0 0 5px rgba(0, 120, 212, 0.5);
  cursor: pointer;
  position: relative;
  transition: all 0.2s ease;
  margin-left: 10px;
}
.custom-checkbox:checked {
  background-color: #0078d4;
  box-shadow: 0 0 8px rgba(0, 120, 212, 0.7);
}
.custom-checkbox:checked::after {
  content: '✓';
  position: absolute;
  color: var(--text-color);
  font-size: 18px;
  font-weight: bold;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

/* Dark mode checkbox styling */
.dark-mode .custom-checkbox {
  border: 3px solid #5df1ff;
  background-color: rgba(30, 70, 120, 0.8);
  box-shadow: 0 0 8px rgba(93, 241, 255, 0.7);
}
.dark-mode .custom-checkbox:checked {
  background-color: #5df1ff;
  box-shadow: 0 0 12px rgba(93, 241, 255, 1);
}
.dark-mode .custom-checkbox:checked::after {
  color: #000;
}
/* Layer rows */
.layer {
  box-shadow: 0 8px 15px var(--shadow-color), 0 0 8px rgba(93, 241, 255, 0.2);
  position: absolute;
  transform: skewX(-45deg);
  background: rgba(221, 224, 226, 1);
  width: 500px;
  padding: 10px;
  border-radius: 8px;
  transition: transform 0.3s ease, box-shadow 0.3s ease;
  transform-style: preserve-3d;
  will-change: transform;
}
.dark-mode .layer {
  background: rgba(60, 60, 70, 1);
}
.layer:hover {
  box-shadow: 0 12px 20px rgba(0, 0, 0, 0.25), 0 0 12px rgba(93, 241, 255, 0.3);
}
.layer .row {
  display: flex;
  margin: 2px 0;
}
.layer .row .col {
  color: white;
  flex: 1;
  background: rgba(221, 224, 226, 1);
  border-radius: 4px;
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  margin: 0 2px;
}
.dark-mode .layer .row .col {
  background: rgba(50, 50, 60, 0.8);
}
.layer .row .col:hover {
  transform: scale(1.03);
  box-shadow: 0 3px 8px rgba(0, 0, 0, 0.8);
  z-index: 5;
}
.layer .row .col .col-bg {
  //padding: 2px 2px;
  //margin: 2px;
  border-radius: 4px;
  transition: box-shadow 0.2s ease;
}
.layer .row .col .col-bg:hover {
  box-shadow: inset 0 0 6px rgba(255, 255, 255, 0.8);
}
.layer .row .col .col-bg .num {
  display: inline-block;
  width: 100%;
  text-align: center;
  transform: skewX(45deg);
  font-weight: bold;
  text-shadow: 0 1px 2px rgba(0, 0, 0, 0.8);
  font-size: 14px;
}
.dark-mode .container {
  background: #121212;
}
.dark-mode .container-bottom .table {
  background: var(--card-bg-dark);
}
.dark-mode .layer-con {
  background: var(--card-bg-dark);
}
</style>