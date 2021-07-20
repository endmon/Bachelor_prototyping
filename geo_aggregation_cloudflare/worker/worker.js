addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})


/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {

    const init = {
        status: 200,
        headers: {
            "content-type": "application/vnd.mapbox-vector-tile",
        },
    }



    const { test } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const greeting = await test(request)

    var blob = new Blob([new Uint8Array(greeting)]);

    return new Response(blob, init)
}
