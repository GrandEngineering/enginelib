use std::{
    any::Any,
    sync::{Arc, RwLock},
};

use sled::Db;

use crate::{Identifier, api::EngineAPI, event::Event};

use super::{Events, ID};

#[derive(Clone, Debug)]
pub struct AuthEvent {
    pub cancelled: bool,
    pub id: Identifier,
    pub uid: String,
    pub challenge: String,
    pub db: Db,
    pub output: Arc<RwLock<bool>>,
}
#[macro_export]
macro_rules! RegisterAuthEventHandler {
    ($handler:ident,$handler_fn:expr) => {
        use crate::event::Event;
        use crate::event::EventCTX;
        use crate::event::EventHandler;
        use crate::events::auth_event::AuthEvent;
        pub struct $handler;
        impl EventHandler for $handler {
            fn handle(&self, event: &mut dyn Event) {
                let event: &mut AuthEvent =
                    <Self as EventCTX<AuthEvent>>::get_event::<AuthEvent>(event);
                self.handleCTX(event);
            }
        }
        impl EventCTX<AuthEvent> for $handler {
            fn handleCTX(&self, event: &mut AuthEvent) {
                $handler_fn(event)
            }
        }
    };
}
impl Events {
    pub fn CheckAuth(api: &mut EngineAPI, uid: String, challenge: String, db: Db) -> bool {
        let output = Arc::new(RwLock::new(false));
        Self::AuthEvent(api, uid, challenge, db, output.clone());
        return *output.read().unwrap();
    }
    pub fn AuthEvent(
        api: &mut EngineAPI,
        uid: String,
        challenge: String,
        db: Db,
        output: Arc<RwLock<bool>>,
    ) {
        api.event_bus.handle(
            ID("core", "auth_event"),
            &mut AuthEvent {
                db,
                cancelled: false,
                id: ID("core", "auth_event"),
                uid,
                challenge,
                output,
            },
        );
    }
}

impl Event for AuthEvent {
    fn clone_box(&self) -> Box<dyn Event> {
        Box::new(self.clone())
    }

    fn cancel(&mut self) {
        self.cancelled = true;
    }
    fn is_cancelled(&self) -> bool {
        self.cancelled
    }
    fn get_id(&self) -> Identifier {
        self.id.clone()
    }
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
