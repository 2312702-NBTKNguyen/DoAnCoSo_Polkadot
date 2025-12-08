//! # Blog Pallet
//!
//! Pallet quản lý blog phi tập trung trên blockchain Substrate.
//! Cho phép người dùng tạo, chỉnh sửa, xóa bài viết blog và quản lý bình luận.
//!
//! ## Tính năng chính:
//! ### Quản lý bài viết:
//! - Tạo bài viết blog mới
//! - Cập nhật bài viết đã tồn tại
//! - Xóa bài viết (soft delete)
//! - Thêm tags cho bài viết
//!
//! ### Quản lý bình luận:
//! - Thêm bình luận vào bài viết
//! - Cập nhật bình luận
//! - Xóa bình luận
//!
//! ### Tương tác xã hội:
//! - Like/Unlike bài viết và bình luận
//! - Bookmark bài viết yêu thích
//! - Follow/Unfollow tác giả
//!
//! ### Quản lý quyền truy cập:
//! - Chỉ tác giả mới có thể chỉnh sửa/xóa bài viết/bình luận của mình
//! - Phí giao dịch cho việc tạo bài viết và bình luận

#![cfg_attr(not(feature = "std"), no_std)]

pub use codec::{Decode, Encode, MaxEncodedLen};
pub use scale_info::TypeInfo;

use frame_support::{
	dispatch::{DispatchResult, DispatchResultWithPostInfo},
	ensure,
	pallet_prelude::*,
	traits::{
		ConstU32, Currency, ExistenceRequirement, Get, IsType, OnRuntimeUpgrade,
		ReservableCurrency, StorageVersion,
	},
	BoundedVec,
	PalletId,
	weights::Weight,
};

use frame_system::pallet_prelude::*;
use sp_runtime::{
	traits::AccountIdConversion,
	DispatchError,
};
use sp_std::vec::Vec;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;
pub mod weights;

