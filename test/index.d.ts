
export interface Avdan {
    Clipboard   :   Avdan.Clipboard;
    Debug       :   Avdan.Debug;
    Environment :   Avdan.Environment;
    LocalStorage:   Avdan.Storage.LocalStorage;
    File        :   Avdan.File;


    /**
     * @inherit Raycast 
     * Creates and shows a confirmation Alert with the given options.
     * 
     * @param options The options used to create the Alert.
     * @return A Promise that resolves to a boolean when the user triggers one of the actions. It will be `true` for the primary Action, `false` for the dismiss Action.
     */
    confirmAlert(options: Avdan.Feedback.Alert.Options) : Promise<boolean>;

    /**
     * @inherit Raycast
     * A HUD will automatically hide the main window and show a compact message at the bottom of the screen.
     * @param title The title that will be displayed in the HUD.
     * @return A Promise that resolves when the HUD is shown.
     */
    showHUD(title: string) : Promise<void>;

    /**
     * @inherit Raycast
     * Creates and shows a Toast with the given options.
     * @param options The options used to create the Toast.
     * @return A Promise that resolves with the shown Toast. The Toast can be used to change or hide it.
     */
    showToast(options : Avdan.Feedback.Toast.Options) : Promise<Avdan.Feedback.Toast>

    /**
     * A function to access the preference values that have been passed to the command.
     * 
     * Each preference name is mapped to its value, and the defined default values are used as fallback values.
     * 
     * @return An object with the preference names as property key and the typed value as property value.
     * 
     * See [Raycast Docs](https://developers.raycast.com/api-reference/preferences).
     */
    getPreferenceValues<T extends {[preferenceName : string] : any}>() : T;

    /**
     * Returns all applications that can open the file.
     * @param path The path of the file or folder to get the applications for. If no path is specified, all installed applications are returned.
     * @return An array of {@link Avdan.Utils.Application}.
     */
    getApplications(path?: Avdan.Utils.PathLike) : Promise<Avdan.Utils.Application[]>;

    /**
     * Returns the default application that the file would be opened with.
     * @param path The path of the file or folder to get the default application for.
     * @return The default Application that would open the file. Throws an error if no application was found.
     */
    getDefaultApplication(path : Avdan.Utils.PathLike) : Promise<Avdan.Utils.Application>;

    /**
     * Shows a file or directory in the Finder.
     * @param path The path to show in the Finder.
     * @return A Promise that resolves when the item is revealed in the Finder.
     */
    showInFinder(path: Avdan.Utils.PathLike) : Promise<void>;

    /**
     * Moves a file or directory to the Trash.
     * @param path 
     * @return A Promise that resolves when all files are moved to the trash.
     */
    trash(path : Avdan.Utils.PathLike | Avdan.Utils.PathLike[]) : Promise<void>;

    /**
     * Opens a target with the default application or specified application.
     * @param target The file, folder or URL to open
     * @param application The application name to use for opening the file. If no application is specified, the default application as determined by the system is used to open the specified file. Note that you can use the application name, app identifier, or absolute path to the app.
     * @return A Promise that resolves when the target has been opened.
     */
    open(target : string, application ?: Avdan.Utils.Application | string) : Promise<void>;

    /**
     * Clear the text in the search bar.
     * @param options Can be used to force scrolling to the top. Defaults to scrolling to the top after the search bar was cleared.
     * @return A Promise that resolves when the search bar is cleared.
     */
    clearSearchBar(options ?: {forceScrollToTop : boolean}) : Promise<void>;

    /**
     * Closes the main Raycast window.
     * @param options Can be used to clear the root search. Defaults to not clearing the root search after the window was closed.
     * @return A Promise that resolves when the main window is closed.
     */
    closeMainWindow(options ?: {clearRootSearch: boolean}) : Promise<void>;

    /**
     * Pops the navigation stack back to root search.
     * @param options Can be used to clear the search bar. Defaults to clearing the search bar after popped to root.
     * @return A Promise that resolves when Raycast popped to root.
     */
    popToRoot(options ?: {clearSearchBar: boolean}) : Promise<void>;
}

/**
 * Global object for use in developing search extensions/plugins.
 * 
 * Ideally should be compatible with the existing Raycast extension API, which is documented here:
 * [Raycast API Reference](https://github.com/raycast/extensions/blob/main/docs/SUMMARY.md#api-reference).
 * 
 * Perhaps, even implement an entire React Reconciler as a compatability layer?
 */
export namespace Avdan {

    export interface Debug {
        log(...msg : any[]) : void;
      


    }

    export interface Shell {}

    export namespace Shell {
        
    }

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

    
    export namespace Environment {
        /**
         * @inherits Raycast
         * 
         * Holds data about a File System item. Use the {@link Environment.getSelectedFinderItems} method to retrieve values.
         */
        interface FileSystemItem {
            /**
             * The path to the item.
             */
            path : string;
        }
    }

    export interface Environment {
        /**
         * @inherits Raycast
         * 
         * The absolute path to the assets directory of the extension.
         */
        assetsPath : string;

        /**
         * @inherits Raycast
         * 
         * The name of the launched command, as specified in package.json
         */
        commandName : string;

        /**
         * @inherits Raycast
         * 
         * The name of the extension, as specified in package.json
         */
        extensionName : string;

