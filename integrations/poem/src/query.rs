use async_graphql::{ObjectType, Request as GraphQLRequest, Schema, SubscriptionType};
use poem::http::StatusCode;
use poem::web::Json;
use poem::{Endpoint, Error, IntoResponse, Request, Response};

pub struct GraphQL<Query, Mutation, Subscription> {
    schema: Schema<Query, Mutation, Subscription>,
}

#[poem::async_trait]
impl<Query, Mutation, Subscription> Endpoint for GraphQL<Query, Mutation, Subscription>
where
    Query: ObjectType + 'static,
    Mutation: ObjectType + 'static,
    Subscription: SubscriptionType + 'static,
{
    async fn call(&self, mut req: Request) -> Response {
        let body = match req.take_body().into_vec().await {
            Ok(resp) => resp,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(err.to_string());
            }
        };

        let req = match serde_json::from_slice::<GraphQLRequest>(&body) {
            Ok(req) => req,
            Err(err) => {
                return Response::builder()
                    .status(StatusCode::BAD_REQUEST)
                    .body(err.to_string());
            }
        };

        Json(self.schema.execute(req).await).into_response()
    }
}
