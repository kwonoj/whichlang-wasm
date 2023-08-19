### Whichlang-wasm

`whichlang-wasm` is a thin webassembly bindings to [whichlang](https://github.com/quickwit-oss/whichlang) library.

### Installation

```bash
npm install whichlang-wasm
```

### Usage

This bindings exports a single interface `detectLanguage` which takes a string and returns a iso 6391 based language code.

```ts
const { detectLanguage } = require("whichlang-wasm");

const text = "한글 텍스트";
const language = detectLanguage(text);

console.log(language) // KO
```