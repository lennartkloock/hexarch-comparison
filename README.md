# Hexarch vs. Plain Old Functions

This repo compares Hexarch to using plain old functions.

## The example

The example is a banking software that stores multiple users which can have multiple accounts.

New users should get a new account with a start balance of 10.

Every account has a random account address.

The HTTP handler should do the following things:

- Check if the given username isn't used by any other user yet
- Check if the given email address isn't used by any other user yet
- Create a new user
- Generate a new random account address
- Create the new account with a balance of 10

## Evalution

### Boilerplate

The functional approach has a lot less code which speeds up development and maintenance as well as making the code more readable.

### Onboarding

Onboarding is much easier with the functional approach since new developers don't have to learn a new architecture.
Everyone knows what a function is.

### Project file structure

The project file structure is equally easy to navigate in both implementations.
Business logic can be extracted into their own separated modules with both approaches.

### Testability

The code that can't be tested is mostly the same in both architectures.

In the Hexarch implementation we can't test the `Postgres` and `RabbitMq` adapters.

In the functional approach we can't test the `postgres` and `rmq` modules.

The HTTP handler can only be tested in the Hexarch implementation but it doesn't contain any business logic.

**All the business logic can be tested in both implementations.**

## My conclusion

Plain functions are better.
