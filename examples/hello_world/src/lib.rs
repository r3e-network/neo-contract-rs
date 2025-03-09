#![no_std]

#[neo_contract::contract]
mod hello_world {
    use neo_contract::prelude::*;

    #[storage]
    struct HelloWorld {
        message: Item<String>,
    }

    impl HelloWorld {
        #[constructor]
        fn new(message: String) -> Self {
            Self {
                message: Item::new(message),
            }
        }

        #[method]
        fn set_message(&mut self, message: String) {
            self.message.set(message);
        }

        #[safe]
        fn get_message(&self) -> String {
            self.message.get().clone()
        }

        #[safe]
        fn hello(&self, name: String) -> String {
            format!("{} {}", self.message.get(), name)
        }
    }
}
