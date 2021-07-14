addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})



/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {

    var data = {
        users: [ {
            person: {
                firstName: "Garry",
                lastName: "Finch"
            },
            jobTitle: "Front End Technical Lead",
            twitter: "gazraa"
        }, {
            person: {
                firstName: "Garry",
                lastName: "Finch"
            },
            jobTitle: "Photographer",
            twitter: "photobasics"
        }, {
            person: {
                firstName: "Garry",
                lastName: "Finch"
            },
            jobTitle: "LEGO Geek",
            twitter: "minifigures"
        } ]
    };

    const { main } = wasm_bindgen;
    await wasm_bindgen(wasm)
    const greeting = await main(JSON.stringify(data))

    //let value = await user_KV.put("M. Test", 383);
    //let val = await user_KV.get("M. Test");

    //return new Response(JSON.stringify(greeting), {status: 200})
    return new Response(greeting, {status: 200})
}
