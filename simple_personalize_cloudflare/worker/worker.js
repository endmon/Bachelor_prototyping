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

    const { test } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const greeting = test(request)
    let response = new Response("originalResponse.body", {statusText: "some message", })


    response.headers.set("foo", "bar")
    let msg = ""
    if ( getCookie(request, "visite") == false){
        msg = "C'est votre première visite"
        response.headers.set("Set-Cookie", "visite=true")
    } else {
        msg = "Bon retour!"
    }

    //return new Response(JSON.stringify(greeting), {status: 200})
    return new Response(msg, response)
}
