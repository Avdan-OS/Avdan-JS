(async () => {
    // const [type, content] = await
    //         Avdan.Clipboard.source("primary")
    //             .read("text/plain", "UTF8_STRING");
    
    const [type, content] =  await Avdan.Clipboard.source("primary").read("text/plain", "UTF8_STRING");
    Avdan.Debug.log({type, content});
})();