/// 本结构体用于存储博文的内容与状态
pub struct Post {
    /// 博文状态
    state: Option<Box<dyn State>>,
    /// 博文内容
    content: String,
}

impl Post {
    /// 本关联函数用于创建一个新的博文
    pub fn new() -> Post {
        Post {
            // 新创建的博文初始状态为草稿状态
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    /// 本方法用于存储博文的内容
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    /// 本方法用于获取博文的内容
    pub fn content(&self) -> &str {
        // Option.as_ref()方法会返回Option中的值的引用
        // 本例中 self.state.as_ref() 返回的是Option<&Box<dyn State>>
        // 而State Trait中定义的content()方法的签名为:
        // fn content<'a> (&self, post: &'a Post) -> &'a str
        // 因此只需要State特征对象的引用即可调用该方法
        self.state.as_ref().unwrap().content(self)
    }

    /// 本方法用于审批博文并改变博文的状态
    pub fn request_review(&mut self) {
        // Option.take()方法会取出原Option中的值,并将原Option置为None
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
        }
    }

    /// 本方法用于审批通过博文并改变博文的状态
    pub fn approve(&mut self) {
        if let Some(state) = self.state.take() {
            self.state = Some(state.approve());
        }
    }
}

/// 本trait用于定义博文状态的行为
trait State {
    /// 本方法用于申请审批 从而实现状态的转换
    /// Self: 表示实现了该Trait的类型
    /// Box<Self>: 表示该方法只能被 Box<当前类型> 的类型调用 无法被当前类型直接调用
    /// 而如果把这里改成 Box<dyn State>,则表示这个Box指向的是一个实现了State Trait的类型
    /// 二者的区别在于: 在调用时, Box<Self> 可以直接调用当前类型的方法,而 Box<dyn State> 则不行
    /// 因为 Box<dyn State> 只是一个实现了State Trait的类型,而 Box<Self> 则是当前类型的一个实例
    fn request_review(self: Box<Self>) -> Box<dyn State>;

    /// 本方法用于通过审批 针对草稿状态和已审批状态 直接返回自身即可
    /// 待审批状态需覆写此方法
    fn approve(self: Box<Self>) -> Box<dyn State>;

    /// 本方法用于显示当前状态下应显示的博文内容 针对草稿状态和待审批状态 直接返回空字符串即可
    /// 已发布状态需覆写此方法
    fn content<'a> (&self, _post: &'a Post) -> &'a str {
        ""
    }
}

/// 本结构体表示博文的草稿状态
struct Draft {}

impl Draft {
    fn some_specific_draft_function(&self) {
        println!("This is a specific function for Draft");
    }
}

impl State for Draft {
    /// 本方法用于草稿状态的申请审批 即:返回一个待审批的状态对象
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        // 此处若Trait定义的方法签名类型为Box<dyn State> 则无法调用Draft具型的方法
        self.some_specific_draft_function();
        Box::new(PendingReview {})
    }

    /// 本方法用于草稿状态的审批通过 但草稿状态无法被直接审批通过 所以直接返回自身即可
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

/// 本结构体表示博文的待审核状态
struct PendingReview {}

impl State for PendingReview {
    /// 本方法用于待审核状态的申请审批 此处暂时不做任何处理 直接返回自身
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    /// 本方法用于待审核状态的审批通过 即:返回一个已审批的状态对象
    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

/// 本结构体表示博文的已发布状态
struct Published {}

impl State for Published {
    /// 本方法用于已发布状态的申请审批 已审批状态无需再次审批 直接返回自身即可
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    /// 本方法用于已发布状态的审批通过 已审批状态无需再次审批 直接返回自身即可
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    /// 本方法用于显示已发布状态下的博文内容
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}