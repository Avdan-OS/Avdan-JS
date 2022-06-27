// import Clipboard from "@avdan/clipboard";
// import * as test from "./test.mjs";
import test2 from "./test_module.mjs";


await Avdan.Debug.wait(100, 60_00)
    .on("tick", tick => {
        // Avdan.Debug.log(`Tick #${tick.tick}`);
        Avdan.Debug.log(test2);
    })

export {};