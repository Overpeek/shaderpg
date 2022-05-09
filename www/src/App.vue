<template>
  <h1>ShaderPG</h1>
  <div class="canvas-editor-div">
    <Render class="canvas" :code="code" :texture="texture" />
    <ShaderEditor
      @wgslCodeUpdated="wgslCodeUpdated"
      @textureUpdated="textureUpdated"
      :initial="initial"
      class="editor"
    />
  </div>
</template>

<script>
import ShaderEditor from "./components/ShaderEditor.vue";
import Render from "./components/Render.vue";

import img from "raw-loader!@/assets/logo.png";
import frag_a from "raw-loader!@/assets/default.wgsl";
import frag_b from "raw-loader!@/assets/mandelbrot.wgsl";
import frag_c from "raw-loader!@/assets/ray-tracer.wgsl";

export default {
  name: "App",
  components: {
    ShaderEditor,
    Render,
  },
  data() {
    return {
      initial: frag_a,
      code: frag_a,
      texture: new Uint8Array(img),
    };
  },
  methods: {
    wgslCodeUpdated(code) {
      this.code = code;
    },
    textureUpdated(data) {
      console.log("got texture");
      this.texture = data;
    },
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
  margin-top: 20px;
}
</style>

<style scoped>
.canvas-editor-div {
  display: flex;
  flex-wrap: wrap;
  justify-content: center;
}

.canvas-editor-div .canvas {
  width: 600px;
  height: 400px;
}

.canvas-editor-div .editor {
  flex: 1 1 600px;
}
</style>
