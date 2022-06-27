// import Clipboard from "@avdan/clipboard"; 
import TEST from "./test_module.mjs"
/**
 * Simple module export test.
 * @returns Hello world string.
 */
export function helloWorld() {
    return "Hello, World!";
}

export function goodbyeWorld() {
    return "Goodbye, World!";
}

export function returnImportedString() {
    return TEST;
}