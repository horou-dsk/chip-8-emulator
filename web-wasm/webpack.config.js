const HtmlWebpackPlugin = require('html-webpack-plugin')
const webpack = require('webpack')
const path = require('path')
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

const dist = path.resolve(__dirname, "dist");

module.exports = {
  mode: 'development',
  devtool: 'inline-source-map',
  optimization: {
    minimize: true,
  },
  devServer: {
    contentBase: dist,
    hot: true,
    port: 1234,
    inline: true,
    host: '0.0.0.0'
  },
  entry: './index.ts',
  output: {
    filename: 'index.js',
    path: dist,
    publicPath: '/',
  },
  experiments: {
    asyncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: [ '.tsx', '.ts', '.js' ],
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: "./index.html"
    }),
    new webpack.HotModuleReplacementPlugin(),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
    new CopyPlugin([
      {
        from: path.resolve(__dirname, "../roms"),
        to: path.resolve(__dirname, 'dist/roms'),
      },
    ])
  ],
}
