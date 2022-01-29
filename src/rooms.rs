#![no_std]

elrond_wasm::imports!();

// NOTE: max value for u32 is 4_294_967_295
// use bigger variable types for more rooms /matches

#[elrond_wasm::derive::contract]
pub trait Room {
    // ***** STORAGE ***** //

    // ***** STORAGE WITH VIEWS  ***** //
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

    #[view(getAddressToMatchIdMap)]
    #[storage_mapper("address_to_match_id_map")]
    fn address_to_match_id_map(&self, address: ManagedAddress) -> SingleValueMapper<u32>;

    #[storage_mapper("match_id_to_address_map")]
    #[view(getMatchIdToAddressMap)]
    fn match_id_to_address_map(&self, match_id: u32) -> SingleValueMapper<Vec<ManagedAddress>>;


    // ***** VIEWS  ***** //
    #[view(getWaitingRoomLength)]
    fn waiting_room_length(&self) -> u32 {
        self.waiting_room().len() as u32
    }

    // ***** ENDPOINTS  ***** //
    #[endpoint]
    fn wait(&self) -> LinkedListMapper<ManagedAddress> {
        let caller = self.blockchain().get_caller();

        if self.waiting_room().len() != 0 {
            require!(self.waiting_room().back().unwrap().get_value_cloned() != caller, "This address is already in the wait list");
        }

        self.waiting_room().push_back(caller);

        let match_size: u32 = self.match_size().get();
        let waiting_room_size: u32 = self.waiting_room().len() as u32;

        if waiting_room_size % match_size == 0 {
            self.match_users()
        }

        self.waiting_room()
    }

    #[endpoint(leaveMatch)]
    fn leave_match(&self) {
        let caller = self.blockchain().get_caller();
        let caller_clone = caller.clone();

        let match_id = self.address_to_match_id_map(caller);

        require!(!match_id.is_empty(), "This address who called this function is not in a match");
        let match_id = match_id.get();

        self.address_to_match_id_map(caller_clone).clear();
        // match id turns to 0, 0 means no room
        let address_vector = self.match_id_to_address_map(match_id).get();

        for address in address_vector {
            self.address_to_match_id_map(address).clear();
        }
    }

    #[init]
    fn init(&self, match_size: u32) {
        self.match_size().set(match_size);
    }

    // ***** PRIVATE  ***** //

    fn match_users(&self) {
        let match_size: u32 = self.match_size().get();
        let waiting_room_size: u32 = self.waiting_room().len() as u32;

        require!(waiting_room_size % match_size == 0, "Match size does not support the current number of people waiting");

        self.matches_count().set(self.matches_count().get() as u32 + 1);

        let new_match_id = self.matches_count().get();

        let mut users_list: Vec<ManagedAddress> = Vec::new();

        for _ in 0..match_size {
            let user = self.waiting_room().pop_front().unwrap().get_value_cloned();
            self.address_to_match_id_map(user.clone()).set(new_match_id);
            users_list.push(user);
        }

        require!(users_list.len() as u32 == match_size, "Something went wrong, not all the users have joined the room");

        self.match_id_to_address_map(new_match_id).set(users_list);
    }
}
