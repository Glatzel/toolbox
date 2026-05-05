export default {
  base: "./",
  server: {
    proxy: {
      "/ws": {
        target: "ws://127.0.0.1:7681",
        ChangeOrigin: true,
        ws: true,
      },
    },
  },
  build: {
    rolldownOptions: {
      output: {
        manualChunks: (id) => {
          if (id.includes("addon-webgl")) return "xterm-webgl";
          if (id.includes("addon-image")) return "xterm-image";
          if (id.includes("@xterm/xterm")) return "xterm-core";
          if (id.includes("@xterm/addon")) return "xterm-addons";
        },
      },
    },
  },
};
