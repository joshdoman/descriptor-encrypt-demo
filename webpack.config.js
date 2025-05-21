const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './bootstrap.js',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  mode: 'development',
  experiments: {
    asyncWebAssembly: true,  // Enable WebAssembly
  },
  plugins: [
    new HtmlWebpackPlugin({
      template: 'index.html'
    }),
    new CopyWebpackPlugin({
      patterns: [
        { 
          from: 'pkg/descriptor_encrypt_demo_bg.wasm', 
          to: '[name][ext]'
        }
      ]
    }),
  ],
  devServer: {
    static: './dist',
    hot: true
  }
};