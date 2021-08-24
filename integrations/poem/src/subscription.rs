use async_graphql::{Data, ObjectType, Result, Schema, SubscriptionType};
use futures_util::future::Ready;
use futures_util::Future;
use poem::{Endpoint, Request, Response};

pub struct GraphQLSubscription<Query, Mutation, Subscription, F> {
    schema: Schema<Query, Mutation, Subscription>,
    initializer: F,
}

impl<Query, Mutation, Subscription>
    GraphQLSubscription<Query, Mutation, Subscription, fn(serde_json::Value) -> Ready<Result<Data>>>
{
    pub fn new(schema: Schema<Query, Mutation, Subscription>) -> Self {
        Self {
            schema,
            initializer: |_| futures_util::future::ready(Ok(Default::default())),
        }
    }
}

impl<Query, Mutation, Subscription, F> GraphQLSubscription<Query, Mutation, Subscription, F> {
    pub fn with_initializer<F2, R>(
        self,
        initializer: F2,
    ) -> GraphQLSubscription<Query, Mutation, Subscription, F2>
    where
        F2: FnOnce(serde_json::Value) -> R + Unpin + Send + Sync + 'static,
        R: Future<Output = Result<Data>> + Send + 'static,
    {
        GraphQLSubscription {
            schema: self.schema,
            initializer,
        }
    }
}

#[poem::async_trait]
impl<Query, Mutation, Subscription, F, R> Endpoint
    for GraphQLSubscription<Query, Mutation, Subscription, F>
where
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
    F: FnOnce(serde_json::Value) -> R + Unpin + Send + Sync + 'static,
    R: Future<Output = Result<Data>> + Send + 'static,
{
    type Output = ();

    async fn call(&self, req: Request) -> Response {
        todo!()
    }
}
