class Base64 {}
Base64.ALPHABET = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/"; 
/** @param {Uint8ClampedArray} arr */
Base64.encode = function(arr) {
    let toPad = "=".repeat(3 - arr.length % 3);
};

(async () => {
    {
        const Clip =  Avdan.Clipboard.source("clipboard");
        const formats = await Clip.formats();
        Avdan.Debug.log(formats);

        const [mime, rawContents] = await Clip.readRaw("text/plain");



        Avdan.Debug.log({mime, length : rawContents?.length, });

        // Avdan.Debug.log(String.fromCharCode(...rawContents))
        // Avdan.Debug.log({text})
        // Avdan.Debug.log({Clip});
        // const [type, content] = await Avdan.Clipboard.read("image/png");
        // Avdan.Debug.log({type, content});
    }
})();