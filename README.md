# Citadel-crud
Citadel-crud will improve your app's relationship with its Database. Normally, database access leverages a flexible ORM-like system to do whatever you need at the time.

This can muddle the purpose of what your database code is doing. "Add Person" may create a person, and then return them. It's doing creates as well as reads.

Citadel lets you instantiate CRUD components that can work independantly and asynchonously of one another.

### Creators
In charge of creating items in the database.
### Readers
In charge of instantiating other objects from the database
### Updaters
In charge of changing items in the database
### Deleters
In charge of deleting items in the database
## System
In charge of abstractying database functions for cross-DBMS compatibility

# Planned Features
### Component composition
(i.e binding creators / updators to successfully run)

### Helpful macros
Better usages of DatabaseConnection, etc

### DBMS-agnostic DatabaseConnection
### Diesel-agnostic database interaction
### ORM-esque object filling
### Local and global caching
### UNIT TESTS!
