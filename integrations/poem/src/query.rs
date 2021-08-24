use async_graphql::{
    BatchResponse as GraphQLBatchResponse, ObjectType, Request as GraphQLRequest, Schema,
    SubscriptionType,
};
use poem::http::StatusCode;
use poem::web::Json;
use poem::{async_trait, Endpoint, Error, FromRequest, IntoResponse, Request, Response, Result};

use crate::GraphQLBatchRequest;

pub struct GraphQL<Query, Mutation, Subscription> {
    schema: Schema<Query, Mutation, Subscription>,
}

#[async_trait]
impl<Query, Mutation, Subscription> Endpoint for GraphQL<Query, Mutation, Subscription>
where
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
{
    type Output = Result<Json<GraphQLBatchResponse>>;

    async fn call(&self, mut req: Request) -> Self::Output {
        let (req, mut body) = req.split();
        let req = GraphQLBatchRequest::from_request(&req, &mut body).await?;
        Ok(Json(self.schema.execute_batch(req.0).await))
    }
}
