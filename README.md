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
