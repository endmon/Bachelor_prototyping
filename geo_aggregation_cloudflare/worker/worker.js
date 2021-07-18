addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})


async function gatherResponse(response) {
    const { headers } = response
    const contentType = headers.get("content-type") || ""
    if (contentType.includes("application/json")) {
        return await response.blob()
    }
    else if (contentType.includes("application/text")) {
        return await response.blob()
    }
    else if (contentType.includes("text/html")) {
        return await response.blob()
    }
    else {
        return await response.blob()
    }
}

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {

    const init = {
        headers: {
            "content-type": "application/vnd.mapbox-vector-tile",
        },
    }

    //JSON.stringify(greeting);
    const response_a = await fetch("http://miguel-gouveia.me/geo/14/8494/5777.pbf", init)
    const results_a = await gatherResponse(response_a)
    var result_size = results_a.size

    const { test } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const greeting = await test(request)

    //let value = await user_KV.put("M. Test", 383);
    //let val = await user_KV.get("M. Test");

    //return new Response(JSON.stringify(greeting), {status: 200})
    return new Response(results_a, {status: 200, headers: { "content-type": "aplication/vnd.mapbox-vector-tile",},})
}
