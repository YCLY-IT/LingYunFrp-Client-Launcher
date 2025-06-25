import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import router from './router'
import 'naive-ui/es/message/styles'
import { invoke } from '@tauri-apps/api/core';

const isDebug = await invoke<boolean>('get_now_mode')
if (!isDebug){
  document.addEventListener('keydown', (e) => {
    e.preventDefault()
  })
  document.addEventListener('contextmenu', (e) => {
     e.preventDefault() 
  })
  document.addEventListener('copy', (e) => {
    e.preventDefault() 
  })
  document.addEventListener('paste', (e) => {
    e.preventDefault()
  })
  document.addEventListener('cut', (e) => {
    e.preventDefault() 
  })
  //禁用开发者
  window.oncontextmenu = () => {
    return false 
  }
}

const app = createApp(App)

app.use(createPinia())
app.use(router)
app.mount('#app')

