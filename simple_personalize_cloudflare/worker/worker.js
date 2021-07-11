addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

function getCookie(request, name) {
    let result = false
    const cookieString = request.headers.get("Cookie")
    if (cookieString) {
        const cookies = cookieString.split(";")
        cookies.forEach(cookie => {
            const cookiePair = cookie.split("=", 2)
            const cookieName = cookiePair[0].trim()
            if (cookieName === name) {
                const cookieVal = cookiePair[1]
                result = true
            }
        })
    }
    return result
}

const headerNameSrc = "foo" //"Orig-Header"
const headerNameDst = "Last-Modified"

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {

    const originalResponse = await fetch(request)

    const { test } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const greeting = test(request)
    let response = new Response(originalResponse.body, {    status: 200,    statusText: "some message",    headers: originalResponse.headers,  })

    const originalBody = await originalResponse
    const body = JSON.stringify({ foo: "bar", ...originalBody })
    response = new Response(body, response)

    response.headers.set("foo", "bar")
    let msg = ""
    if ( getCookie(request, "visite") == false){
        msg = "C'est votre première visite"
        response.headers.set("Set-Cookie", "visite=true")
    }

    const src = response.headers.get(headerNameSrc)

    if (src != null) {
        response.headers.set(headerNameDst, src)
        console.log(
            `Response header "${headerNameDst}" was set to "${response.headers.get(
                headerNameDst,
            )}"`, msg,
        )
    }

    //return new Response(JSON.stringify(greeting), {status: 200})
    return response
}
