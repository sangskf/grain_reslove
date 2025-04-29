import { defineStore } from 'pinia'
import { DEFAULT_CONFIG } from '../utils/dataSample'

export const useConfigStore = defineStore('config', {
  state: () => ({
    config: { ...DEFAULT_CONFIG }
  }),
  
  actions: {
    updateConfig(newConfig) {
      this.config = { ...newConfig }
    },
    
    loadConfig() {
      try {
        const savedConfig = localStorage.getItem('appConfig')
        if (savedConfig) {
          const parsedConfig = JSON.parse(savedConfig)
          if (typeof parsedConfig === 'object' && parsedConfig !== null) {
            this.config = {
              ...DEFAULT_CONFIG,
              ...parsedConfig
            }
          }
        }
      } catch (e) {
        console.error('加载配置失败:', e)
        this.config = { ...DEFAULT_CONFIG }
      }
    },

    saveConfig(configToSave) {
      try {
        localStorage.setItem('appConfig', JSON.stringify(configToSave))
        this.config = { ...configToSave }
      } catch (e) {
        console.error('保存配置失败:', e)
      }
    }
  }
})