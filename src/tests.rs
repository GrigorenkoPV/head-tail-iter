use super::*;

const EMPTY_SLICE: &[i32] = &[];

#[test]
fn size_hint_and_exact_size() {
    let size_hint = |n| (n, Some(n));
    let mut iter = [1, 2, 3].head_tail_pairs();
    assert_eq!(iter.size_hint(), size_hint(3));
    assert!(iter.next().is_some());
    assert_eq!(iter.size_hint(), size_hint(2));
    assert!(iter.next().is_some());
    assert_eq!(iter.size_hint(), size_hint(1));
    assert!(iter.next().is_some());
    assert_eq!(iter.size_hint(), size_hint(0));
    assert!(iter.next().is_none());
    assert_eq!(EMPTY_SLICE.head_tail_pairs().size_hint(), size_hint(0));
}

#[test]
fn count() {
    #[track_caller]
    fn check_count(slice: &[i32]) {
        let iter = slice.head_tail_pairs();
        assert_eq!(iter.count(), slice.len());
    }

    check_count(&[]);
    check_count(&[1, 2, 3]);
}

#[test]
fn last() {
    assert_eq!([1, 2, 3].head_tail_pairs().last(), Some((&3, EMPTY_SLICE)));
    assert_eq!(EMPTY_SLICE.head_tail_pairs().last(), None);
}

#[test]
#[allow(clippy::iter_nth_zero)]
fn nth() {
    let iter = || [1, 2, 3].head_tail_pairs();
    assert_eq!(iter().nth(0), iter().next());
    assert_eq!(iter().nth(1), Some((&2, &[3] as &[_])));
    assert_eq!(iter().nth(2), Some((&3, &[] as &[_])));
    assert_eq!(iter().nth(3), None);
    assert_eq!(iter().nth(10), None);

    let iter = || -> HeadTailIter<'static, ()> { [].head_tail_pairs() };
    assert_eq!(iter().nth(0), iter().next());
    assert_eq!(iter().nth(0), None);
    assert_eq!(iter().nth(1), None);
    assert_eq!(iter().nth(2), None);
    assert_eq!(iter().nth(3), None);
    assert_eq!(iter().nth(10), None);

    let mut iter = [1, 2, 3, 4, 5, 6].head_tail_pairs();
    assert_eq!(iter.nth(2), Some((&3, &[4, 5, 6] as &[_])));
    assert_eq!(iter.next(), Some((&4, &[5, 6] as &[_])));
    assert_eq!(iter.nth(10), None);
    assert_eq!(iter.next(), None);
}

#[test]
fn len() {
    let mut iter = [1, 2, 3].head_tail_pairs();
    assert_eq!(iter.len(), 3);
    assert!(iter.next().is_some());
    assert_eq!(iter.len(), 2);
    assert!(iter.next().is_some());
    assert_eq!(iter.len(), 1);
    assert!(iter.next().is_some());
    assert_eq!(iter.len(), 0);
    assert!(iter.next().is_none());
    assert_eq!(EMPTY_SLICE.head_tail_pairs().len(), 0);
}

#[test]
fn head_tail_pairs() {
    let HeadTailIter { tail } = [1, 2, 3].head_tail_pairs();
    assert_eq!(tail, &[1, 2, 3]);
}

#[test]
fn from() {
    let slice: &[i32] = &[1, 2, 3];
    let HeadTailIter { tail } = slice.into();
    assert_eq!(tail, slice);
}

#[test]
fn fused() {
    let mut iter = [1, 2, 3].head_tail_pairs();
    let _ = iter.next();
    let _ = iter.next();
    let _ = iter.next();
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    iter = [].head_tail_pairs();
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
    assert_eq!(iter.next(), None);
}
