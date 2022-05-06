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

export default {
  name: "Editor",
  components: {
    PrismEditor,
  },
  props: {
    initial: String,
  },
  emits: ["shaderSource"],
  setup(props) {
    const state = reactive({
      code: props.initial,
      lineNumbers: true,
      readonly: props.readonly,
    });

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
  background: #222526;
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
