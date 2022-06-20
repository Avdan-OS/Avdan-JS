// import Clipboard from "@avdan/clipboard";
import {helloWorld} from "./test.mjs";

Avdan.Debug.log(A());

await Avdan.Debug.wait(10, 1_000)
    .on("tick", tick => {
        Avdan.Debug.log(`Tick #${tick.tick} !`);
    })

export {};