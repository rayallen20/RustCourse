# PART1. 状态模式

- 状态模式(state pattern)是一种面向对象设计模式
  - 一个值拥有的内部状态由数个对象(state object)表达而成,而值的行为则随着内部状态的改变而改变
- 使用状态模式意味着:
  - 业务需求变化时,不需要修改持有状态的值的代码,或者使用这个值的代码
  - 只需要更新状态对象内部的代码,改变其规则即可;或者增加一些新的状态对象

# PART2. 示例

本例中,我们实现一个发布博客的工作流程,其中:

1. 在新建博客文章时生成一份空白的草稿文档
2. 在草稿撰写完毕后,请求对这篇草稿状态的文章进行审批
3. 在文章通过审批后正式对外发布
4. 仅返回并打印成功发布后的文章,而不能意外地发布没有通过审批的文章

```
tree ./
./
├── Cargo.lock
├── Cargo.toml
└── src
    └── lib.rs

1 directory, 3 files
```

## 2.1 定义`Post`结构体与新建博文的过程

`lib.rs`:

```rust
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
}

/// 本trait用于定义博文状态的行为
trait State {}

/// 本结构体表示博文的草稿状态
struct Draft {}

impl State for Draft {}
```

这里稍后解释为什么`state`字段的类型需要套一层`Option`

## 2.2 存储文章内容

此时我们的`Post`仍然只有一个状态:草稿状态.我们需要添加一个方法用于存储文章内容

`lib.rs`:

```rust
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
}

/// 本trait用于定义博文状态的行为
trait State {}

/// 本结构体表示博文的草稿状态
struct Draft {}

impl State for Draft {}
```

## 2.3 确保草稿的可读内容为空

对于一篇处于草稿状态的博文而言,即使调用其`add_text()`方法添加了内容,这些内容也不应该被返回

这一步我们实现一个`content()`方法,用于返回博文的内容,但是对于处于草稿状态的博文,其内容应该为空

此时`Post`只有一个状态:草稿状态,因此直接返回一个空字符串即可

`lib.rs`:

```rust
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
        ""
    }
}

/// 本trait用于定义博文状态的行为
trait State {}

/// 本结构体表示博文的草稿状态
struct Draft {}

impl State for Draft {}
```

## 2.4 请求审批文章并改变其状态

我们将待审批状态定义为`PendingReview`,并在`State` Trait中定义`request_review()`方法,该方法用于请求审批

- 对于草稿状态的博文,调用`request_review()`方法后,博文的状态应该变为待审批状态
- 对于待审批状态的博文,调用`request_review()`方法后,博文的状态应该保持不变
- 该方法应该由`Post`结构体调用

### 2.4.1 定义`PendingReview`状态

`lib.rs`:

```rust
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
        ""
    }
}

/// 本trait用于定义博文状态的行为
trait State {}

/// 本结构体表示博文的草稿状态
struct Draft {}

impl State for Draft {}

/// 本结构体表示博文的待审核状态
struct PendingReview {}

impl State for PendingReview {}
```

### 2.4.2 定义并实现`request_review()`方法

`lib.rs`:

```rust
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
        ""
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
}

/// 本结构体表示博文的待审核状态
struct PendingReview {}

impl State for PendingReview {
    /// 本方法用于待审核状态的申请审批 此处暂时不做任何处理 直接返回自身
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

这一步要注意`Box<Self>`与`Box<dyn State>`的区别:

- `Box<Self>`表示当前类型的一个实例,在具型的实现中可以调用当前类型的方法
- `Box<dyn State>`表示实现了`State` Trait的一个类型,在具型的实现中无法调用具型的方法

### 2.4.3 实现`Post`结构体的`request_review()`方法

`lib.rs`:

```rust
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
        ""
    }

    /// 本方法用于审批博文并改变博文的状态
    pub fn request_review(&mut self) {
        // Option.take()方法会取出原Option中的值,并将原Option置为None
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
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
}

/// 本结构体表示博文的待审核状态
struct PendingReview {}

