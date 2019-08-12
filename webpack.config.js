const path = require("path");
const CopyPlugin = require("copy-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

const dist = path.resolve(__dirname, "pkg");

module.exports = {
  mode: "production",
  entry: {
    index: "./js/index.js"
  },
  output: {
    path: dist,
    filename: "[name].js"
  },
  devServer: {
    contentBase: dist,
  },
  plugins: [
    new CopyPlugin([
      path.resolve(__dirname, "static")
    ]),

    new WasmPackPlugin({
      crateDirectory: __dirname,
      extraArgs: "--out-name index",
      watchDirectories: [
        path.resolve(__dirname, "../smithy_css/crates/smithy_css/src"),
        path.resolve(__dirname, "../smithy_css/crates/smithy_css_macro/src"),
        path.resolve(__dirname, "../smithy/crates/smd_macro/src"),
        path.resolve(__dirname, "../smithy/crates/smithy/src"),
        path.resolve(__dirname, "../smithy/crates/smithy_core/src"),
        path.resolve(__dirname, "../smithy/crates/smithy_types/src"),
        path.resolve(__dirname, "../smithy/crates/smd_tests/src"),
      ],
    }),
  ]
};
