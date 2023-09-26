# Postgres

This project uses [sqlx](https://docs.rs/sqlx/latest/sqlx/) as it's postgres database client.

## Migrations

ChatGPT says...

```
A database migration is a set of procedures that:

1. Modifies the Schema or Data: It alters the structure (schema) of the database, such as adding or removing tables, columns, indexes, or constraints. It can also transform, add, or remove data within the database.

2. Is Versioned: Migrations are typically versioned, allowing developers to apply them in a specific order. This ensures that the database evolves in a consistent and predictable manner across different environments (e.g., development, staging, production).

3. Allows Forward and Backward Changes: Migrations usually come in pairs:

Up: This moves the database forward. It might create a table, add a column, etc.

Down: This undoes what the up migration did, allowing the system to revert to a previous state. It might drop a table, remove a column, etc.
Is Automated: Migrations are typically run using automated tools or scripts, ensuring that they are applied consistently and reducing the potential for human error.

4. Ensures Consistency: Migrations provide a way to keep the database schema consistent across multiple instances of the same application. Whether it's another developer's machine, a testing environment, or a production server, migrations help ensure everyone is working with the same database structure.

5. Keeps a Record: Most migration systems track which migrations have been applied (often in a dedicated database table). This allows the system to determine which migrations need to be run in the future.

In essence, a database migration manages and tracks changes to the database's structure and data, ensuring that changes are applied in a consistent, controlled, and versioned manner.
```

### Creating a Migration

Use `sqlx` to add a migration:

```
sqlx migrate add <migration-name>
```

Note: make sure that migrations happen within the associated backend directory. For example, migrations for `account-db` should occur in the `accounts-backend` directory.

## Prepared Queries

See [Offline Mode in sqlx](https://docs.rs/sqlx/latest/sqlx/macro.query.html#offline-mode-requires-the-offline-feature)

```
cargo sqlx prepare --database-url postgres://myuser:mypassword@localhost/mydatabase --workspace
```

Keep the generated `.json` file in the root of the workspace:
> query data written to .sqlx in the workspace root; please check this into version control