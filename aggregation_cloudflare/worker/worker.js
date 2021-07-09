addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {

    const { test } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const greeting = await test()


    //return new Response(JSON.stringify(greeting), {status: 200})
    return new Response(greeting, {status: 200})
}
