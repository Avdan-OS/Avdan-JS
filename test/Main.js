async function main() {
    try {
        Avdan.Debug.wait(10, 100)
            .on("tick", tick => {
                Avdan.Debug.log(`Got tick:`, tick.tick);
            })
            .on("tick", tick => {
                Avdan.Debug.log(`Got overridden tick!`, tick.tick);
            })
            .then(() => Avdan.Debug.log(`Finished waiting!`));

        Avdan.Debug.log("Waiting for result!");

    } catch (e) {
        Avdan.Debug.log(e.toString());
    }
}
main();