async function main() {
    try {
        Avdan.Debug.fetch("https://google.com").then(txt => Avdan.Debug.log({length: txt.length}));

        Avdan.Debug.log("Waiting for result!");

    } catch (e) {
        Avdan.Debug.log(e.toString());
    }
}
main();