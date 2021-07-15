addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})



/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {

    const { main } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const greeting = await main(request)

    let newResponce = new Response(greeting, {status: 200})
    newResponce.headers.set("Content-Type", "text/html")
    return newResponce
}
