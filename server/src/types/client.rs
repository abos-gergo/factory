use log::info;
use spacetimedb::{Identity, ReducerContext, Table};

/// Representing a client connected to the server.
#[derive(Debug)]
#[spacetimedb::table(name = client)]
pub(crate) struct Client {
    #[primary_key]
    pub(crate) identity: Identity,
    /// Account the client has logged into. None until the user logs in.
    pub(crate) player_steam_id: Option<u64>,
}
impl Client {
    pub(crate) fn new(identity: Identity) -> Self {
        Self {
            identity,
            player_steam_id: None,
        }
    }
    pub(crate) fn insert_sender(ctx: &ReducerContext) -> Self {
        let c = Client::new(ctx.sender);
        info!("Inserted into Client:\n\t{:?}", c);
        ctx.db.client().insert(c)
    }
    pub(crate) fn delete_sender(ctx: &ReducerContext) -> bool {
        let deleted = ctx.db.client().identity().delete(ctx.sender);
        if deleted {
            info!("Deleted from Client:\n\t{}", ctx.sender);
        }
        deleted
    }
    pub(crate) fn get_sender(ctx: &ReducerContext) -> Self {
        ctx.db
            .client()
            .identity()
            .find(ctx.sender)
            .expect("There should be a client when calling get_sender")
    }

    pub(crate) fn update(ctx: &ReducerContext, client: Client) -> Self {
        let c = ctx.db.client().identity().update(client);
        info!("Updated Client to: \n\t {:?}", c);
        c
    }
}
