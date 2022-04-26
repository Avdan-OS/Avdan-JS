async function main() {
    try {
        let content = await Avdan.File.read("test/Main.js", "utf8");
        Avdan.Debug.log("My own source code:");
        Avdan.Debug.log(content);
    } catch(e) {
        Avdan.Debug.log(e.toString());
    }
}
main();