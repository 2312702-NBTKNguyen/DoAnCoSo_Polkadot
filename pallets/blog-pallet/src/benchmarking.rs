//! Benchmarks cho blog pallet
//! Sử dụng frame-benchmarking để đo lường performance

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use frame_benchmarking::v2::*;
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_runtime::traits::SaturatedConversion;
use sp_std::{vec, vec::Vec};

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const SEED: u32 = 0;

fn set_block_number<T: Config>(block: u32) {
	frame_system::Pallet::<T>::set_block_number(block.saturated_into());
}

fn fund_account<T: Config>(account: &T::AccountId) {
	let deposit: BalanceOf<T> = 1_000_000_000u128.saturated_into();
	let _imbalance = T::Currency::deposit_creating(account, deposit);
}

fn default_title<T: Config>() -> Vec<u8> {
	let len = core::cmp::min(T::MaxTitleLength::get(), 16) as usize;
	vec![b't'; len]
}

fn default_content<T: Config>() -> Vec<u8> {
	let len = core::cmp::min(T::MaxContentLength::get(), 64) as usize;
	vec![b'c'; len]
}

fn default_comment<T: Config>() -> Vec<u8> {
	let len = core::cmp::min(T::MaxCommentLength::get(), 32) as usize;
	vec![b'm'; len]
}

fn create_post_for_bench<T: Config>(author: &T::AccountId) -> PostId {
	let post_id = NextPostId::<T>::get();
	let _ = Pallet::<T>::create_post(
		RawOrigin::Signed(author.clone()).into(),
		default_title::<T>(),
		default_content::<T>(),
	);
	post_id
}

