const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

const distPath = path.resolve(__dirname, 'dist');

module.exports = (_, argv) => {
  return {
    devServer: {
      contentBase: distPath,
      compress: argv.mode === 'production',
      port: 8000,
    },
    entry: './src/main.js',
    experiments: {
      syncWebAssembly: true
    },
    output: {
      path: distPath,
      filename: 'website.js',
      webassemblyModuleFilename: 'website.wasm',
      assetModuleFilename: 'images/[hash][ext][query]'
    },
    module: {
      rules: [
        {
          test: /\.svg/,
          type: 'asset',
          
        },
        {
          test: /\.(png|jpg|gif)/,
          type: 'asset/resource'
        },
        {
          test: /\.css$/i,
          use: [
            MiniCssExtractPlugin.loader,
            'css-loader',
            'postcss-loader',
          ],
        },
        {
          test: /\.wasm$/,
          type: 'webassembly/sync',
        }
      ],
    },
    plugins: [
      new MiniCssExtractPlugin(),
      new CopyWebpackPlugin({
        patterns: [
          { from: './static', to: distPath },
          { from: './src/img', to: distPath+'/images' },
          { from: './static', to: distPath+'/images' },
        ],
      }),
      new WasmPackPlugin({
        crateDirectory: '.',
        extraArgs: '--no-typescript',
      })
    ],
  };
};

