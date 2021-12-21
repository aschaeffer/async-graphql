Define a GraphQL input object

*[See also the Book](https://async-graphql.github.io/async-graphql/en/define_input_object.html).*

# Macro attributes

| Attribute     | description               | Type     | Optional |
|---------------|---------------------------|----------|----------|
| name          | Object name               | string   | Y        |
| rename_fields | Rename all the fields according to the given case convention. The possible values are "lowercase", "UPPERCASE", "PascalCase", "camelCase", "snake_case", "SCREAMING_SNAKE_CASE".| string   | Y        |
| visible       | If `false`, it will not be displayed in introspection. *[See also the Book](https://async-graphql.github.io/async-graphql/en/visibility.html).* | bool | Y |
| visible       | Call the specified function. If the return value is `false`, it will not be displayed in introspection. | string | Y |

# Field attributes

| Attribute    | description                              | Type        | Optional |
|--------------|------------------------------------------|-------------|----------|
| name         | Field name                               | string      | Y        |
| default      | Use `Default::default` for default value | none        | Y        |
| default      | Argument default value                   | literal     | Y        |
| default_with | Expression to generate default value     | code string | Y        |
| validator    | Input value validator *[See also the Book](https://async-graphql.github.io/async-graphql/en/input_value_validators.html)*                   | object | Y        |
| flatten      | Similar to serde (flatten)               | boolean     | Y        |
| skip         | Skip this field, use `Default::default` to get a default value for this field. | bool     | Y        |
| visible      | If `false`, it will not be displayed in introspection. *[See also the Book](https://async-graphql.github.io/async-graphql/en/visibility.html).* | bool | Y |
| visible      | Call the specified function. If the return value is `false`, it will not be displayed in introspection. | string | Y |
| secret       | Mark this field as a secret, it will not output the actual value in the log. | bool | Y |

# Examples

```rust
use async_graphql::*;

#[derive(InputObject)]
struct MyInputObject {
    a: i32,
    #[graphql(default = 10)]
    b: i32,
}

struct Query;

#[Object]
impl Query {
    /// value
    async fn value(&self, input: MyInputObject) -> i32 {
        input.a * input.b
    }
}

# tokio::runtime::Runtime::new().unwrap().block_on(async move {
let schema = Schema::new(Query, EmptyMutation, EmptySubscription);
let res = schema.execute(r#"
{
    value1: value(input:{a:9, b:3})
    value2: value(input:{a:9})
}"#).await.into_result().unwrap().data;
assert_eq!(res, value!({ "value1": 27, "value2": 90 }));
# });
```
