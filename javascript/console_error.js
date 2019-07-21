function console_error(...args) {
    console.log(...args);
    Game.notify(args.join(' '));
}
