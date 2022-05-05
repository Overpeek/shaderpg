<template>
  <prism-editor
    class="editor"
    v-model="code"
    :highlight="highlighter"
    :line-numbers="lineNumbers"
  >
  </prism-editor>
</template>

<script>
import { reactive } from "vue";

import { PrismEditor } from "vue-prism-editor";
import "vue-prism-editor/dist/prismeditor.min.css";

import { highlight, languages } from "prismjs/components/prism-core";
import "prismjs/components/prism-rust";
import "prismjs/themes/prism-tomorrow.css";

import frag from "raw-loader!@/assets/shaderpg.frag.wgsl";

export default {
  name: "Editor",
  components: {
    PrismEditor,
  },
  emits: ["shaderSource"],
  setup() {
    const state = reactive({
      code: frag,
      lineNumbers: true,
    });

    if (localStorage.source) {
      state.code = localStorage.source;
    }

    return state;
  },
  methods: {
    highlighter(code) {
      this.$emit("shaderSource", code);
      localStorage.source = code;
      return highlight(code, languages.rust); //returns html
    },
  },
};
</script>

<style scoped>
.editor {
  height: inherit;

  background: #2d2d2d;
  color: #ccc;

  font-family: Fira code, Fira Mono, Consolas, Menlo, Courier, monospace;
  font-size: 14px;
  line-height: 1.5;
  padding: 0;
}

.prism-editor__textarea:focus {
  outline: none;
}
</style>
