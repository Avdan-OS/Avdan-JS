declare module '@avdan' {
    export interface Task<R, E extends Task.Event> extends Promise<R> {
        on<N extends keyof E>(name : N, callback:E[N]) : Task<E>;
    }   
    export namespace Task {
    
        export interface Event {
            [name: string] : (...args : any[]) => void;
        }
    
        export namespace Event {
            export namespace Type {
                export interface Progressable extends Event { "progress": (percent : number, [completed, total] : [number, number]) => void }
                export interface Milestone<K extends string[]> extends Event { "milestone" : (stage : K[number]) => void }
            }
        }
    }
}

/**
 * Avdan Clipboard API 
 */
declare module '@avdan/clipboard' {
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
             * Copies raw bytes into the clipboard.
             * @param type The MIME type of the data.
             * @param data The data to copy to the clipboard.
             * @param delayMs 
             */
            copyRaw(type : string, data : Uint8Array, delayMs ?: number) : Promise<void>;
    
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
             * @returns A tuple containing the MIME type and the data itself (encoded as a utf8 string).
             */
            read<T extends string>(PrimaryType : T, ...Formats : T[]) : Promise<[T, string]>;

            /**
             * Reads the current clipboard and negotiates the MIME format with the host application.
             * @param PrimaryType The ideal MIME format.
             * @param Formats A set of other acceptable MIME formats in order of priority.
             * @returns A tuple containing the MIME type and the data itself (raw bytes).
             */
            readRaw<T extends string>(PrimaryType : T, ...Formats : T[]) : Promise<[T, Uint8Array] | [ undefined, undefined]>;
    
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

    let d : Clipboard;

    export default d;
}

declare module '@avdan/file' {
    export interface File {
        write(path: string, data: string | Uint8Array) : Promise<void>;
        read<K extends keyof File.ReadMap, V extends File.ReadMap[K]>(path : string, format:K) : Promise<V>;
    }

    export namespace File {
        export interface ReadMap {
            "bytes" : Uint8Array;
            "utf8"  : string;
        }
    }

    const api : File;
    export default api;
}

declare module '@avdan/debug' {
    import {Task} from "@avdan";
    export interface Debug {
        /**
         * Logs message into console.
         * @param msg Message
         */
        log(...msg : any[]) : void;
        
        /** 
         * Waits for a specific timeout for a specified number of times (ticks). 
         * 
         * @param ticks Number of total ticks.
         * @param time Time of each tick in miliseconds.
        */
        wait(ticks: number, time: number): Task<void, {
            "tick": (tick: {tick : number}) => void
        }>;
    }

    const contents : Debug;
    export default contents;
}

declare module '*.json' {
    const contents: any;
    export default contents;
}

