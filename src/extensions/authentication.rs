use std::sync::Arc;

use async_graphql::extensions::{Extension, ExtensionContext, ExtensionFactory, NextRequest};

pub struct Authentication;

impl ExtensionFactory for Authentication {
    fn create(&self) -> Arc<dyn Extension> {
        Arc::new(AuthenticationExtension)
    }
}

struct AuthenticationExtension;

#[async_trait::async_trait]
impl Extension for AuthenticationExtension {
    async fn request(
        &self,
        ctx: &ExtensionContext<'_>,
        next: NextRequest<'_>,
    ) -> async_graphql::Response {
        let header = warp::header::<String>("authorization");
        next.run(ctx).await
    }
}
