enum Message {
    Hello {id: i32}
}

fn main() {
    let msg = Message::Hello {id: 5};

    match msg {
        // 该模式要求匿名结构体的id字段的值在3到7之间 若匹配成功 则将匹配到的id字段值绑定到id_variable变量上
        Message::Hello {id: id_variable @ 3 ..= 7} => {
            println!("Found an id in range: {}", id_variable)
        },

        // 该模式要求匿名结构体的id字段的值在10到12之间 但是没有绑定值到变量上 因此该分支内无法访问到id字段的值
        Message::Hello {id: 10 ..= 12} => {
            println!("Found an id in another range")
        },

        // 该模式对id字段的值没有任何限制 仅将匹配到的id字段值绑定到变量id上
        Message::Hello {id} => {
            println!("Found some other id: {}", id)
        }
    }
}
