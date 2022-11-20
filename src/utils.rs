use crate::virtual_node::VirtualNode;

pub(crate) fn get_hash(input: &str) -> String {
    md5::compute(input)
        .iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("-")
}

// TODO: make it generic over T if possible.
pub(crate) fn cyclic_binary_search<'a>(items: &'a [VirtualNode], target: &'_ str) -> Option<usize> {
    let n = items.len();
    let mut left = 0;
    let mut right = n - 1;
    let mut mid = (left + right) / 2;

    loop {
        if left > right {
            break;
        }
        mid = (left + right) / 2;
        let mid_value = items.get(mid).unwrap();

        if mid_value.hash == target {
            return Some(mid);
        }
        if target > &mid_value.hash {
            left = mid + 1;
        } else {
            if mid == 0 {
                break;
            }
            right = mid - 1;
        }
    }

    if mid == 0 {
        // - target is smaller than the smallest item.
        return Some(0);
    } else if left > n - 1 {
        return Some(0);
    } else {
        // let left_value = *items.get(left).unwrap();
        // let right_value = *items.get(right).unwrap();

        if items.get(right).unwrap().hash.as_str() > target {
            return Some(right);
        }
        return Some(left);
    }
}