impl State for PendingReview {
    /// 本方法用于待审核状态的申请审批 此处暂时不做任何处理 直接返回自身
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
}
```

这里就是为什么我们在定义`state`字段时套了一层`Option`的原因:

- Rust不允许结构体的字段值为一个未被填充的值
- 但是,可以使用`Option.take()`方法取出`Option`中的值,并将该字段的值置为`None`变体
  - 这样,既达到了在状态转换期间将状态置为未被填充的目的
  - 又获得了当前的状态
  - 更直白的理解是,套了一层`Option`,使得我们在状态转换时,可以将当前状态取走而非借用当前状态

## 2.5 添加通过审批的`approve()`方法并修改`Post`结构体的`content()`方法

`approve()`方法用于将待审批状态的博文转换为已发布状态.该方法的实现思路和`request_review()`方法类似

### 2.5.1 为`State` Trait添加`approve()`方法

`lib.rs`:

```rust
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
        ""
    }

    /// 本方法用于审批博文并改变博文的状态
    pub fn request_review(&mut self) {
        // Option.take()方法会取出原Option中的值,并将原Option置为None
        if let Some(state) = self.state.take() {
            self.state = Some(state.request_review());
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
}
```

### 2.5.2 实现`Post`结构体的`approve()`方法

```rust
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
        ""
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
}
```

### 2.5.3 为`State` Trait定义`content()`方法

由于只需要在状态为`Published`时返回博文内容,而其他状态下不需要返回内容,因此我们在`State` Trait中实现一个默认的`content()`方法

然后`Published`结构体下覆写该方法即可

最终在`Post`结构体的`content()`方法中调用`state`字段的`content()`方法即可

```rust
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
        ""
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
    fn content<'a> (&self, post: &'a Post) -> &'a str {
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
```

### 2.5.4 修改`Post`结构体的`content()`方法

```rust
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
    fn content<'a> (&self, post: &'a Post) -> &'a str {
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
```

# PART3. 使用状态模式

`main.rs`:

```rust
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
```

```
cargo run
   Compiling blog_example v0.1.0 (/blog_example)
warning: unused variable: `post`
  --> src/lib.rs:66:28
   |
66 |     fn content<'a> (&self, post: &'a Post) -> &'a str {
   |                            ^^^^ help: if this is intentional, prefix it with an underscore: `_post`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `blog_example` (lib) generated 1 warning
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.16s
     Running `target/debug/blog_example`
This is a specific function for Draft
```

# PART4. 状态模式的取舍权衡

- 缺点:
  - 某些状态之间是相互耦合的
    - 如果我们现在新增了一个状态,那么和该状态相关联的状态,都需要进行修改
    - 需要重复实现一些逻辑代码
      - 比如`Draft`状态和`Published`的`approve()`方法的代码是完全相同的
      - 但是这里是没法把这个方法写成`State` Trait的默认方法的,因为该方法在`State` Trait中无法确定返回值类型的大小

# PART5. 将状态和行为编码为类型

这次我们将状态完全编码为不同的类型,而非完全封装状态和状态转移的过程

这里我们仍然希望通过`Post::new()`的方式来创建一个草稿状态的博文,但是我们不再为草稿状态的博文提供`content()`方法

这样用户在尝试读取草稿状态的博文内容时,会直接得到一个编译错误.这样就避免了用户在草稿状态下能够读取博文内容的情况

## 5.1 定义`Post`结构体和`DraftPost`结构体

```rust
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
}
```

注意:此时`Post`结构体在被创建之后,返回的是一个`DraftPost`结构体,而非`Post`结构体

## 5.2 将状态转移实现为不同类型之间的转换

- 我们依然希望草稿状态的文章能够在审批后发布
- 同样的,一篇待审批状态的文章也不应该对外显示任何内容
- 因此,我们可以创建一个新的结构体:`PendingReviewPost`来表示待审批状态的文章

```rust
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
```

可以看到,这种思路是靠类型来区分状态的,而不是通过状态字段来区分状态.这样其实比较符合Rust的思路,因为Rust是一门静态类型语言,因此我们可以通过类型来区分状态

而且也省去了重复的代码

```rust
use blog_example_another::Post;

fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");

    let post = post.request_review();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());
}
```

# PART6. 总结

- Rust不仅能够实现面向对象的设计模式,还可以支持更多的模式
  - 例如:将状态和行为编码为类型
- 面向对象的经典模式在Rust编程中,并不总是最佳选择,因为Rust具有所有权等其他面向对象语言没有的特性