fn create_comment_for_bench<T: Config>(author: &T::AccountId, post_id: PostId) -> CommentId {
	let comment_id = NextCommentId::<T>::get();
	let _ = Pallet::<T>::create_comment(
		RawOrigin::Signed(author.clone()).into(),
		post_id,
		default_comment::<T>(),
	);
	comment_id
}

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn create_post() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let caller: T::AccountId = whitelisted_caller();
		fund_account::<T>(&caller);

		#[extrinsic_call]
		create_post(
			RawOrigin::Signed(caller.clone()),
			default_title::<T>(),
			default_content::<T>(),
		);

		assert!(NextPostId::<T>::get() > 0);
		Ok(())
	}

	#[benchmark]
	fn update_post() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let caller: T::AccountId = whitelisted_caller();
		fund_account::<T>(&caller);
		let post_id = create_post_for_bench::<T>(&caller);

		let new_title = vec![b'n'; core::cmp::min(T::MaxTitleLength::get(), 12) as usize];
		let new_content = vec![b'd'; core::cmp::min(T::MaxContentLength::get(), 48) as usize];

		#[extrinsic_call]
		update_post(
			RawOrigin::Signed(caller.clone()),
			post_id,
			Some(new_title.clone()),
			Some(new_content.clone()),
		);

		let stored = Posts::<T>::get(post_id).expect("post exists");
		assert!(stored.updated_at >= frame_system::Pallet::<T>::block_number());
		Ok(())
	}

	#[benchmark]
	fn delete_post() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let caller: T::AccountId = whitelisted_caller();
		fund_account::<T>(&caller);
		let post_id = create_post_for_bench::<T>(&caller);

		#[extrinsic_call]
		delete_post(RawOrigin::Signed(caller.clone()), post_id);

		let stored = Posts::<T>::get(post_id).expect("post exists");
		assert!(stored.is_deleted);
		Ok(())
	}

	#[benchmark]
	fn create_comment() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let author: T::AccountId = whitelisted_caller();
		let commenter: T::AccountId = account("commenter", 0, SEED);
		fund_account::<T>(&author);
		fund_account::<T>(&commenter);
		let post_id = create_post_for_bench::<T>(&author);

		#[extrinsic_call]
		create_comment(
			RawOrigin::Signed(commenter.clone()),
			post_id,
			default_comment::<T>(),
		);

		assert!(NextCommentId::<T>::get() > 0);
		Ok(())
	}

	#[benchmark]
	fn update_comment() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let author: T::AccountId = whitelisted_caller();
		fund_account::<T>(&author);
		let post_id = create_post_for_bench::<T>(&author);
		let comment_id = create_comment_for_bench::<T>(&author, post_id);

		let new_content = vec![b'u'; core::cmp::min(T::MaxCommentLength::get(), 24) as usize];

		#[extrinsic_call]
		update_comment(
			RawOrigin::Signed(author.clone()),
			comment_id,
			new_content.clone(),
		);

		let stored = Comments::<T>::get(comment_id).expect("comment exists");
		assert!(stored.updated_at >= frame_system::Pallet::<T>::block_number());
		Ok(())
	}

	#[benchmark]
	fn delete_comment() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let author: T::AccountId = whitelisted_caller();
		fund_account::<T>(&author);
		let post_id = create_post_for_bench::<T>(&author);
		let comment_id = create_comment_for_bench::<T>(&author, post_id);

		#[extrinsic_call]
		delete_comment(RawOrigin::Signed(author.clone()), comment_id);

		let stored = Comments::<T>::get(comment_id).expect("comment exists");
		assert!(stored.is_deleted);
		Ok(())
	}

	#[benchmark]
	fn toggle_post_like() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let author: T::AccountId = whitelisted_caller();
		let liker: T::AccountId = account("liker", 0, SEED);
		fund_account::<T>(&author);
		let post_id = create_post_for_bench::<T>(&author);

		#[extrinsic_call]
		toggle_post_like(RawOrigin::Signed(liker.clone()), post_id);

		assert!(PostLikedBy::<T>::get(post_id, liker));
		Ok(())
	}

	#[benchmark]
	fn toggle_comment_like() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let author: T::AccountId = whitelisted_caller();
		let liker: T::AccountId = account("comment_liker", 0, SEED);
		fund_account::<T>(&author);
		let post_id = create_post_for_bench::<T>(&author);
		let comment_id = create_comment_for_bench::<T>(&author, post_id);

		#[extrinsic_call]
		toggle_comment_like(RawOrigin::Signed(liker.clone()), comment_id);

		assert!(CommentLikedBy::<T>::get(comment_id, liker));
		Ok(())
	}

	#[benchmark]
	fn add_tags() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let author: T::AccountId = whitelisted_caller();
		fund_account::<T>(&author);
		let post_id = create_post_for_bench::<T>(&author);

		if T::MaxTagsPerPost::get() == 0 || T::MaxTagLength::get() == 0 {
			return Err(BenchmarkError::Skip);
		}

		let max_tags = T::MaxTagsPerPost::get() as usize;
		let tags_len = core::cmp::min(max_tags, 5).max(1);
		let tag_len = core::cmp::min(T::MaxTagLength::get(), 8) as usize;
		let tags: Vec<Vec<u8>> = (0..tags_len)
			.map(|i| vec![(i % 255) as u8; tag_len])
			.collect();

		#[extrinsic_call]
		add_tags(RawOrigin::Signed(author.clone()), post_id, tags.clone());

		let stored = PostTags::<T>::get(post_id);
		assert!(stored.len() >= tags_len);
		Ok(())
	}

	#[benchmark]
	fn toggle_bookmark() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let author: T::AccountId = whitelisted_caller();
		let user: T::AccountId = account("bookmark", 0, SEED);
		fund_account::<T>(&author);
		let post_id = create_post_for_bench::<T>(&author);

		#[extrinsic_call]
		toggle_bookmark(RawOrigin::Signed(user.clone()), post_id);

		assert!(UserBookmarks::<T>::get(&user).contains(&post_id));
		Ok(())
	}

	#[benchmark]
	fn toggle_follow() -> Result<(), BenchmarkError> {
		set_block_number::<T>(1);
		let author: T::AccountId = whitelisted_caller();
		let follower: T::AccountId = account("follower", 0, SEED);
		fund_account::<T>(&author);
		fund_account::<T>(&follower);

		#[extrinsic_call]
		toggle_follow(RawOrigin::Signed(follower.clone()), author.clone());

		assert!(IsFollowing::<T>::get(&follower, &author));
		Ok(())
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test);
}
