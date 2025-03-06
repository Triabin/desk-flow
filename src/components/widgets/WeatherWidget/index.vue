<!-- 天气组件 -->
<template>

</template>
<script setup>
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

const weatherData = ref(null)

const fetchWeather = async () => {
  try {
    weatherData.value = await invoke('get_weather_data', { city: 'Beijing' })
  } catch (error) {
    console.error('天气数据获取失败:', error)
  }
}

onMounted(fetchWeather)
</script>
<style lang="scss" scoped>

</style>