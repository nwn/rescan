use std::mem::MaybeUninit;

pub struct PartialResult<T: MaybeUninitInner> {
    /// Bitset of initialized elements.
    initialized: u16,
    /// Collection of MaybeUninit elements.
    result: T::Inner,
}

impl<T: MaybeUninitInner> PartialResult<T> {
    /// Create an totally uninitialized PartialResult.
    pub fn empty() -> Self {
        Self {
            initialized: 0,
            result: T::uninit(),
        }
    }

    /// Take the full result if it has been entirely initialized, leaving it empty.
    /// Otherwise, any initialized elements will remain initialized.
    pub fn take(&mut self) -> Option<T> {
        self.is_fully_initialized().then(||
            // SAFETY: `self.is_fully_initialized` guarantees all elements have been initialized.
            unsafe {
                self.set_fully_uninitialized();
                std::mem::transmute_copy(&self.result)
            }
        )
    }

    /// Return `true` iff the specified element has been initialized.
    pub fn is_initialized(&self, elem: u32) -> bool {
        (self.initialized & (1 << elem)) != 0
    }

    /// Return `true` iff all elements have been initialized.
    pub fn is_fully_initialized(&self) -> bool {
        let mask = if T::NUM_ELEMS < u16::BITS as usize {
            (1 << T::NUM_ELEMS) - 1
        } else {
            u16::MAX
        };
        self.initialized == mask
    }

    /// Mark the specified element as being initialized.
    unsafe fn set_initialized(&mut self, elem: u32) {
        self.initialized |= 1 << elem
    }

    /// Mark the specified element as being uninitialized.
    pub fn set_uninitialized(&mut self, elem: u32) {
        self.initialized &= !(1 << elem)
    }

    /// Mark the specified element as being uninitialized.
    fn set_fully_uninitialized(&mut self) {
        self.initialized = 0
    }
}

impl PartialResult<()> { }

// TODO: How do we generate these impls using a macro? It's hard, since we can't
// use the `concat_idents!()` macro when defining new functions.
impl<T0> PartialResult<(T0,)> {
    /// Returns a reference to the 0th element if it is initialized, or `None` otherwise.
    pub fn get_0(&self) -> Option<&T0> {
        self.is_initialized(0).then(|| {
            // SAFETY: `self.is_initialized` guarantees this element has been initialized.
            unsafe {
                self.result.0.assume_init_ref()
            }
        })
    }

    /// Returns a mutable reference to the 0th element if it is initialized, or `None` otherwise.
    pub fn get_0_mut(&mut self) -> Option<&mut T0> {
        self.is_initialized(0).then(|| {
            // SAFETY: `self.is_initialized` guarantees this element has been initialized.
            unsafe {
                self.result.0.assume_init_mut()
            }
        })
    }

    /// Takes the 0th element if it is initialized, leaving it uninitialized.
    pub fn take_0(&mut self) -> Option<T0> {
        self.is_initialized(0).then(|| {
            self.set_uninitialized(0);
            // SAFETY: `self.is_initialized` guarantees this element has been initialized.
            // Moreover, we mark the element as uninitialized to prevent duplicate reads.
            unsafe {
                // Workaround until `assume_init_read` is stabilized.
                self.result.0.as_ptr().read()
            }
        })
    }

    /// Initializes the 0th element with the given value.
    pub fn set_0(&mut self, val: T0) -> &mut T0 {
        // SAFETY: We initialize this element immediately.
        unsafe { self.set_initialized(0); }
        self.result.0.write(val)
    }
}

impl<T0, T1> PartialResult<(T0, T1)> {
    /// Returns a reference to the 0th element if it is initialized, or `None` otherwise.
    pub fn get_0(&self) -> Option<&T0> {
        self.is_initialized(0).then(|| {
            // SAFETY: `self.is_initialized` guarantees this element has been initialized.
            unsafe {
                self.result.0.assume_init_ref()
            }
        })
    }

    /// Returns a mutable reference to the 0th element if it is initialized, or `None` otherwise.
    pub fn get_0_mut(&mut self) -> Option<&mut T0> {
        self.is_initialized(0).then(|| {
            // SAFETY: `self.is_initialized` guarantees this element has been initialized.
            unsafe {
                self.result.0.assume_init_mut()
            }
        })
    }

    /// Takes the 0th element if it is initialized, leaving it uninitialized.
    pub fn take_0(&mut self) -> Option<T0> {
        self.is_initialized(0).then(|| {
            self.set_uninitialized(0);
            // SAFETY: `self.is_initialized` guarantees this element has been initialized.
            // Moreover, we mark the element as uninitialized to prevent duplicate reads.
            unsafe {
                // Workaround until `assume_init_read` is stabilized.
                self.result.0.as_ptr().read()
            }
        })
    }

    /// Initializes the 0th element with the given value.
    pub fn set_0(&mut self, val: T0) -> &mut T0 {
        // SAFETY: We initialize this element immediately.
        unsafe { self.set_initialized(0); }
        self.result.0.write(val)
    }
}



/// Maps a type to an ABI-equivalent type that can be safely assumed to always
/// be initialized.
///
/// To safely implement this trait, you *must* guarantee that the following
/// expression is safe:
///
/// ```rust,no_run
/// MaybeUninit<Inner>::uninit().assume_init()
/// ```
pub trait MaybeUninitInner {
    type Inner;
    const NUM_ELEMS: usize;

    fn uninit() -> Self::Inner {
        unsafe {
            MaybeUninit::uninit().assume_init()
        }
    }
}

macro_rules! impl_maybe_uninit_inner_for_tuple {
    ($len:expr; ($($ty:ident,)*)) => {
        impl<$($ty),*> MaybeUninitInner for ($($ty,)*) {
            const NUM_ELEMS: usize = $len;
            type Inner = ($(MaybeUninit<$ty>,)*);
        }
    }
}
impl_maybe_uninit_inner_for_tuple!{0; ()}
impl_maybe_uninit_inner_for_tuple!{1; (T0,)}
impl_maybe_uninit_inner_for_tuple!{2; (T0, T1,)}
impl_maybe_uninit_inner_for_tuple!{3; (T0, T1, T2,)}
impl_maybe_uninit_inner_for_tuple!{4; (T0, T1, T2, T3,)}
impl_maybe_uninit_inner_for_tuple!{5; (T0, T1, T2, T3, T4,)}
impl_maybe_uninit_inner_for_tuple!{6; (T0, T1, T2, T3, T4, T5,)}
impl_maybe_uninit_inner_for_tuple!{7; (T0, T1, T2, T3, T4, T5, T6,)}
impl_maybe_uninit_inner_for_tuple!{8; (T0, T1, T2, T3, T4, T5, T6, T7,)}
impl_maybe_uninit_inner_for_tuple!{9; (T0, T1, T2, T3, T4, T5, T6, T7, T8,)}
impl_maybe_uninit_inner_for_tuple!{10; (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9,)}
impl_maybe_uninit_inner_for_tuple!{11; (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10,)}
impl_maybe_uninit_inner_for_tuple!{12; (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11,)}
impl_maybe_uninit_inner_for_tuple!{13; (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12,)}
impl_maybe_uninit_inner_for_tuple!{14; (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13,)}
impl_maybe_uninit_inner_for_tuple!{15; (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14,)}
impl_maybe_uninit_inner_for_tuple!{16; (T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15,)}
