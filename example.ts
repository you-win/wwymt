/**
 * Shows a help message with all possible commands.
 */
function showHelpMessage(): void {
    // TODO deno doesn't like newlines
    //     return `
    // help  - Show this message
    // hello - Say hello
    // quit  - Quit the game
    //     `.trim()
    console.log("help  - Show this message");
    console.log("hello - Say hello");
    console.log("quit  - Quit the game");
}

/**
 * It's main idk.
 */
async function main(): Promise<void> {
    Deno.core.initializeAsyncOps();

    console.log("hello!");
    console.error("this is console.error!")
    log.info("This is an info log!");
    log.debug("This is a debug log?");
    log.warn("THIS IS A WARNING");
    log.error("This is an error log.");

    console.log(await rust.ping());
    console.log(rust.sum([1, 2, 3, 4]));

    [1, 2, 3].forEach((v: number) => console.log(v));

    let response: string = await rust.get("https://api.github.com/users/you-win", [
        "Accept: */*",
        "User-Agent: you-win#github"
    ]);

    let data: any = JSON.parse(response);

    console.log(data)
    console.log(data.login, data.id, data.url);

    while (true) {
        console.log("Please input a command: ");
        let input: string = await rust.readInput();

        switch (input.toLowerCase()) {
            case "help":
                showHelpMessage();
                break;
            case "hello":
                console.log("Hello, world!");
                break;
            case "quit":
                console.log("Quitting, おやすみなさい！");
                return;
            default:
                console.log("Unrecognized input: ", input);
                break;
        }
    }
}

main()
