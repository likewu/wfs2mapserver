import { createApp } from "vue";
import { createMemoryHistory, createRouter } from 'vue-router'

import App from "./App.vue";
import Three from './components/Three.vue'

const routes = [
  { path: '/', component: App },
  { path: '/three', component: Three },
]
const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

createApp(App).use(router).mount("#app");
