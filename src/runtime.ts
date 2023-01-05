((globalThis: any) => {
    const core: Core = Deno.core;
    const ops: Ops = core.ops;

    function argsToMessage(...args: any[]): string {
        return args.map(arg => JSON.stringify(arg)).join(" ");
    }

    globalThis.console = {
        log: (...args: any[]): void => {
            core.print(`${argsToMessage(...args)}\n`, false);
        },
        error: (...args: any[]): void => {
            core.print(`${argsToMessage(...args)}\n`, true);
        }
    };

    globalThis.rust = {
        ping: async (): Promise<string> => {
            return await ops.ping();
        },
        sum: (arr: any[]): number => {
            return ops.sum(arr);
        },
        get: async (url: string, headers: string[]): Promise<string> => {
            return await ops.get(url, headers);
        },
        readInput: async (): Promise<string> => {
            return await ops.read_input();
        }
    }

    globalThis.log = {
        info: (...args: any[]): void => {
            ops.info(`${argsToMessage(...args)}`);
        },
        debug: (...args: any[]): void => {
            ops.debug(`${argsToMessage(...args)}`);
        },
        warn: (...args: any[]): void => {
            ops.warn(`${argsToMessage(...args)}`);
        },
        error: (...args: any[]): void => {
            ops.error(`${argsToMessage(...args)}`);
        }
    }
})(globalThis);
