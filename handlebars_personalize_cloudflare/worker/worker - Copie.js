/*addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

async function gatherResponse(response) {
    const { headers } = response
    const contentType = headers.get("content-type") || ""
    if (contentType.includes("application/json")) {
        return JSON.stringify(await response.json())
    }
    else if (contentType.includes("application/text")) {
        return await response.text()
    }
    else if (contentType.includes("text/html")) {
        return await response.text()
    }
    else {
        return await response.text()
    }
}
const COOKIE_NAME = "visite"

/**
 *  * Gets the cookie with the name from the request headers
 *  * @param {Request} request incoming Request
 *  * @param {string} name of the cookie to get

function getCookie(request, name) {
    let result = ""
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


function checkCookie(request, name){
    result = false
    if (getCookie(request, name) == true) {
        result =true
    }
    return result
}


/**
 * Fetch and log a request
 * @param {Request} request

async function handleRequest(request) {

    /*const init = {
        headers: {
            "content-type": "application/json;charset=UTF-8",
        },
    }
    const { greet } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const greeting = greet()
    //JSON.stringify(greeting);
    //const response_a = await fetch("http://miguel-gouveia.me/a", init)
    //const response_b = await fetch("http://miguel-gouveia.me/b", init)
    //const response_c = await fetch("http://miguel-gouveia.me/c", init)
    //const results_a = await gatherResponse(response_a)
    //const results_b = await gatherResponse(response_b)
    //const results_c = await gatherResponse(response_c)
    //aggrega = results_a + results_b + results_c
    let response = await fetch(request);


    if (checkCookie(request, COOKIE_NAME) == true){
        response.body
    }else {
        reponse = "C'est votre première visite"
        response = new Response(response.body, response);
        response.headers.set('Set-Cookie', "visite=true");
        reponse = response
    }

    return new Response(reponse, {status: 200})
}
*/
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
