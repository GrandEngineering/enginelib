use std::{
    any::Any,
    sync::{Arc, RwLock},
};

use sled::Db;

use crate::{Identifier, api::EngineAPI, event::Event};

use super::{Events, ID};

#[derive(Clone, Debug)]
pub struct AdminAuthEvent {
    pub cancelled: bool,
    pub id: Identifier,
    pub payload: String,
    pub target: Identifier,
    pub db: Db,
    pub output: Arc<RwLock<bool>>,
}
#[macro_export]
macro_rules! RegisterAdminAuthEventHandler {
    ($handler:ident,$mod_ctx:ty, $handler_fn:expr) => {
        use crate::event::Event;
        use crate::event::EventCTX;
        use crate::event::EventHandler;
        use crate::events::admin_auth_event::AdminAuthEvent;
        use std::sync::Arc;
        pub struct $handler {
            mod_ctx: Arc<$mod_ctx>,
        };
        impl $handler {
            pub fn new(mod_ctx: Arc<$mod_ctx>) -> Self {
                Self { mod_ctx }
            }
        }
        impl EventHandler for $handler {
            fn handle(&self, event: &mut dyn Event) {
                let event: &mut AdminAuthEvent =
                    <Self as EventCTX<AdminAuthEvent>>::get_event::<AdminAuthEvent>(event);
                self.handleCTX(event);
            }
        }
        impl EventCTX<AdminAuthEvent> for $handler {
            fn handleCTX(&self, event: &mut AdminAuthEvent) {
                let mod_ctx: &Arc<$mod_ctx> = &self.mod_ctx;
                $handler_fn(event, mod_ctx)
            }
        }
    };
    ($handler:ident,$handler_fn:expr) => {
        use crate::event::Event;
        use crate::event::EventCTX;
        use crate::event::EventHandler;
        use crate::events::admin_auth_event::AdminAuthEvent;
        pub struct $handler;
        impl EventHandler for $handler {
            fn handle(&self, event: &mut dyn Event) {
                let event: &mut AdminAuthEvent =
                    <Self as EventCTX<AdminAuthEvent>>::get_event::<AdminAuthEvent>(event);
                self.handleCTX(event);
            }
        }
        impl EventCTX<AdminAuthEvent> for $handler {
            fn handleCTX(&self, event: &mut AdminAuthEvent) {
                $handler_fn(event)
            }
        }
    };
}
impl Events {
    pub fn CheckAdminAuth(
        api: &mut EngineAPI,
        payload: String,
        target: Identifier,
        db: Db,
    ) -> bool {
        let output = Arc::new(RwLock::new(false));
        Self::AdminAuthEvent(api, payload, target, db, output.clone());
        return *output.read().unwrap();
    }
    pub fn AdminAuthEvent(
        api: &mut EngineAPI,
        payload: String,
        target: Identifier,
        db: Db,
        output: Arc<RwLock<bool>>,
    ) {
        api.event_bus.handle(
            ID("core", "admin_auth_event"),
            &mut AdminAuthEvent {
                db,
                cancelled: false,
                id: ID("core", "admin_auth_event"),
                payload,
                output,
                target,
            },
        );
    }
}

impl Event for AdminAuthEvent {
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
