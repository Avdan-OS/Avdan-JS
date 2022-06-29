import Debug from "@avdan/debug";

await Debug.wait(15, 500)
    .on("tick", (obj) => {
        Debug.log(`Got tick #${obj.tick} !`);
    });

Debug.log("Welcome to Avdan.JS !");