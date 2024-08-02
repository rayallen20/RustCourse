use blog_example::Post;

fn main() {
    let mut post = Post::new();

    // 草稿状态下的博文内容应为空
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    // 待审批状态下的博文内容应为空
    post.request_review();
    assert_eq!("", post.content());

    // 已发布状态下的博文内容应为"I ate a salad for lunch today"
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}