# Astro Surf

[![NPM version](https://img.shields.io/npm/v/astro-surf.svg?style=for-the-badge)](https://www.npmjs.com/package/astro-surf)

Astro Surf is a server side rendering library for Astro built with [Axum](https://github.com/tokio-rs/axum)


## Installation

```bash
npm install astro-surf
```
or
```bash
yarn add astro-surf
```


## Usage

```js
import AstroSurf from "astro-surf";
import { handler as astroApp } from "./dist/server/entry.mjs";

async function main() {
  console.log('Starting app...');
  let app = AstroSurf.initialize(astroApp, {
    client_path: `${process.cwd()}/dev/dist/client`
  });
  await app.serve(3000)
}
main();

```


