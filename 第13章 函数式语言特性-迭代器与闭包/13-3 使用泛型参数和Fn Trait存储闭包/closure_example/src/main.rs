use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

// T表示闭包的类型
struct Cacher<T, E>
    where T: Fn(E) -> E,
          E: Display + Copy + Eq + Hash
{
    calculation: T,
    result_collection: HashMap<E, E>,
}

impl <T, E> Cacher<T, E>
where T: Fn(E) -> E,
    E: Display + Copy + Eq + Hash
{
    fn new(calculation: T) -> Cacher<T, E> {
        Cacher {
            calculation,
            result_collection: HashMap::new(),
        }
    }

    fn value(&mut self, arg: E) -> E {
        if self.result_collection.contains_key(&arg) {
            self.result_collection.get(&arg).unwrap().clone()
        } else {
            let v = (self.calculation)(arg);
            self.result_collection.insert(arg, v);
            v
        }
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// 本函数用于模拟生成健身计划的过程
fn generate_workout(intensity: u32, random_number: u32) {

    // 本闭包用于模拟一个耗时的计算过程
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    let mut cacher :Cacher<fn(u32) -> u32, u32> = Cacher::new(expensive_closure);

    if intensity < 25 {
        println!("Today, do {} pushups!", cacher.value(intensity));
        println!("Next, do {} situps!", cacher.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", cacher.value(intensity));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
        let v1 = c.value(1);
        assert_eq!(v1, 1);

        let v2 = c.value(2);
        assert_eq!(v2, 2)
    }

    #[test]
    fn call_with_genericity_values() {
        let mut c = Cacher::new(|a :u32| -> u32 {a});
        let v1 = c.value(1);
        assert_eq!(v1, 1);

        let mut c2 = Cacher::new(|a :i64| -> i64 {a});
        let v2 = c2.value(2);
        assert_eq!(v2, 2);
    }
}