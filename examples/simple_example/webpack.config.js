const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
module.exports = {
  entry: "./index.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
    clean: true,
  },
  experiments: {
    asyncWebAssembly: true,
  },
  devServer: {
    static: path.resolve(__dirname, "dist"),
    compress: true,
    port: 5000,
    open: true,
    hot: true,
  },
  module: {
    rules: [
      {
        test: /\.ttf/,
        loader: "file-loader",
        options: {
          name: "[path][name].[ext]",
        },
      },
    ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "index.html",
      filename: "index.html",
    }),
  ],
  mode: "development",
};
