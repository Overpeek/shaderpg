<template>
  <tabs :options="{ useUrlFragment: false }" class="tabs">
    <tab name="WGSL">
      <Editor @codeUpdated="wgsl" :initial="initial" />
    </tab>
    <!-- <tab name="GLSL">
      <Editor :initial="'glsl\ngenerated'" />
    </tab> -->
    <tab name="Input">
      <div>
        <h3>Texture Slot 0</h3>
        <input
          ref="texture_0"
          type="file"
          accept=".png,.jpg,.qoi"
          @input="pick_texture_0"
        />
      </div>
    </tab>
    <tab name="Errors">
      <Editor initial="some errors" :readonly="true" />
    </tab>
  </tabs>
</template>

<script>
import Editor from "./Editor.vue";

import { Tabs, Tab } from "vue3-tabs-component";

export default {
  name: "ShaderEditor",
  props: {
    initial: String,
  },
  components: { Editor, Tabs, Tab },
  setup(props) {
    var initial = props.initial;
    if (localStorage.wgsl_code) {
      initial = localStorage.wgsl_code;
    }
    return {
      initial,
    };
  },
  methods: {
    wgsl(code) {
      localStorage.wgsl_code = code;
      this.$emit("wgslCodeUpdated", code);
    },
    /* glsl(code) {
      localStorage.glsl_code = code;
      this.$emit("glslCodeUpdated", code);
    }, */
    pick_texture_0() {
      let input = this.$refs.texture_0;
      let file = input.files;
      if (file && file[0]) {
        let reader = new FileReader();
        reader.onload = (e) => {
          this.$emit("textureUpdated", new Uint8Array(e.target.result));
        };
        reader.readAsArrayBuffer(file[0]);
      }
    },
  },
};
</script>

<style :scoped>
.tabs ul {
  padding: 2px 0px;
  display: flex;
  justify-content: center;
  list-style-type: none;
  background-color: rgb(53, 65, 65);
  margin: 0px;
}

.tabs li a {
  padding: 3px 10px;
  color: black;
  text-decoration: none;
}

.tabs li:hover a {
  background-color: rgb(65, 80, 80);
}

.tabs li.is-active a {
  background-color: rgb(83, 100, 100);
}
</style>
