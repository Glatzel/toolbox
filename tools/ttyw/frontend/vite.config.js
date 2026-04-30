export default {
  base: "./",
  server: {
    proxy: {
      "/ws": {
        target: "ws://127.0.0.1:7681",
        ChangeOrigin: true,
        ws: true,
        configure: (proxy) => {
          console.log("WS proxy active");
        },
      },
    },
  },
};
