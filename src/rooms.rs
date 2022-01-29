#![no_std]

elrond_wasm::imports!();

// NOTE: max value for u32 is 4_294_967_295
// use bigger variable types for more rooms /matches

#[elrond_wasm::derive::contract]
pub trait Room {
    // storage
    #[view(getMatchesSize)]
    #[storage_mapper("match_size")]
    // THIS IS A CONSTANT, it should never be modified during run time
    fn match_size(&self) -> SingleValueMapper<u32>;

    #[view(getWaitingRoom)]
    #[storage_mapper("waiting_room")]
    fn waiting_room(&self) -> LinkedListMapper<ManagedAddress>;

    #[view(getMatchesCount)]
    #[storage_mapper("matches_count")]
    fn matches_count(&self) -> SingleValueMapper<u32>;

    #[view(getMatchesMap)]
    #[storage_mapper("matches_map")]
    fn matches_map(&self, address: ManagedAddress) -> SingleValueMapper<u32>;


    #[view(getWaitingRoomLength)]
    fn waiting_room_length(&self) -> u32 {
        self.waiting_room().len() as u32
    }

    #[init]
    fn init(&self, match_size: u32) {
        self.match_size().set(match_size);
    }

    #[endpoint]
    fn wait(&self) -> LinkedListMapper<ManagedAddress>{
        let caller = self.blockchain().get_caller();

        if self.waiting_room().len() != 0 {
            require!(self.waiting_room().back().unwrap().get_value_cloned() != caller, "This address is already in the wait list");
        }

        self.waiting_room().push_back(caller);

        let match_size:u32 = self.match_size().get();
        let waiting_room_size:u32 = self.waiting_room().len() as u32;

        if waiting_room_size % match_size == 0  {
            self.match_users()
        }

        self.waiting_room()
    }

    fn match_users(&self) {
        let match_size:u32 = self.match_size().get();
        let waiting_room_size:u32 = self.waiting_room().len() as u32;

        require!(waiting_room_size % match_size == 0, "Match size does not support the current number of people waiting");

        self.matches_count().set(self.matches_count().get() as u32 + 1);
        let user_one = self.waiting_room().pop_front().unwrap().get_value_cloned();
        let user_two = self.waiting_room().pop_front().unwrap().get_value_cloned();
        self.matches_map(user_one).set(self.matches_count().get());
        self.matches_map(user_two).set(self.matches_count().get());
    }
}
