// 字典树模板
struct Tire<K, T> {
    data: T,
    child: std::collections::HashMap<K, Box<Tire>>, // 看情况
}
