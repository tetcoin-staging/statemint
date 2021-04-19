// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{mock::*, Error, AuthorInfo};
use frame_support::{assert_noop, assert_ok, traits::{OnInitialize, Currency}};
use sp_runtime::traits::BadOrigin;
use pallet_balances::Error as BalancesError;

#[test]
fn it_should_set_invulnerables() {
	new_test_ext().execute_with(|| {
		let new_handlers = vec![1, 2, 3, 4];
		assert_ok!(SimpleStaking::set_invulnerables(
			Origin::signed(RootAccount::get()),
			new_handlers.clone()
		));
		assert_noop!(
			SimpleStaking::set_invulnerables(Origin::signed(1), new_handlers.clone()),
			BadOrigin
		);
		assert_eq!(SimpleStaking::invulnerables(), new_handlers);
	});
}

#[test]
fn set_allowed_author_count() {
	new_test_ext().execute_with(|| {
		assert_ok!(SimpleStaking::set_allowed_author_count(Origin::signed(RootAccount::get()), 7));
		assert_noop!(SimpleStaking::set_allowed_author_count(Origin::signed(1), 8), BadOrigin);
		assert_eq!(SimpleStaking::allowed_authors(), 7);
	});
}

#[test]
fn set_author_bond() {
	new_test_ext().execute_with(|| {
		assert_ok!(SimpleStaking::set_author_bond(Origin::signed(RootAccount::get()), 7));
		assert_noop!(SimpleStaking::set_author_bond(Origin::signed(1), 8), BadOrigin);
		assert_eq!(SimpleStaking::author_bond(), 7);
	});
}

#[test]
fn register_as_author() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			SimpleStaking::register_as_author(Origin::signed(1)),
			Error::<Test>::TooManyAuthors
		);

		assert_ok!(SimpleStaking::set_author_bond(Origin::signed(RootAccount::get()), 10));
		assert_ok!(SimpleStaking::set_allowed_author_count(Origin::signed(RootAccount::get()), 1));
		
		// can add 1 new 
		assert_ok!(SimpleStaking::register_as_author(Origin::signed(1)));
		let addition = AuthorInfo {
			who: 1,
			deposit: 10,
			last_block: None
		};
		assert_eq!(SimpleStaking::authors(), vec![addition]);
		assert_eq!(Balances::free_balance(1), 90);
		
		// but no more
		assert_noop!(
			SimpleStaking::register_as_author(Origin::signed(1)),
			Error::<Test>::TooManyAuthors
		);

		// increase limit
		assert_ok!(SimpleStaking::set_allowed_author_count(Origin::signed(RootAccount::get()), 5));
		// but still won't accept dupe..
		assert_noop!(
			SimpleStaking::register_as_author(Origin::signed(1)),
			Error::<Test>::AlreadyAuthor
		);
		// or poor author (3 is not endowed)
		assert_noop!(
			SimpleStaking::register_as_author(Origin::signed(3)),
			BalancesError::<Test>::InsufficientBalance
		);
		// but an endowed account works now
		assert_ok!(SimpleStaking::register_as_author(Origin::signed(2)));
		
		assert_eq!(SimpleStaking::authors().len(), 2);
	});
}

#[test]
fn leave_intent() {
	new_test_ext().execute_with(|| {
		assert_ok!(SimpleStaking::set_allowed_author_count(Origin::signed(RootAccount::get()), 1));
		assert_ok!(SimpleStaking::set_author_bond(Origin::signed(RootAccount::get()), 10));
		assert_ok!(SimpleStaking::register_as_author(Origin::signed(1)));
		assert_eq!(Balances::free_balance(1), 90);
		assert_noop!(
			SimpleStaking::leave_intent(Origin::signed(RootAccount::get())),
			Error::<Test>::NotAuthor
		);
		assert_ok!(SimpleStaking::leave_intent(Origin::signed(1)));
		assert_eq!(SimpleStaking::authors(), vec![]);
		assert_eq!(Balances::free_balance(1), 100);
	});
}

#[test]
fn authorship_event_handler() {
	new_test_ext().execute_with(|| {
		Balances::make_free_balance_be(&SimpleStaking::account_id(), 100);
		// 4 is the default author. 
		assert_eq!(Balances::free_balance(4), 0);
		assert_eq!(Balances::free_balance(SimpleStaking::account_id()), 100);
		// triggers `note_author`
		Authorship::on_initialize(1);
		
		// half of the pot goes to the author. 
		assert_eq!(Balances::free_balance(4), 50);
		// half stays. 
		assert_eq!(Balances::free_balance(SimpleStaking::account_id()), 50);
	});
}

#[test]
fn on_genesis() {
	new_test_ext().execute_with(|| {
		assert_eq!(SimpleStaking::invulnerables(), vec![1,2,3]);
		assert_eq!(SimpleStaking::invulnerables().len(), 3);
	});
}