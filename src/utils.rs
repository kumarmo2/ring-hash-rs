// use md5::Digest;
use std::borrow::Borrow;

pub(crate) fn get_hash<T>(input: &T) -> String
where
    T: AsRef<[u8]> + ?Sized,
{
    md5::compute(input)
        .iter()
        .map(|b| format!("{:02x}", b).to_string())
        .collect::<Vec<String>>()
        .join("-")
}

// pub(crate) fn get_hash<T>(input: &T) -> String
// where
// {
// md5::compute()
// .iter()
// .map(|b| format!("{:02x}", b).to_string())
// .collect::<Vec<String>>()
// .join("-")
// }

// pub(crate) fn get_hash(input: &str) -> String {
// md5::compute(input)
// .iter()
// .map(|b| format!("{:02x}", b).to_string())
// .collect::<Vec<String>>()
// .join("-")
// }

pub(crate) fn binary_search<Owned, Borrowed>(items: &[Owned], target: &Borrowed) -> Option<usize>
where
    Owned: Borrow<Borrowed>,
    Borrowed: Ord + ?Sized,
{
    let n = items.len();
    if n == 0 {
        return None;
    }

    let mut left = 0;
    let mut right = n - 1;

    loop {
        if left > right {
            return None;
        }
        let mid = (left + right) / 2;
        // This unwrap is safe.
        let value = items.get(mid).unwrap();

        if *value.borrow() == *target {
            return Some(mid);
        }
        if *target > *value.borrow() {
            left = mid + 1;
        } else {
            if mid == 0 {
                return None;
            }
            right = mid - 1
        }
    }
}
pub(crate) fn cyclic_binary_search<Owned, Borrowed>(items: &[Owned], target: &Borrowed) -> usize
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
            return mid;
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
        return 0;
    } else if left > n - 1 {
        return 0;
    } else {
        if *items.get(right).unwrap().borrow() > *target {
            return right;
        }
        return left;
    }
}
