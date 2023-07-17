use candid::{ CandidType, Deserialize, Principal };

#[derive(CandidType, Clone, Deserialize)]
pub struct User {
	pub user_id: Principal,
	pub username: Option<String>,
	pub created_at: u64,
	pub canisters: Vec<Principal>,
	pub alias_user_ids: Option<Vec<Principal>>,
}

impl Default for User {
	fn default() -> Self {
		Self {
			user_id: Principal::anonymous(),
			username: Default::default(),
			created_at: Default::default(),
			canisters: Default::default(),
			alias_user_ids: Default::default(),
		}
	}
}
