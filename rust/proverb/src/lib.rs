pub fn build_proverb(list: &[&str]) -> String {
    let mut result = String::from("");

    if list.len() == 0 {
        return result;
    }

    for (i, &item) in list.iter().enumerate() {
        if i + 1 != list.len() {
            result += &format!("For want of a {} the {} was lost.\n", item, list[i+1]);
        } else {
            break;
        }
    }

    result += &format!("And all for the want of a {}.", list[0]);
    result
}
