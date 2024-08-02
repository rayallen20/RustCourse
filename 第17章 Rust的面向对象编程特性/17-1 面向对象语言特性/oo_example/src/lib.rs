/// 本结构体用于维护(添加/删除)一个i32类型的集合,并在集合每次发生变化时更新平均值
pub struct AveragedCollection {
    /// 本字段用于存储所有的数字
    list: Vec<i32>,
    /// 本字段用于存储所有数字的平均值 避免每次读取平均值时都重新计算
    average: f64,
}

impl AveragedCollection {
    /// 本方法用于根据结构体实例中当前的数字集合计算平均值
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

    /// 本方法用于向集合中添加一个数字,并更新平均值
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    /// 本方法用于删除集合末尾的数字,并更新平均值
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    /// 本方法用于获取当前集合的平均值
    pub fn average(&self) -> f64 {
        self.average
    }
}