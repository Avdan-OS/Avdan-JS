async function main() {
    try {
        // Copy "Hello World!" to the clipboard
        Avdan.Clipboard.copy("Hello World!");
        Avdan.Debug.log("Done!")
    } catch (e) {
        Avdan.Debug.log(e.toString());
    }
}
main();