<!-- 软件入口 -->
<template>
    <div>
      <h1>轻桌</h1>
      <div>
        <n-button @click="refresh">naive-ui</n-button>
      </div>
      <n-card :title="wData?.city">
        <span>天气：{{ wData?.weather }}</span><br/>
        <span>温度：{{ wData?.currentTemp }}</span>
      </n-card>
    </div>
</template>
<script setup>
import { ref, onMounted } from "vue";
import { NButton, NCard } from 'naive-ui';
import { weatherData } from "./api/weather.js";

const wData = ref({});

const refresh = () => weatherData().then(res => wData.value = res);

onMounted(() => {
  refresh();
})
</script>
<style lang="scss" scoped>
.main {
  /* Add a nice colorscheme */
  background-color: #ececec;
  color: #222831;

  /* Make the body the exact size of the window */
  margin: 0;
  height: 100vh;
  width: 100vw;

  /* Vertically and horizontally center children of the body tag */
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>