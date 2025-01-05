import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';

export default defineConfig({
  plugins: [pluginReact()],
  html: {
    template: './public/index.html',
    favicon: './public/logo.svg',
    title: 'Rico Playground',
    meta: {
      description:
        'Rico Playground is a modern web application that provides a seamless interface for converting between Thrift IDL and JSON formats.',
      keywords: 'Rico, Playground, Thrift, JSON, Conversion, Web Application'
    }
  }
});
