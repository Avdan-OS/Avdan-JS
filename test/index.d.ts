export interface Avdan {
    Clipboard   :   Avdan.Clipboard;
    Debug       :   Avdan.Debug;
}

export namespace Avdan {

    export interface Debug {
        log(...msg : any[]) : void;
      


    }
    /**
     * Allows for easy access of the user's clipboard(s).
     * 
     * If Clipboard.source(...) is not used, the operations will be run on `clipboard` by default.
     */
    export interface Clipboard extends Clipboard.ClipboardSource {
        source(s : Clipboard.Source) : Clipboard.ClipboardSource;
    }
    export namespace Clipboard {
        export interface ClipboardOptions {
            /**
             * MIME type.
             */
            Type : string;
        }
        
        /**
         * Which clipboard source? [See SuperUser post](https://superuser.com/questions/90257/what-is-the-difference-between-the-x-clipboards)
         * 
         * It is not advisable to use `secondary`.
         */
        export type Source = "primary" | "secondary" | "clipboard";

        interface ClipboardSource {
            /**
             * Copies text to the clipboard.
             * @inherits Raycast
             * @param text The text to copy to the clipboard.
             * @returns A Promise that resolves when the text is copied to the clipboard.
             */
            copy(text : string, Options?: Clipboard.ClipboardOptions) : Promise<void>;
    
            /**
             * Pastes text to the current selection of the frontmost application.
             * @inherits Raycast
             * @param text The text to insert at the cursor.
             * @param delayMs How long to wait for before inserting the text in ms. (Default: 250ms)
             * @returns A Promise that resolves when the text is pasted.
             */
            paste(text : string, delayMs?: number) : Promise<void>;
    
            /**
             * Clears the current clipboard contents.
             * @inherits Raycast
             * @returns A Promise that resolves when the clipboard is cleared.
             */
            clear(): Promise<void>;
    
            /**
             * Reads the current clipboard and negotiates the MIME format with the host application.
             * 
             * If no desired format is found, or the clipboard is empty, this will throw the respective exception.
             * @param PrimaryType The ideal MIME format.
             * @param Formats A set of acceptable MIME formats in order of priority.
             * @returns An array containing the MIME type and the data itself (encoded as a utf8 string).
             */
            read(PrimaryType : string, ...Formats : string[]) : Promise<[string, string]>;
    
            /**
             * Reads the clipboard as plain text.
             * 
             * An alias for `ClipboardSource.read("text/plain")`
             * 
             * Mainly for Raycast compatibility.
             * 
             * @returns A promise that resolves when the clipboard content was read as plain text.
             */
            readText(): Promise<string | undefined>;

            /**
             * Returns the types available to read from clipboard. 
             */
            formats(): Promise<string[]>;
        }
    }
}

declare global {
    var Avdan : Avdan;
}
export {}