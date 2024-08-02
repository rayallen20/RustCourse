/// 本结构体表示一个已发布的博文
pub struct Post {
    content: String
}

impl Post {
    /// 本方法用于创建一个草稿状态的博文
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new()
        }
    }

    /// 本方法用于获取博文内容
    pub fn content(&self) -> &str {
        &self.content
    }
}

/// 本结构体表示一个草稿状态的博文
pub struct DraftPost {
    content: String
}

impl DraftPost {
    /// 本方法用于向草稿中添加文本
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// 本方法用于申请审批博文
    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content
        }
    }
}

/// 本结构体表示一个待审批状态的博文
pub struct PendingReviewPost {
    content: String
}

impl PendingReviewPost {
    /// 本方法用于审批通过博文
    pub fn approve(self) -> Post {
        Post {
            content: self.content
        }
    }
}