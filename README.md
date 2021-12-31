# BudgetMan

Yet another app to help you budget...


## Concepts

### Account 
Represents a place where you have money.
E.g. Banks, paypal, physical money, etc

#### Third party account
This is a special kind of account that can be created when you add a transaction to act as the source or the destination
E.g. a store you paid money to, a family member that gave money, etc

### Goal
A pool of money with an upper soft limit. E.g. Car: 10.000, PC: 2.000 etc

You can transfer money from an account to a goal, thus showing less money than you actually have in that account 
and hopefully helping you save money towards a... goal.

### Transactions

Represents any money moving in and out of accounts.  
Transactions have 0 or more tags, a source, a destination, and other useful info  
The source or the destination can be a goal+account pair, indicating that the money will be added or removed from that
goal as well as the account

### Tags

What kind of transaction something is. For example if you buy a game you can add a "gaming" tag, etc You can also set a
soft upper limit on how much you are allowed to spend on a certain tag per month (like a budget)

## License

BudgetMan is licensed under the AGPLv3, you can find it [here](./LICENSE)
