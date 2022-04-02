function logReq(req) {
    let img = req.url.split("/").pop();

    return {
        redirectUrl: `https://localhost:8000/${img}`
    };
}

browser.webRequest.onBeforeRequest.addListener(
    logReq,
    {urls: ["https://hot-potato.reddit.com/*"]},
    ["blocking"]
);
