#[macro_export]
macro_rules! BuildEventHandler {
    ($handler:ident,$event:ty, $handler_fn:expr) => {
        pub struct $handler;
        impl EventHandler for $handler {
            fn handle(&self, event: &mut dyn Event) {
                let event: &mut $event = <Self as EventCTX<$event>>::get_event::<$event>(event);
                self.handleCTX(event);
            }
        }
        impl EventCTX<$event> for $handler {
            fn handleCTX(&self, event: &mut $event) {
                $handler_fn(event)
            }
        }
    };
    ($handler:ident,$event:ty,$mod_ctx:expr, $handler_fn:expr) => {
        pub struct $handler {
            mod_ctx: ModCTX,
        };
        impl $handler {
            pub fn new(mod_ctx: ModCTX) -> Self {
                Self { mod_ctx }
            }
        }
        impl EventHandler for $handler {
            fn handle(&self, event: &mut dyn Event) {
                let event: &mut $event = <Self as EventCTX<$event>>::get_event::<$event>(event);
                self.handleCTX(event);
            }
        }
        impl EventCTX<$event> for $handler {
            fn handleCTX(&self, event: &mut $event) {
                let mod_ctx: &ModCTX = &self.mod_ctx;
                $handler_fn(event, mod_ctx.clone())
            }
        }
    };
}
