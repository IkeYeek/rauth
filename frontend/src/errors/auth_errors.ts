export class BadCreditentials extends Error {
  constructor() {
    super("bad creditentials");
  }
}

export class NotAuthenticated extends Error {
  constructor() {
    super("not authenticated");
  }
}

export class NotAuthorized extends Error {
  constructor() {
    super("not authorized");
  }
}
