async function main() {
    try {
        Avdan.Debug.wait(1_500).then(() => Avdan.Debug.log("Finished first timeout!"));
        Avdan.Debug.wait(500).then(() => Avdan.Debug.log("Finished second timeout!"));


        // Copy "Hello World!" to the clipboard
        Avdan.Debug.log({
            status: {
                done: true,
                text: {
                    en: "Finished!",
                    it: "Finito!",
                }
            }
        });
    } catch (e) {
        Avdan.Debug.log(e.toString());
    }
}
main();