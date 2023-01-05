/**
 * Core Deno types.
 */
declare var Deno: {
    core: Core;
}
/**
 * Core Deno utilities.
 */
type Core = any;
/**
 * Custom operations defined in Rust.
 */
type Ops = any;

/**
 * Catch-all for Rust functions.
 */
declare var rust: {
    /**
     * Returns pong.
     */
    ping(): Promise<string>;
    /**
     * Adds an array of numbers.
     * @param arr The numbers to add up.
     */
    sum(arr: any[]): number;
    /**
     * Send a GET request to the given url with the given headers.
     * @param url The url to send the request to.
     * @param headers The headers to send with the request.
     */
    get(url: string, headers: string[]): Promise<string>;
    /**
     * Prompt the user for input.
     */
    readInput(): Promise<string>;
}

/**
 * Logging-related functions.
 */
declare var log: {
    /**
     * Log a message at the info level.
     * @param text The message to log.
     */
    info(text: string): void;
    /**
     * Log a message at the debug level.
     * @param text The message to log.
     */
    debug(text: string): void;
    /**
     * Log a message at the warn level.
     * @param text The message to log.
     */
    warn(text: string): void;
    /**
     * Log a message at the error level.
     * @param text The message to log.
     */
    error(text: string): void;
}