pub use crate::pallet::*;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	// Import ensure_signed
	use frame_system::ensure_signed;

	/// Cấu hình pallet
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Loại sự kiện được phát ra từ pallet
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Đồng tiền được sử dụng cho các giao dịch
		type Currency: Currency<Self::AccountId> + ReservableCurrency<Self::AccountId>;

		/// Pallet ID để dự trữ tiền
		#[pallet::constant]
		type PalletId: Get<PalletId>;

		/// Phí tối đa để tạo bài viết
		#[pallet::constant]
		type PostCreationFee: Get<BalanceOf<Self>>;

		/// Phí tối đa để tạo bình luận
		#[pallet::constant]
		type CommentCreationFee: Get<BalanceOf<Self>>;

		/// Giới hạn độ dài tiêu đề bài viết
		#[pallet::constant]
		type MaxTitleLength: Get<u32>;

		/// Giới hạn độ dài nội dung bài viết
		#[pallet::constant]
		type MaxContentLength: Get<u32>;

		/// Giới hạn độ dài bình luận
		#[pallet::constant]
		type MaxCommentLength: Get<u32>;

		/// Giới hạn số lượng bình luận trên mỗi bài viết
		#[pallet::constant]
		type MaxCommentsPerPost: Get<u32>;

		/// Giới hạn số lượng tags trên mỗi bài viết
		#[pallet::constant]
		type MaxTagsPerPost: Get<u32>;

		/// Giới hạn độ dài mỗi tag
		#[pallet::constant]
		type MaxTagLength: Get<u32>;

		/// Weight information cho các extrinsics
		type WeightInfo: WeightInfo;
	}

	/// Balance type alias
	type BalanceOf<T> =
		<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// AccountId type alias
	type AccountIdOf<T> = <T as frame_system::Config>::AccountId;

	/// Weight functions required by the pallet extrinsics
	pub trait WeightInfo {
		fn create_post() -> Weight;
		fn update_post() -> Weight;
		fn delete_post() -> Weight;
		fn create_comment() -> Weight;
		fn update_comment() -> Weight;
		fn delete_comment() -> Weight;
		fn toggle_post_like() -> Weight;
		fn toggle_comment_like() -> Weight;
		fn add_tags() -> Weight;
		fn toggle_bookmark() -> Weight;
		fn toggle_follow() -> Weight;
	}

	const STORAGE_VERSION: StorageVersion = StorageVersion::new(1);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	/// ID bài viết hiện tại (auto-increment)
	#[pallet::storage]
	#[pallet::getter(fn next_post_id)]
	pub type NextPostId<T: Config> = StorageValue<_, PostId, ValueQuery>;

	/// Lưu trữ thông tin bài viết
	#[pallet::storage]
	#[pallet::getter(fn posts)]
	pub type Posts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		PostId,
		Post<AccountIdOf<T>, BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Lưu trữ danh sách bài viết của mỗi tác giả
	#[pallet::storage]
	#[pallet::getter(fn author_posts)]
	pub type AuthorPosts<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<PostId, ConstU32<1000>>,
		ValueQuery,
	>;

	/// ID bình luận hiện tại (auto-increment)
	#[pallet::storage]
	#[pallet::getter(fn next_comment_id)]
	pub type NextCommentId<T: Config> = StorageValue<_, CommentId, ValueQuery>;

	/// Lưu trữ thông tin bình luận
	#[pallet::storage]
	#[pallet::getter(fn comments)]
	pub type Comments<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		CommentId,
		Comment<AccountIdOf<T>, BlockNumberFor<T>>,
		OptionQuery,
	>;

	/// Lưu trữ danh sách bình luận của mỗi bài viết
	#[pallet::storage]
	#[pallet::getter(fn post_comments)]
	pub type PostComments<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		PostId,
		BoundedVec<CommentId, T::MaxCommentsPerPost>,
		ValueQuery,
	>;

	/// Lưu trữ số lượt like của mỗi bài viết
	#[pallet::storage]
	#[pallet::getter(fn post_likes)]
	pub type PostLikes<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		PostId,
		u64,
		ValueQuery,
	>;

	/// Lưu trữ người dùng đã like bài viết nào (để tránh like nhiều lần)
	#[pallet::storage]
	#[pallet::getter(fn post_liked_by)]
	pub type PostLikedBy<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		PostId,
		Blake2_128Concat,
		T::AccountId,
		bool,
		ValueQuery,
	>;

	/// Lưu trữ số lượt like của mỗi bình luận
	#[pallet::storage]
	#[pallet::getter(fn comment_likes)]
	pub type CommentLikes<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		CommentId,
		u64,
		ValueQuery,
	>;

	/// Lưu trữ người dùng đã like bình luận nào
	#[pallet::storage]
	#[pallet::getter(fn comment_liked_by)]
	pub type CommentLikedBy<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		CommentId,
		Blake2_128Concat,
		T::AccountId,
		bool,
		ValueQuery,
	>;

	/// Lưu trữ tags của mỗi bài viết
	#[pallet::storage]
	#[pallet::getter(fn post_tags)]
	pub type PostTags<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		PostId,
		BoundedVec<Vec<u8>, T::MaxTagsPerPost>,
		ValueQuery,
	>;

	/// Lưu trữ danh sách bài viết đã bookmark của mỗi người dùng
	#[pallet::storage]
	#[pallet::getter(fn user_bookmarks)]
	pub type UserBookmarks<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<PostId, ConstU32<1000>>,
		ValueQuery,
	>;

	/// Lưu trữ danh sách người dùng đã bookmark một bài viết
	#[pallet::storage]
	#[pallet::getter(fn post_bookmarked_by)]
	pub type PostBookmarkedBy<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		PostId,
		u64,
		ValueQuery,
	>;

	/// Lưu trữ danh sách người dùng đang follow của mỗi tác giả
	#[pallet::storage]
	#[pallet::getter(fn author_followers)]
	pub type AuthorFollowers<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		u64,
		ValueQuery,
	>;

	/// Lưu trữ danh sách tác giả mà người dùng đang follow
	#[pallet::storage]
	#[pallet::getter(fn user_following)]
	pub type UserFollowing<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		BoundedVec<T::AccountId, ConstU32<1000>>,
		ValueQuery,
	>;

	/// Kiểm tra xem người dùng có đang follow tác giả không
	#[pallet::storage]
	#[pallet::getter(fn is_following)]
	pub type IsFollowing<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::AccountId,
		bool,
		ValueQuery,
	>;

	pub mod migrations {
		use super::*;
		use core::marker::PhantomData;
		use frame_support::weights::Weight;

		#[cfg(feature = "try-runtime")]
		use codec::{Decode, Encode};

		pub struct V0ToV1<T>(PhantomData<T>);

		impl<T: Config> OnRuntimeUpgrade for V0ToV1<T> {
			fn on_runtime_upgrade() -> Weight {
				if <Pallet<T>>::on_chain_storage_version() >= STORAGE_VERSION {
					return Weight::zero();
				}

				let mut reads: u64 = 0;
				let mut writes: u64 = 0;

				Posts::<T>::translate::<PostV1<AccountIdOf<T>, BlockNumberFor<T>>, _>(
					|post_id, old| {
						reads = reads.saturating_add(1);
						let like_count = PostLikes::<T>::get(&post_id);
						reads = reads.saturating_add(1);

						let new_post = Post {
							id: old.id,
							author: old.author,
							title: old.title,
							content: old.content,
							created_at: old.created_at,
							updated_at: old.updated_at,
							is_deleted: old.is_deleted,
							likes: like_count,
						};

						writes = writes.saturating_add(1);
						Some(new_post)
					},
				);

				STORAGE_VERSION.put::<Pallet<T>>();
				writes = writes.saturating_add(1);

				T::DbWeight::get().reads_writes(reads, writes)
			}

			#[cfg(feature = "try-runtime")]
			fn pre_upgrade() -> Result<Vec<u8>, sp_runtime::TryRuntimeError> {
				let post_count = Posts::<T>::iter().count() as u64;
				Ok(post_count.encode())
			}

			#[cfg(feature = "try-runtime")]
			fn post_upgrade(state: Vec<u8>) -> Result<(), sp_runtime::TryRuntimeError> {
				let count_before = u64::decode(&mut &state[..])
					.map_err(|_| "Failed to decode pre-upgrade post count")?;
				let count_after = Posts::<T>::iter().count() as u64;
				frame_support::ensure!(
					count_before == count_after,
					"Post count mismatch after migration",
				);
				Ok(())
			}
		}
	}

	/// Events được phát ra từ pallet
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Bài viết mới được tạo
		/// [post_id, author, title]
		PostCreated {
			post_id: PostId,
			author: T::AccountId,
			title: Vec<u8>,
		},
		/// Bài viết được cập nhật
		/// [post_id, author]
		PostUpdated {
			post_id: PostId,
			author: T::AccountId,
		},
		/// Bài viết được xóa
		/// [post_id, author]
		PostDeleted {
			post_id: PostId,
			author: T::AccountId,
		},
		/// Bình luận mới được tạo
		/// [comment_id, post_id, author]
		CommentCreated {
			comment_id: CommentId,
			post_id: PostId,
			author: T::AccountId,
		},
		/// Bình luận được cập nhật
		/// [comment_id, author]
		CommentUpdated {
			comment_id: CommentId,
			author: T::AccountId,
		},
		/// Bình luận được xóa
		/// [comment_id, post_id, author]
		CommentDeleted {
			comment_id: CommentId,
			post_id: PostId,
			author: T::AccountId,
		},
		/// Bài viết được like
		/// [post_id, user]
		PostLiked {
			post_id: PostId,
			user: T::AccountId,
		},
		/// Like bài viết bị hủy
		/// [post_id, user]
		PostUnliked {
			post_id: PostId,
			user: T::AccountId,
		},
		/// Bình luận được like
		/// [comment_id, user]
		CommentLiked {
			comment_id: CommentId,
			user: T::AccountId,
		},
		/// Like bình luận bị hủy
		/// [comment_id, user]
		CommentUnliked {
			comment_id: CommentId,
			user: T::AccountId,
		},
		/// Tags được thêm vào bài viết
		/// [post_id, tags]
		PostTagged {
			post_id: PostId,
			tags: Vec<Vec<u8>>,
		},
		/// Bài viết được bookmark
		/// [post_id, user]
		PostBookmarked {
			post_id: PostId,
			user: T::AccountId,
		},
		/// Bookmark bài viết bị hủy
		/// [post_id, user]
		PostUnbookmarked {
			post_id: PostId,
			user: T::AccountId,
		},
		/// Bắt đầu follow tác giả
		/// [follower, author]
		AuthorFollowed {
			follower: T::AccountId,
			author: T::AccountId,
		},
		/// Hủy follow tác giả
		/// [follower, author]
		AuthorUnfollowed {
			follower: T::AccountId,
			author: T::AccountId,
		},
	}

	/// Errors có thể xảy ra trong pallet
	#[pallet::error]
	pub enum Error<T> {
		/// Bài viết không tồn tại
		PostNotFound,
		/// Không có quyền chỉnh sửa bài viết này
		NotPostAuthor,
		/// Bình luận không tồn tại
		CommentNotFound,
		/// Không có quyền chỉnh sửa bình luận này
		NotCommentAuthor,
		/// Tiêu đề quá dài
		TitleTooLong,
		/// Nội dung quá dài
		ContentTooLong,
		/// Bình luận quá dài
		CommentTooLong,
		/// Đã đạt giới hạn số lượng bình luận cho bài viết
		TooManyComments,
		/// Không đủ tiền để thực hiện giao dịch
		InsufficientBalance,
		/// Lỗi số học (overflow/underflow)
		ArithmeticError,
		/// Bài viết đã bị xóa
		PostDeleted,
		/// Đã like rồi, không thể like lại
		AlreadyLiked,
		/// Chưa like, không thể unlike
		NotLiked,
		/// Tag quá dài
		TagTooLong,
		/// Quá nhiều tags
		TooManyTags,
		/// Đã bookmark rồi
		AlreadyBookmarked,
		/// Chưa bookmark, không thể unbookmark
		NotBookmarked,
		/// Không thể follow chính mình
		CannotFollowSelf,
		/// Đã follow rồi
		AlreadyFollowing,
		/// Chưa follow, không thể unfollow
		NotFollowing,
	}

	/// Hooks cho pallet
	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(_n: BlockNumberFor<T>) -> Weight {
			Weight::zero()
		}

		fn on_finalize(_n: BlockNumberFor<T>) {
			// Cleanup logic có thể được thêm vào đây
		}

		fn on_runtime_upgrade() -> Weight {
			migrations::V0ToV1::<T>::on_runtime_upgrade()
		}
	}

	/// Extrinsics (các hàm callable từ bên ngoài)
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Tạo bài viết blog mới
		///
		/// Yêu cầu phí từ tác giả để tạo bài viết.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::create_post())]
		pub fn create_post(
			origin: OriginFor<T>,
			title: Vec<u8>,
			content: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let author = ensure_signed(origin)?;

			// Kiểm tra độ dài
			let max_title_len = T::MaxTitleLength::get() as usize;
			let max_content_len = T::MaxContentLength::get() as usize;
			ensure!(
				title.len() <= max_title_len,
				Error::<T>::TitleTooLong
			);
			ensure!(
				content.len() <= max_content_len,
				Error::<T>::ContentTooLong
			);

			// Thu phí
			let fee = T::PostCreationFee::get();
			Self::charge_fee(&author, fee)?;

			// Tạo ID bài viết mới
			let post_id = NextPostId::<T>::try_mutate(|id| -> Result<PostId, DispatchError> {
				let current = *id;
				*id = id.checked_add(1).ok_or(Error::<T>::ArithmeticError)?;
				Ok(current)
			})?;

			// Tạo bài viết
			let now = <frame_system::Pallet<T>>::block_number();
			let post = Post {
				id: post_id,
				author: author.clone(),
				title: title.clone(),
				content,
				created_at: now,
				updated_at: now,
				is_deleted: false,
				likes: PostLikes::<T>::get(&post_id),
			};

			Posts::<T>::insert(post_id, &post);

			// Cập nhật danh sách bài viết của tác giả
			AuthorPosts::<T>::try_mutate(&author, |posts| -> DispatchResult {
				posts.try_push(post_id)
					.map_err(|_| Error::<T>::ArithmeticError)?;
				Ok(())
			})?;

			Self::deposit_event(Event::PostCreated {
				post_id,
				author,
				title,
			});

			Ok(().into())
		}

		/// Cập nhật bài viết đã tồn tại
		///
		/// Chỉ tác giả của bài viết mới có thể cập nhật.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::update_post())]
		pub fn update_post(
			origin: OriginFor<T>,
			post_id: PostId,
			title: Option<Vec<u8>>,
			content: Option<Vec<u8>>,
		) -> DispatchResultWithPostInfo {
			let author = ensure_signed(origin)?;

			// Lấy bài viết
			let mut post = Posts::<T>::get(post_id)
				.ok_or(Error::<T>::PostNotFound)?;

			// Kiểm tra quyền
			ensure!(author == post.author, Error::<T>::NotPostAuthor);
			ensure!(!post.is_deleted, Error::<T>::PostDeleted);

			// Cập nhật tiêu đề nếu có
			if let Some(new_title) = title {
				let max_title_len = T::MaxTitleLength::get() as usize;
				ensure!(
					new_title.len() <= max_title_len,
					Error::<T>::TitleTooLong
				);
				post.title = new_title;
			}

			// Cập nhật nội dung nếu có
			if let Some(new_content) = content {
				let max_content_len = T::MaxContentLength::get() as usize;
				ensure!(
					new_content.len() <= max_content_len,
					Error::<T>::ContentTooLong
				);
				post.content = new_content;
			}

			post.updated_at = <frame_system::Pallet<T>>::block_number();
			Posts::<T>::insert(post_id, &post);

			Self::deposit_event(Event::PostUpdated {
				post_id,
				author,
			});

			Ok(().into())
		}

		/// Xóa bài viết
		///
		/// Chỉ tác giả của bài viết mới có thể xóa.
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::delete_post())]
		pub fn delete_post(
			origin: OriginFor<T>,
			post_id: PostId,
		) -> DispatchResultWithPostInfo {
			let author = ensure_signed(origin)?;

			// Lấy bài viết
			let mut post = Posts::<T>::get(post_id)
				.ok_or(Error::<T>::PostNotFound)?;

			// Kiểm tra quyền
			ensure!(author == post.author, Error::<T>::NotPostAuthor);
			ensure!(!post.is_deleted, Error::<T>::PostDeleted);

			// Đánh dấu là đã xóa
			post.is_deleted = true;
			post.updated_at = <frame_system::Pallet<T>>::block_number();
			Posts::<T>::insert(post_id, &post);

			Self::deposit_event(Event::PostDeleted {
				post_id,
				author,
			});

			Ok(().into())
		}

		/// Thêm bình luận vào bài viết
		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::create_comment())]
		pub fn create_comment(
			origin: OriginFor<T>,
			post_id: PostId,
			content: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let author = ensure_signed(origin)?;

			// Kiểm tra bài viết tồn tại
			let post = Posts::<T>::get(post_id)
				.ok_or(Error::<T>::PostNotFound)?;
			ensure!(!post.is_deleted, Error::<T>::PostDeleted);

			// Kiểm tra độ dài bình luận
			let max_comment_len = T::MaxCommentLength::get() as usize;
			ensure!(
				content.len() <= max_comment_len,
				Error::<T>::CommentTooLong
			);

			// Kiểm tra số lượng bình luận
			let comments = PostComments::<T>::get(&post_id);
			let max_comments = T::MaxCommentsPerPost::get() as usize;
			ensure!(
				comments.len() < max_comments,
				Error::<T>::TooManyComments
			);

			// Thu phí
			let fee = T::CommentCreationFee::get();
			Self::charge_fee(&author, fee)?;

			// Tạo ID bình luận mới
			let comment_id = NextCommentId::<T>::try_mutate(|id| -> Result<CommentId, DispatchError> {
				let current = *id;
				*id = id.checked_add(1).ok_or(Error::<T>::ArithmeticError)?;
				Ok(current)
			})?;

			// Tạo bình luận
			let now = <frame_system::Pallet<T>>::block_number();
			let comment = Comment {
				id: comment_id,
				post_id,
				author: author.clone(),
				content,
				created_at: now,
				updated_at: now,
				is_deleted: false,
			};

			Comments::<T>::insert(comment_id, &comment);

			// Cập nhật danh sách bình luận của bài viết
			PostComments::<T>::try_mutate(&post_id, |comments| -> DispatchResult {
				comments.try_push(comment_id)
					.map_err(|_| Error::<T>::TooManyComments)?;
				Ok(())
			})?;

			Self::deposit_event(Event::CommentCreated {
				comment_id,
				post_id,
				author,
			});

			Ok(().into())
		}

		/// Cập nhật bình luận
		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::update_comment())]
		pub fn update_comment(
			origin: OriginFor<T>,
			comment_id: CommentId,
			content: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let author = ensure_signed(origin)?;

			// Lấy bình luận
			let mut comment = Comments::<T>::get(comment_id)
				.ok_or(Error::<T>::CommentNotFound)?;

			// Kiểm tra quyền
			ensure!(author == comment.author, Error::<T>::NotCommentAuthor);
			ensure!(!comment.is_deleted, Error::<T>::CommentNotFound);

			// Kiểm tra độ dài
			let max_comment_len = T::MaxCommentLength::get() as usize;
			ensure!(
				content.len() <= max_comment_len,
				Error::<T>::CommentTooLong
			);

			comment.content = content;
			comment.updated_at = <frame_system::Pallet<T>>::block_number();
			Comments::<T>::insert(comment_id, &comment);

			Self::deposit_event(Event::CommentUpdated {
				comment_id,
				author,
			});

			Ok(().into())
		}

		/// Xóa bình luận
		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::delete_comment())]
		pub fn delete_comment(
			origin: OriginFor<T>,
			comment_id: CommentId,
		) -> DispatchResultWithPostInfo {
			let author = ensure_signed(origin)?;

			// Lấy bình luận
			let mut comment = Comments::<T>::get(comment_id)
				.ok_or(Error::<T>::CommentNotFound)?;

			// Kiểm tra quyền
			ensure!(author == comment.author, Error::<T>::NotCommentAuthor);
			ensure!(!comment.is_deleted, Error::<T>::CommentNotFound);

			// Đánh dấu là đã xóa
			comment.is_deleted = true;
			comment.updated_at = <frame_system::Pallet<T>>::block_number();
			Comments::<T>::insert(comment_id, &comment);

			Self::deposit_event(Event::CommentDeleted {
				comment_id,
				post_id: comment.post_id,
				author,
			});

			Ok(().into())
		}

		/// Like hoặc unlike bài viết
		#[pallet::call_index(6)]
		#[pallet::weight(T::WeightInfo::toggle_post_like())]
		pub fn toggle_post_like(
			origin: OriginFor<T>,
			post_id: PostId,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Kiểm tra bài viết tồn tại
			let post = Posts::<T>::get(post_id)
				.ok_or(Error::<T>::PostNotFound)?;
			ensure!(!post.is_deleted, Error::<T>::PostDeleted);

			let already_liked = PostLikedBy::<T>::get(&post_id, &user);

			if already_liked {
				// Unlike
				PostLikedBy::<T>::remove(&post_id, &user);
				PostLikes::<T>::mutate(&post_id, |likes| {
					*likes = likes.saturating_sub(1);
				});

				Posts::<T>::mutate(post_id, |maybe_post| {
					if let Some(post) = maybe_post {
						post.likes = post.likes.saturating_sub(1);
					}
				});

				Self::deposit_event(Event::PostUnliked {
					post_id,
					user,
				});
			} else {
				// Like
				PostLikedBy::<T>::insert(&post_id, &user, true);
				PostLikes::<T>::mutate(&post_id, |likes| {
					*likes = likes.saturating_add(1);
				});

				Posts::<T>::mutate(post_id, |maybe_post| {
					if let Some(post) = maybe_post {
						post.likes = post.likes.saturating_add(1);
					}
				});

				Self::deposit_event(Event::PostLiked {
					post_id,
					user,
				});
			}

			Ok(().into())
		}

		/// Like hoặc unlike bình luận
		#[pallet::call_index(7)]
		#[pallet::weight(T::WeightInfo::toggle_comment_like())]
		pub fn toggle_comment_like(
			origin: OriginFor<T>,
			comment_id: CommentId,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Kiểm tra bình luận tồn tại
			let comment = Comments::<T>::get(comment_id)
				.ok_or(Error::<T>::CommentNotFound)?;
			ensure!(!comment.is_deleted, Error::<T>::CommentNotFound);

			let already_liked = CommentLikedBy::<T>::get(&comment_id, &user);

			if already_liked {
				// Unlike
				CommentLikedBy::<T>::remove(&comment_id, &user);
				CommentLikes::<T>::mutate(&comment_id, |likes| {
					*likes = likes.saturating_sub(1);
				});

				Self::deposit_event(Event::CommentUnliked {
					comment_id,
					user,
				});
			} else {
				// Like
				CommentLikedBy::<T>::insert(&comment_id, &user, true);
				CommentLikes::<T>::mutate(&comment_id, |likes| {
					*likes = likes.saturating_add(1);
				});

				Self::deposit_event(Event::CommentLiked {
					comment_id,
					user,
				});
			}

			Ok(().into())
		}

		/// Thêm tags cho bài viết (chỉ tác giả mới có thể thêm)
		#[pallet::call_index(8)]
		#[pallet::weight(T::WeightInfo::add_tags())]
		pub fn add_tags(
			origin: OriginFor<T>,
			post_id: PostId,
			tags: Vec<Vec<u8>>,
		) -> DispatchResultWithPostInfo {
			let author = ensure_signed(origin)?;

			// Kiểm tra bài viết tồn tại và quyền
			let post = Posts::<T>::get(post_id)
				.ok_or(Error::<T>::PostNotFound)?;
			ensure!(author == post.author, Error::<T>::NotPostAuthor);
			ensure!(!post.is_deleted, Error::<T>::PostDeleted);

			// Kiểm tra số lượng tags
			let max_tags = T::MaxTagsPerPost::get() as usize;
			ensure!(
				tags.len() <= max_tags,
				Error::<T>::TooManyTags
			);

			// Validate và lưu tags
			let mut valid_tags: BoundedVec<Vec<u8>, T::MaxTagsPerPost> = BoundedVec::new();
			let max_tag_len = T::MaxTagLength::get() as usize;
			for tag in tags.iter() {
				ensure!(
					tag.len() <= max_tag_len,
					Error::<T>::TagTooLong
				);
				valid_tags.try_push(tag.clone())
					.map_err(|_| Error::<T>::TooManyTags)?;
			}

			// Lấy tags hiện tại và merge
			let mut current_tags = PostTags::<T>::get(&post_id);
			for tag in valid_tags.iter() {
				// Tránh duplicate
				if !current_tags.contains(tag) {
					current_tags.try_push(tag.clone())
						.map_err(|_| Error::<T>::TooManyTags)?;
				}
			}

			PostTags::<T>::insert(&post_id, &current_tags);

			Self::deposit_event(Event::PostTagged {
				post_id,
				tags: current_tags.into_inner(),
			});

			Ok(().into())
		}

		/// Bookmark hoặc unbookmark bài viết
		#[pallet::call_index(9)]
		#[pallet::weight(T::WeightInfo::toggle_bookmark())]
		pub fn toggle_bookmark(
			origin: OriginFor<T>,
			post_id: PostId,
		) -> DispatchResultWithPostInfo {
			let user = ensure_signed(origin)?;

			// Kiểm tra bài viết tồn tại
			let post = Posts::<T>::get(post_id)
				.ok_or(Error::<T>::PostNotFound)?;
			ensure!(!post.is_deleted, Error::<T>::PostDeleted);

			let user_bookmarks = UserBookmarks::<T>::get(&user);
			let is_bookmarked = user_bookmarks.contains(&post_id);

			if is_bookmarked {
				// Unbookmark
				UserBookmarks::<T>::try_mutate(&user, |bookmarks| -> DispatchResult {
					let pos = bookmarks.iter()
						.position(|&id| id == post_id)
						.ok_or(Error::<T>::NotBookmarked)?;
					bookmarks.remove(pos);
					Ok(())
				})?;

				PostBookmarkedBy::<T>::mutate(&post_id, |count| {
					*count = count.saturating_sub(1);
				});

				Self::deposit_event(Event::PostUnbookmarked {
					post_id,
					user,
				});
			} else {
				// Bookmark
				UserBookmarks::<T>::try_mutate(&user, |bookmarks| -> DispatchResult {
					bookmarks.try_push(post_id)
						.map_err(|_| Error::<T>::ArithmeticError)?;
					Ok(())
				})?;

				PostBookmarkedBy::<T>::mutate(&post_id, |count| {
					*count = count.saturating_add(1);
				});

				Self::deposit_event(Event::PostBookmarked {
					post_id,
					user,
				});
			}

			Ok(().into())
		}

		/// Follow hoặc unfollow tác giả
		#[pallet::call_index(10)]
		#[pallet::weight(T::WeightInfo::toggle_follow())]
		pub fn toggle_follow(
			origin: OriginFor<T>,
			author: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let follower = ensure_signed(origin)?;

			// Không thể follow chính mình
			ensure!(follower != author, Error::<T>::CannotFollowSelf);

			let is_following = IsFollowing::<T>::get(&follower, &author);

			if is_following {
				// Unfollow
				IsFollowing::<T>::remove(&follower, &author);
				AuthorFollowers::<T>::mutate(&author, |count| {
					*count = count.saturating_sub(1);
				});
				UserFollowing::<T>::try_mutate(&follower, |following| -> DispatchResult {
					let pos = following.iter()
						.position(|acc| acc == &author)
						.ok_or(Error::<T>::NotFollowing)?;
					following.remove(pos);
					Ok(())
				})?;

				Self::deposit_event(Event::AuthorUnfollowed {
					follower,
					author,
				});
			} else {
				// Follow
				IsFollowing::<T>::insert(&follower, &author, true);
				AuthorFollowers::<T>::mutate(&author, |count| {
					*count = count.saturating_add(1);
				});
				UserFollowing::<T>::try_mutate(&follower, |following| -> DispatchResult {
					following.try_push(author.clone())
						.map_err(|_| Error::<T>::ArithmeticError)?;
					Ok(())
				})?;

				Self::deposit_event(Event::AuthorFollowed {
					follower,
					author,
				});
			}

			Ok(().into())
		}
	}

	// Implementation block cho các hàm helper
	impl<T: Config> Pallet<T> {
		/// Thu phí từ tài khoản
		fn charge_fee(account: &T::AccountId, fee: BalanceOf<T>) -> DispatchResult {
			let pallet_account: T::AccountId = T::PalletId::get().into_account_truncating();

			T::Currency::transfer(
				account,
				&pallet_account,
				fee,
				ExistenceRequirement::KeepAlive,
			)?;

			Ok(())
		}
	}
}

