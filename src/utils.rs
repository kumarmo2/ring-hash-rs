use std::borrow::Borrow;

pub(crate) fn get_hash(input: &str) -> String {
    md5::compute(input)
        .iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("-")
}

pub(crate) fn cyclic_binary_search<Owned, Borrowed>(
    items: &[Owned],
    target: &Borrowed,
) -> Option<usize>
where
    Owned: Borrow<Borrowed>,
    Borrowed: Ord + ?Sized,
{
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

        if *mid_value.borrow() == *target {
            return Some(mid);
        }
        if *target > *mid_value.borrow() {
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
        if *items.get(right).unwrap().borrow() > *target {
            return Some(right);
        }
        return Some(left);
    }
}
