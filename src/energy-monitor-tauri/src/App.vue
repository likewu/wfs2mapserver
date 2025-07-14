<script lang="ts">
import { defineComponent, ref } from 'vue'
import MyChart from './components/MyChart.vue'
import { invoke } from "@tauri-apps/api/core"
import { listen } from '@tauri-apps/api/event'

export default defineComponent({
  name: 'App',
  components: {
    MyChart,
  },
  data(){
      return {
          name: ref(""),
          greetMsg: ref(""),
          dataValues: new Array(40).fill(10),
          demo: ref(false) 
      };
  },
  async mounted(){
    await invoke('init_process');
    listen("distance_emitter", x => {
        this.dataValues.push((x as any).payload as string);
        if(this.dataValues.length > 40) {
          this.dataValues.shift();
        } 
    });
  },
  
  methods: {
    async greet(){ 
      this.greetMsg = await invoke("greet", { name: this.name });
    },
    async toggleDemo(){ 
      this.demo = await invoke("toggle_demo");
    }
  },

});
</script>

<template>
  <main class="container">
    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>

    <h2> ⚠️ Important security information ⚠️ </h2>
    <h3> 
      ... brought to you by a rodent 🐿️.     
      <button @click="toggleDemo()">{{demo ? 'DEMO' : 'LIVE'}}</button>
    </h3>
    <a href="/three">three js</a>
    <router-link to="/three">three js</router-link>
    

    <div class="chart-container" style="margin:0 auto; width:70vw">
      <MyChart :chartSensorData="dataValues"/>
    </div>

    <div class="chart-container" style="margin:0 auto; width:70vw">
      <router-view></router-view>
    </div>
  </main>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>