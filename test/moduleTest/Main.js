// import File from "@avdan/file";
// import {returnImportedString} from "./test.mjs";
import testJSON from "./data.json";

await Avdan.Debug.wait(100, 1200)
    .on("tick", async tick => {
        Avdan.Debug.log(testJSON);
    });