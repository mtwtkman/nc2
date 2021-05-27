const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");

module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "index.js"
  },
  mode: "development",
  module: {
    rules: [
      {
        test: /\.wasm$/,
        use: "wasm-loader",
      },
    ],
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: [{ from: "index.html", to: "index.html" }],
    }),
  ],
}