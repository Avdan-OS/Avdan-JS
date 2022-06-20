async function main() {
    const contents = await Avdan.Clipboard.readText();
    await Avdan.Clipboard.copy("Hello from Avdan.JS !");
    Avdan.Debug.log("Copied text to the clipboard !");
}

main();