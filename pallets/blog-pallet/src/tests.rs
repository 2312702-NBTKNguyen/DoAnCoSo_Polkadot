//! Tests cho blog pallet

use crate::{mock::*, Event};
use frame_support::{assert_noop, assert_ok};

#[cfg(test)]
mod tests {
	use super::*;
	use crate::pallet::Error;

	#[test]
	fn create_post_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Kiểm tra bài viết đã được tạo
			let post = Blog::posts(0).unwrap();
			assert_eq!(post.title, title);
			assert_eq!(post.content, content);
			assert_eq!(post.author, 1);
			assert_eq!(post.id, 0);

			// Kiểm tra event
			System::assert_last_event(
				RuntimeEvent::Blog(Event::PostCreated {
					post_id: 0,
					author: 1,
					title,
				})
			);
		});
	}

	#[test]
	fn update_post_works() {
		new_test_ext().execute_with(|| {
			let title = b"Original Title".to_vec();
			let content = b"Original Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Cập nhật bài viết
			let new_title = b"Updated Title".to_vec();
			assert_ok!(Blog::update_post(
				RuntimeOrigin::signed(1),
				0,
				Some(new_title.clone()),
				None
			));

			// Kiểm tra đã được cập nhật
			let post = Blog::posts(0).unwrap();
			assert_eq!(post.title, new_title);
			assert_eq!(post.content, content); // Nội dung không đổi
		});
	}

	#[test]
	fn update_post_fails_if_not_author() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết bởi user 1
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// User 2 cố gắng cập nhật - should fail
			assert_noop!(
				Blog::update_post(
					RuntimeOrigin::signed(2),
					0,
					Some(b"New Title".to_vec()),
					None
				),
				Error::<Test>::NotPostAuthor
			);
		});
	}

	#[test]
	fn delete_post_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Xóa bài viết
			assert_ok!(Blog::delete_post(RuntimeOrigin::signed(1), 0));

			// Kiểm tra đã được đánh dấu xóa
			let post = Blog::posts(0).unwrap();
			assert_eq!(post.is_deleted, true);
		});
	}

	#[test]
	fn create_comment_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Tạo bình luận
			let comment_content = b"Great post!".to_vec();
			assert_ok!(Blog::create_comment(
				RuntimeOrigin::signed(2),
				0,
				comment_content.clone()
			));

			// Kiểm tra bình luận đã được tạo
			let comment = Blog::comments(0).unwrap();
			assert_eq!(comment.content, comment_content);
			assert_eq!(comment.post_id, 0);
			assert_eq!(comment.author, 2);
		});
	}

	#[test]
	fn create_comment_fails_if_post_not_found() {
		new_test_ext().execute_with(|| {
			let comment_content = b"Comment".to_vec();

			// Cố gắng bình luận vào bài viết không tồn tại
			assert_noop!(
				Blog::create_comment(
					RuntimeOrigin::signed(1),
					999,
					comment_content
				),
				Error::<Test>::PostNotFound
			);
		});
	}

	#[test]
	fn update_comment_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Tạo bình luận
			let comment_content = b"Original comment".to_vec();
			assert_ok!(Blog::create_comment(
				RuntimeOrigin::signed(2),
				0,
				comment_content.clone()
			));

			// Cập nhật bình luận
			let new_comment = b"Updated comment".to_vec();
			assert_ok!(Blog::update_comment(
				RuntimeOrigin::signed(2),
				0,
				new_comment.clone()
			));

			// Kiểm tra đã được cập nhật
			let comment = Blog::comments(0).unwrap();
			assert_eq!(comment.content, new_comment);
		});
	}

	#[test]
	fn delete_comment_works() {
		new_test_ext().execute_with(|| {
			let title = b"Test Title".to_vec();
			let content = b"Test Content".to_vec();

			// Tạo bài viết
			assert_ok!(Blog::create_post(
				RuntimeOrigin::signed(1),
				title.clone(),
				content.clone()
			));

			// Tạo bình luận
			let comment_content = b"Comment to delete".to_vec();
			assert_ok!(Blog::create_comment(
				RuntimeOrigin::signed(2),
				0,
				comment_content.clone()
			));

			// Xóa bình luận
			assert_ok!(Blog::delete_comment(RuntimeOrigin::signed(2), 0));

			// Kiểm tra đã được đánh dấu xóa
			let comment = Blog::comments(0).unwrap();
			assert_eq!(comment.is_deleted, true);
		});
	}

	#[test]
	fn title_too_long_fails() {
		new_test_ext().execute_with(|| {
			let title = vec![0u8; 201]; // Vượt quá MaxTitleLength (200)
			let content = b"Content".to_vec();

			assert_noop!(
				Blog::create_post(
					RuntimeOrigin::signed(1),
					title,
					content
				),
				Error::<Test>::TitleTooLong
			);
		});
	}

	#[test]
	fn content_too_long_fails() {
		new_test_ext().execute_with(|| {
			let title = b"Title".to_vec();
			let content = vec![0u8; 10_001]; // Vượt quá MaxContentLength (10_000)

			assert_noop!(
				Blog::create_post(
					RuntimeOrigin::signed(1),
					title,
					content
				),
				Error::<Test>::ContentTooLong
			);
		});
	}
}
