import App from "./App.vue";

// 此文件运行在 Node.js 服务器上
import { createSSRApp } from 'vue'
// Vue 的服务端渲染 API 位于 `vue/server-renderer` 路径下
import { renderToString } from 'vue/server-renderer'

const app = createSSRApp(App)

renderToString(app).then((html) => {
  console.log(html)
})