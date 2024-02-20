# schmake

Save time creating your SQL schema with less keystrokes and the speed of Rust.

> [!NOTE]  
> Schmake currently only supports PostgreSQL

### Usage

Create your first `.schm` file:

```
touch users.schm
```

Write the schema for your `users` table:

```schm
table users
id serial pk
name varchar
email varchar unique
```

Run `schmake` on that file:

```
schmake --file ./users.schm
```

And magic, you now have the definition for your users table

### Constraints

Schmake provides a number of aliases for common constraints

- `PRIMARY KEY`
  - `pk`
- `NOT NULL`
  - `nn`
  - `not null`
