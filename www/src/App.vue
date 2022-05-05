<template>
  <h1>ShaderPG</h1>
  <div style="display: flex">
    <div id="render-canvas"></div>
    <!-- <canvas id="render-canvas" style="width: 100%"></canvas> -->
    <Editor @shaderSource="onShaderSource" />
  </div>
</template>

<script>
import Editor from "./components/Editor.vue";

export default {
  name: "App",
  components: {
    Editor,
  },
  methods: {
    onShaderSource(source) {
      this.source = source;

      if (typeof this.update !== "undefined") {
        window.clearTimeout(this.update);
      }

      this.update = window.setTimeout(() => {
        if (typeof this.sender !== "undefined") {
          this.sender.send_shader(source);
        }
      }, 1000);
    },
  },
  async mounted() {
    const wasm = await import("shaderpg");
    this.sender = wasm.run("render-canvas");
  },
};
</script>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

/* #render-canvas {
  width: 100%;
  height: 100%;
}

#render-canvas canvas {
  background-color: black;
  width: 100%;
  height: 100%;
} */
</style>
