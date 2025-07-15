import { createApp } from "vue";
import { createMemoryHistory, createRouter } from 'vue-router'

import App from "./App.vue";
import Three from './components/Three.vue'
import Three2 from './components/Three2.vue'

const routes = [
  { path: '/', component: App },
  { path: '/three', component: Three },
  { path: '/three2', component: Three2 },
]
const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

createApp(App).use(router).mount("#app");
