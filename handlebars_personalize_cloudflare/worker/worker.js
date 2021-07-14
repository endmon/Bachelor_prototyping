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
    const greeting = await main()

    //let value = await user_KV.put("M. Test", 383);
    //let val = await user_KV.get("M. Test");

    //return new Response(JSON.stringify(greeting), {status: 200})
    return new Response(greeting, {status: 200})
}
