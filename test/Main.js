async function main() {
    let content = await Avdan.Clipboard.readText();
    Avdan.Debug.log(content);
}

main();