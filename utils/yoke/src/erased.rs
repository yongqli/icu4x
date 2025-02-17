// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains helper types for erasing Cart types.
//!
//! See the docs of [`Yoke::erase_rc_cart()`](crate::Yoke::erase_rc_cart)
//! and [`Yoke::erase_box_cart()`](crate::Yoke::erase_box_cart) for more info.
//!
//! Available with the `"alloc"` feature enabled.

use alloc::boxed::Box;
use alloc::rc::Rc;
use alloc::sync::Arc;

/// Dummy trait that lets us `dyn Drop`
///
/// `dyn Drop` isn't legal (and doesn't make sense since `Drop` is not
/// implement on all destructible types). However, all trait objects come with
/// a destructor, so we can just use an empty trait to get a destructor object.
pub trait ErasedDestructor: 'static {}
impl<T: 'static> ErasedDestructor for T {}

/// A type-erased Cart that has `Rc` semantics
///
/// See the docs of [`Yoke::erase_rc_cart()`](crate::Yoke::erase_rc_cart) for more info.
///
/// Available with the `"alloc"` feature enabled.
pub type ErasedRcCart = Rc<dyn ErasedDestructor>;
/// A type-erased Cart that has `Arc` semantics
///
/// See the docs of [`Yoke::erase_arc_cart()`](crate::Yoke::erase_arc_cart) for more info.
///
/// Available with the `"alloc"` feature enabled.
pub type ErasedArcCart = Arc<dyn ErasedDestructor>;
/// A type-erased Cart that has `Box` semantics
///
/// See the docs of [`Yoke::erase_box_cart()`](crate::Yoke::erase_box_cart) for more info.
///
/// Available with the `"alloc"` feature enabled.
pub type ErasedBoxCart = Box<dyn ErasedDestructor>;
