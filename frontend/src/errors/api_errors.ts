export class ApiError extends Error {
    constructor() {
        super("Api error");
    }
}

export class ApiUsage extends Error {
    constructor() {
        super("you either tried to play too hard or i suck at programming lol either way rip bozo");
    }
}