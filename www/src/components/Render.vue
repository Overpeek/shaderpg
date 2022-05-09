<template>
  <div id="render-canvas"></div>
</template>

<script>
export default {
  name: "Render",
  props: {
    code: {
      type: String,
      required: true,
    },
    texture: Uint8Array,
  },
  watch: {
    code: function () {
      if (this.sender !== "undefined") {
        console.log("sending shader");
        this.sender
          .new()
          .send_shader(this.code)
          .then((result) => console.log("result: ", result));
      }
    },
    texture: function () {
      if (this.sender !== "undefined") {
        console.log("sending texture");
        this.sender
          .new()
          .send_texture(0, this.texture)
          .then((result) => console.log("result: ", result));
      }
    },
  },
  async mounted() {
    const wasm = await import("shaderpg");
    this.sender = wasm.run("render-canvas");
    await this.sender.new().send_shader(this.code);
    await this.sender.new().send_texture(0, this.texture);
  },
};
</script>
