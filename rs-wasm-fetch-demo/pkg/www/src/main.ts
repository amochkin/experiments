import './style.css'

// Render UI
const app = document.querySelector<HTMLDivElement>('#app')!
app.innerHTML = `
  <div>
    <h1>WASM Fetch Demo</h1>
    <div class="card">
      <button id="btn" type="button">Fetch+Post JSON</button>
      <pre id="out"></pre>
    </div>
    <p class="read-the-docs">
      Click Fetch JSON to fetch data via WASM
    </p>
  </div>
`

// Move WASM fetch logic from index.html into here
;(async () => {
  try {
    const { default: init, fetch_json, post_json } = await import('../../bin/pkg/rs_wasm_fetch_demo_bin.js')

    await init()

    const out = document.getElementById('out') as HTMLElement | null
    const btn = document.getElementById('btn') as HTMLButtonElement | null

    if (btn && out) {
      btn.addEventListener('click', async () => {
        btn.disabled = true
        out.textContent = 'Loading...'
        try {
          const data = await fetch_json('https://jsonplaceholder.typicode.com/todos/1')
          out.textContent = JSON.stringify(data, null, 2)

          // Example POST:
          const res = await post_json('https://httpbin.org/post', JSON.stringify({ hello: 'world' }))
          // Show POST result below the GET result
          out.textContent = out.textContent + "\n\nPOST Result:\n" + JSON.stringify(res, null, 2)
          // eslint-disable-next-line no-console
          console.log('POST result', res)
        } catch (e) {
          out.textContent = 'Error: ' + e
        } finally {
          btn.disabled = false
        }
      })
    }
  } catch (e) {
    const out = document.getElementById('out') as HTMLElement | null
    if (out) out.textContent = 'Failed to initialize WASM: ' + e
    // eslint-disable-next-line no-console
    console.error('Failed to initialize WASM', e)
  }
})()