        /**
         * @inherits Raycast
         * 
         * Indicates whether the command is a development command (vs. an installed command from the Store).
         */

        isDevelopment : boolean;
        /**
         * @inherits Raycast
         * 
         * The version of the main Raycast app
         */

        raycastVersion : string;
        /**
         * @inherits Raycast
         * 
         * The absolute path for the support directory of an extension. Use it to read and write files related to your extension or command.
         */
        supportPath : string;
        
        /**
         * @inherits Raycast
         * 
         * Gets the selected items from Finder.
         * 
         * @return A Promise that resolves with the selected file system items.
         */
        getSelectedFinderItems() : Promise<Environment.FileSystemItem[]>;
        
        /**
         * @inherits Raycast
         * 
         * Gets the selected text of the frontmost application.
         */
        getSelectedText() : Promise<string>;

    }

    export namespace Feedback { 
        export namespace Alert {
            export interface Options {
                /**
                 * The title of an alert. Displayed below the icon.
                */
                title: string;

               /**
                * The icon of an alert to illustrate the action. Displayed on the top.
               */
                icon?: ImageLike;

               /**
                * An additional message for an Alert. Useful to show more information, e.g. a confirmation message for a destructive action.
               */
                message?: string;

               /**
                * The primary Action the user can take.
               */
                primaryAction?: Alert.ActionOptions;

                /**
                 * The Action to dismiss the alert. There usually shouldn't be any side effects when the user takes this action.
                 */
                dismissAction?: Alert.ActionOptions;
            }

            export interface ActionOptions {
                /**
                 * The title of the action.
                 */
                title : string;

                /**
                 * The style of the action.
                 */
                style : string;

                /**
                 * A callback called when the action is triggered.
                 */
                onAction() : void;
            }

            export enum ActionStyle {
                Default,
                Destructive,
                Cancel,
            }
        }

        export interface Toast {
            /**
             * The title of a Toast. Displayed on the top.
             */
            title: string;
            
            /**
             * An additional message for the Toast. Useful to show more information, e.g. an identifier of a newly created asset.
             */
            message?:string;

            /**
             * The style of a Toast.
             */
            style: Toast.Style;
            
            /**
             * The primary Action the user can take when hovering on the Toast.
             */
            primaryAction ?: Toast.ActionOptions;
           
            /**
             * The secondary Action the user can take when hovering on the Toast.
             */                
            secondaryAction?: Toast.ActionOptions;

            hide() : Promise<void>;

            show() : Promise<void>;
        }
        export namespace Toast {

            export interface Options {
                title: string;
                style?: Toast.Style;
                message?: string;
                primaryAction?: Toast.ActionOptions;
                secondaryAction?: Toast.ActionOptions;
            }

            export enum Style {
                Animated,
                Success,
                Failure
            }

            export interface ActionOptions {
                title : string;
                onAction : (toast: Toast) => void;
                shortcut?: Keyboard.Shortcut;
            }
        }

        /**
         * @inherit Raycast
         */
        export namespace Keyboard {
            export type KeyEquivalent = "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n" | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "." | "," | ";" | "=" | "+" | "-" | "[" | "]" | "{" | "}" | "«" | "»" | "(" | ")" | "/" | "\\" | "'" | "`" | "§" | "^" | "@" | "$" | "return" | "delete" | "deleteForward" | "tab" | "arrowUp" | "arrowDown" | "arrowLeft" | "arrowRight" | "pageUp" | "pageDown" | "home" | "end" | "space" | "escape" | "enter" | "backspace";
            export type KeyModifier = "cmd" | "ctrl" | "opt" | "shift" | "meta";
            export interface Shortcut {
                key : KeyEquivalent;
                modifiers : KeyModifier[];
            }
        }
    }

    export namespace Storage {
       export type Value = string | number | boolean | null;
       export interface LocalStorage {
           /**
            * Retrieve all stored values in the local storage of an extension.
            * 
            * @return A Promise that resolves with an object containing all Values.
            */
            allItems<V extends Value>() : Promise<V>;

            /**
             * Removes all stored values of an extension.
             * @return A Promise that resolves when all values are removed.
             */
            clear() : Promise<void>;

            /**
             * Retrieve the stored value for the given key.
             * @param key The key you want to retrieve the value of.
             * @return A Promise that resolves with the stored value for the given key. If the key does not exist, undefined is returned.
             */
            getItem<V extends Value>(key : string) : Promise<V>;

            /**
             * Removes the stored value for the given key.
             * @param key TThe key you want to remove.he key you want to remove.
             * @return A Promise that resolves when the value is removed.
             */
            removeItem(key : string) : Promise<void>;

            /**
             * Stores a value for the given key.
             * @param key The key you want to create or update the value of.
             * @param value The value you want to create or update for the given key.
             * @return A Promise that resolves when the value is stored.
             */
            setItem<V extends Value>(key : string, value : V) : Promise<void>;
       } 
    }

    export namespace Utils {
        export interface Application {
            /**
             * The bundle identifier of the application, e.g. com.raycast.macos.
             */
            bundleId ?: string;

            /**
             * The display name of the application.
             */
            name : string;

            /**
             * The display name of the application. e.g. `/Applications/Raycast.app`
             */
            path : string;
        }

        export type URL = string;
        export type PathLike = string | URL;
    }


}

declare global {
    var Avdan : Avdan;
}
export {}