<template>
  <h1>ShaderPG</h1>
  <div id="flexbox">
    <div id="render-canvas"></div>
    <Editor @shaderSource="onShaderSource" :initial="initial" id="editor" />
  </div>
  <div style="width: 70%; margin: auto">
    <h2>Examples</h2>
    <h3>Default (uv quad)</h3>
    <Editor :initial="frag_a" :readonly="true" />
    <h3>Mandelbrot set</h3>
    <Editor :initial="frag_b" :readonly="true" />
    <h3>Ray tracer</h3>
    <Editor :initial="frag_c" :readonly="true" />
  </div>
</template>

<script>
import Editor from "./components/Editor.vue";

import frag_a from "raw-loader!@/assets/default.wgsl";
import frag_b from "raw-loader!@/assets/mandelbrot.wgsl";
import frag_c from "raw-loader!@/assets/ray-tracer.wgsl";

export default {
  name: "App",
  setup() {
    let initial = frag_b;
    if (localStorage.source) {
      initial = localStorage.source;
    }

    const state = { initial, frag_a, frag_b, frag_c };

    return state;
  },
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
  color: #aec2d3;
  margin-top: 60px;
}

#flexbox {
  display: flex;
}

#render-canvas {
  width: 50%;
}

#render-canvas canvas {
  background-color: black;
  width: 100% !important;
  height: 100% !important;
  max-height: 600px !important;
}

#editor {
  width: 50%;
  max-height: 600px;
}
</style>
