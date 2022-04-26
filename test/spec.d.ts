export namespace Avdan {
    export namespace Pipe {
        type PipeContents = Uint8Array; 
        export interface PipeIn<T extends PipeContents> {}
        export interface PipeOut<T extends PipeContents> {
            pipe(destination: PipeIn<T>): Promise<void>;
        }
    }
}