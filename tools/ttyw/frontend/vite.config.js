import process from "node:process";
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
  define: {
    __CONDA_PREFIX__: JSON.stringify(process.env.CONDA_PREFIX),
  },
};