/// ID bài viết
pub type PostId = u64;

/// ID bình luận
pub type CommentId = u64;

/// Dữ liệu bài viết phiên bản cũ (trước khi thêm trường likes)
#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct PostV1<AccountId, BlockNumber>
where
	AccountId: Clone + PartialEq + Eq,
	BlockNumber: Clone + PartialEq + Eq,
{
	pub id: PostId,
	pub author: AccountId,
	pub title: Vec<u8>,
	pub content: Vec<u8>,
	pub created_at: BlockNumber,
	pub updated_at: BlockNumber,
	pub is_deleted: bool,
}

/// Cấu trúc bài viết (phiên bản hiện tại)
#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Post<AccountId, BlockNumber>
where
	AccountId: Clone + PartialEq + Eq,
	BlockNumber: Clone + PartialEq + Eq,
{
	/// ID bài viết
	pub id: PostId,
	/// Tác giả
	pub author: AccountId,
	/// Tiêu đề
	pub title: Vec<u8>,
	/// Nội dung
	pub content: Vec<u8>,
	/// Thời điểm tạo
	pub created_at: BlockNumber,
	/// Thời điểm cập nhật cuối
	pub updated_at: BlockNumber,
	/// Đã bị xóa chưa (soft delete)
	pub is_deleted: bool,
	/// Tổng số lượt like (được migrate từ PostLikes)
	pub likes: u64,
}

/// Cấu trúc bình luận
#[derive(Encode, Decode, Clone, PartialEq, Eq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct Comment<AccountId, BlockNumber>
where
	AccountId: Clone + PartialEq + Eq,
	BlockNumber: Clone + PartialEq + Eq,
{
	/// ID bình luận
	pub id: CommentId,
	/// ID bài viết mà bình luận này thuộc về
	pub post_id: PostId,
	/// Tác giả
	pub author: AccountId,
	/// Nội dung
	pub content: Vec<u8>,
	/// Thời điểm tạo
	pub created_at: BlockNumber,
	/// Thời điểm cập nhật cuối
	pub updated_at: BlockNumber,
	/// Đã bị xóa chưa (soft delete)
	pub is_deleted: bool,
}
