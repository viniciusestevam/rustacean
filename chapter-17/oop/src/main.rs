use blog::{v1, v2};

fn main() {
  let mut post_v1 = v1::Post::new();

  post_v1.add_text("I ate a salad for lunch today");
  assert_eq!("", post_v1.content());

  post_v1.request_review();
  assert_eq!("", post_v1.content());

  post_v1.approve();
  assert_eq!("I ate a salad for lunch today", post_v1.content());

  //
  //
  //

  let mut post_v2 = v2::Post::new();

  post_v2.add_text("I ate a salad for lunch today");

  let post_v2 = post_v2.request_review();

  let post_v2 = post_v2.approve();
  let post_v2 = post_v2.approve();

  assert_eq!("I ate a salad for lunch today", post_v2.content());
}
