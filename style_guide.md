# dn Style Guide

This document delineates the coding standards with regard to style, naming, and choice of language features in expressing ideas in code.

## Formatting

Run `cargo fmt` and let rustfmt.toml handle the details.

## Naming

On the matter of name casing, defer to the general Rust style guidelines.

When declaring multiple variables which share a significant piece of information in the name, prefer a "big-endian" approach to naming. For example:

```rust
let default_input = "foo";
let default_user_role = "user";
```

Rather than:

```rust
let input_default = "foo";
let user_role_default = "user";
```

## Interstitial Values

When binding names to expressions for clarity in defining a final binding to be used elsewhere in the program, if the interstitial values are not needed elsewhere in the program, prefer to enclose them in an expression block. For example:

```rust
let reaper_egg = {
    let boiled_egg = chicken
        .lay_eggs()
        .iter()
        .map(Egg::boil)
        .next()
        .unwrap();
    let reaper_oil = retrieve_oil(Oil::Reaper);

    boiled_egg.dip(reaper_oil)
};
```

Rather than:

```rust
let boiled_egg = chicken
    .lay_eggs()
    .iter()
    .map(Egg::boil)
    .next()
    .unwrap();
let reaper_oil = retrieve_oil(Oil::Reaper);
let reaper_egg = boiled_egg.dip(reaper_oil);
```

Doing this better expresses the scope in which the interstitial values are used, and in some casesallows for clearer naming of similar values with subtly different meanings, e.g. `config_init`, `config_altered`, and `config_final`. If the first two are contained in the assignment expression, then the final can just be called `config` without losing any clarity.

> NOTE: This is somewhat a matter of discretion - in smaller functions or expressions, this may increase visual noise for little benefit. Consider this pattern only when it will help reduce polluting the scope with unnecessary and/or confusing bindings.

## Statements & Expressions

Prefer explicit side-effects when writing statements. This means using language structures that make it clearer that the code being executed is going to have some effect. For example:

```rust
if let Some(foo) = maybe_foo {
    ollie(foo);
};

if eg_predicate {
    kickflip();
};
```

Rather than:

```rust
maybe_foo.map(ollie);

eg_predicate.then(kickflip);
```

Semicolons should always be used with statements, even in an expression scope with an implicit return. For example:

```rust
fn egg_sample(shopper: Shopper) {
    println!("{} chowed that egg properly.", shopper.name);

    shopper.eat_egg();
}
```

Rather than:

```rust
fn egg_sample(shopper: Shopper) {
    println!("{} chowed that egg properly.", shopper.name);

    shopper.eat_egg()
}
```

Conversely, expressions should always prefer to avoid semicolons and unnecessary "statement-ification", such as being used with `return`. For example:

```rust
fn get_spicy_egg() -> Egg {
    let new_egg = Egg::default();

    new_egg.dip(REAPER_OIL)
}
```

Rather than:

```rust
fn get_spicy_egg() -> Egg {
    let new_egg = Egg::default();

    return new_egg.dip(REAPER_OIL);
}
```

## Control Flow

Prefer `match` over `if` in any case where you would need an `else`. For example:

```rust
if !person.has_sampled {
    let egg = Egg::default();

    person.sample_egg(egg);
}

match person.can_ollie {
    true => congratulate(person),
    false => teach(person),
};
```

Rather than:

```rust
if !person.has_sampled {
    let egg = Egg::default();

    person.sample_egg(egg);
}

if person.can_ollie {
    congratulate(person);
} else {
    teach(person);
};
```
