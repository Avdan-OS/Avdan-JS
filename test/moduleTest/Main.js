import Clipboard from "@avdan/clipboard";

await Avdan.Debug.wait(100, 1200)
    .on("tick", async tick => {
        try {
            Avdan.Debug.log({
                text: await Clipboard.source("primary").readText.call(null)
            });
        } catch (err) {
            Avdan.Debug.log(err.toString());
        }
    });