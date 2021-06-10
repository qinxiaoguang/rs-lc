use super::Solution;

impl Solution {
    pub fn get_folder_names(names: Vec<String>) -> Vec<String> {
        // 一个set一个map,set保存这个name是否被使用，map保存这个name的id数
        let mut set = std::collections::HashSet::new();
        let mut map = std::collections::HashMap::new();

        let mut res = vec![String::from(""); names.len()];
        for (i, name) in names.into_iter().enumerate() {
            if set.contains(&name) {
                // 后缀名被占用，需要append 一个cnt
                loop {
                    let mut cnt = map.entry(name.clone()).or_insert(0);
                    *cnt += 1;
                    let new_name = format!("{}({})", name.clone(), *cnt);
                    if set.contains(&new_name) {
                        continue;
                    }
                    set.insert(new_name.clone());
                    res[i] = new_name;
                    break;
                }
            } else {
                set.insert(name.clone());
                res[i] = name
            }
        }

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1487() {
        assert_eq!(
            Solution::get_folder_names(sv!["pes", "fifa", "gta", "pes(2019)"]),
            sv!["pes", "fifa", "gta", "pes(2019)"],
        );

        assert_eq!(
            Solution::get_folder_names(sv!["gta", "gta(1)", "gta", "avalon"]),
            sv!["gta", "gta(1)", "gta(2)", "avalon"]
        );

        assert_eq!(
            Solution::get_folder_names(sv![
                "onepiece",
                "onepiece(1)",
                "onepiece(2)",
                "onepiece(3)",
                "onepiece"
            ]),
            sv![
                "onepiece",
                "onepiece(1)",
                "onepiece(2)",
                "onepiece(3)",
                "onepiece(4)"
            ]
        );

        assert_eq!(
            Solution::get_folder_names(sv!["wano", "wano", "wano", "wano"]),
            sv!["wano", "wano(1)", "wano(2)", "wano(3)"]
        );

        assert_eq!(
            Solution::get_folder_names(sv!["kaido", "kaido(1)", "kaido", "kaido(1)"]),
            sv!["kaido", "kaido(1)", "kaido(2)", "kaido(1)(1)"]
        );
    }
}
