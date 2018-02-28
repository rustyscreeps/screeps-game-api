console.error = function(...args) {
    console.log(...args);
    Game.notify(args.join(' '));
}
