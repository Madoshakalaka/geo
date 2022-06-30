use std::{
    borrow::Borrow,
    cmp::Ordering,
    collections::BTreeSet,
    fmt::Debug,
    ops::{Bound, Deref},
};

/// A segment currently active in the sweep.
///
/// As the sweep-line progresses from left to right, it intersects a subset of
/// the line-segments. These can be totally-ordered from bottom to top, and
/// efficient access to the neighbors of a segment is a key aspect of
/// planar-sweep algorithms.
///
/// We assert `Ord` even though the inner-type is typically only `T:
/// PartialOrd`. It is a logical error to compare two Active which cannot be
/// compared. This is ensured by the algorithm (and cannot be inferred by the
/// compiler?).
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub(super) struct Active<T>(pub(super) T);

impl<T> Active<T> {
    pub(super) fn active_ref(t: &T) -> &Active<T> {
        unsafe { std::mem::transmute(t) }
    }
}

impl<T> Borrow<T> for Active<T> {
    fn borrow(&self) -> &T {
        &self.0
    }
}

impl<T> Deref for Active<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Assert total equality.
impl<T: PartialEq> Eq for Active<T> {}

/// Assert total ordering of active segments.
impl<T: PartialOrd> Ord for Active<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        T::partial_cmp(self, other).unwrap_or(Ordering::Equal)
    }
}

impl<T: PartialOrd> PartialOrd for Active<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Trait abstracting a container of active segments.
pub(super) trait ActiveSet: Default {
    type Seg;
    fn previous(&self, segment: &Self::Seg) -> Option<&Active<Self::Seg>>;
    fn next(&self, segment: &Self::Seg) -> Option<&Active<Self::Seg>>;
    fn insert_active(&mut self, segment: Self::Seg);
    fn remove_active(&mut self, segment: &Self::Seg);
}

impl<T: PartialOrd> ActiveSet for BTreeSet<Active<T>> {
    type Seg = T;

    fn previous(&self, segment: &Self::Seg) -> Option<&Active<Self::Seg>> {
        self.range::<Active<_>, _>((
            Bound::Unbounded,
            Bound::Excluded(Active::active_ref(segment)),
        ))
        .next_back()
    }

    fn next(&self, segment: &Self::Seg) -> Option<&Active<Self::Seg>> {
        self.range::<Active<_>, _>((
            Bound::Excluded(Active::active_ref(segment)),
            Bound::Unbounded,
        ))
        .next()
    }

    fn insert_active(&mut self, segment: Self::Seg) {
        let result = self.insert(Active(segment));
        debug_assert!(result);
    }

    fn remove_active(&mut self, segment: &Self::Seg) {
        self.remove(Active::active_ref(segment));
    }
}
