# rs-wasm-fetch-demo

A tiny demo showing how to call fetch from Rust compiled to WebAssembly and use it in a simple Vite + TypeScript page. The WASM module is imported and used from `pkg/www/src/main.ts`.

## Prerequisites
- Rust toolchain (stable)
- wasm-pack (`cargo install wasm-pack`)
- Node.js 18+ (for the Vite dev server)

## Build the WASM package
From the project root:

```
cd rs-wasm-fetch-demo
wasm-pack build pkg/bin --target web
```

This generates JS bindings and a `.wasm` file under `rs-wasm-fetch-demo/pkg/bin/pkg/` (e.g. `rs_wasm_fetch_demo_bin.js`). The Vite app imports from that folder.

## Run the Vite dev server
```
cd pkg/www
npm install
npm run dev
```
Then open the printed local URL (typically http://localhost:5173). Click "Fetch JSON" to see the WASM-powered request in action.

Note: The WASM initialization and click handler live in `pkg/www/src/main.ts` (moved from inline `<script>` in `index.html`). `vite.config.js` is configured to allow serving files from `../bin/pkg` so the generated WASM and JS can be imported by the web app.

## Alternative: Serve a static build
If you prefer a static server without Vite:
1) Build the WASM (as above)
2) Build the static site:
```
cd pkg/www
npm run build
```
3) Serve the `pkg/www/dist` folder with any static server. Example:
```
python3 -m http.server 8080 --directory dist
```
Open http://localhost:8080/ in your browser.

## Troubleshooting
- If import fails in the browser, ensure you have run `wasm-pack build pkg/bin --target web` and the output exists in `pkg/bin/pkg`.
- After changing Rust code, rebuild via wasm-pack and restart/refresh the dev server.
- If Vite cannot access the WASM package, check `vite.config.js` fs.allow paths.

Updated: 2025-08-30
