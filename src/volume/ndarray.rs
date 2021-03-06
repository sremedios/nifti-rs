//! Interfaces and implementations specific to integration with `ndarray`.
//!
//! This module introduces the trait [`IntoNdArray`], which is implemented for
//! all NIfTI volume types and enable their mapping into an [`Array`] with a
//! dynamic number of dimensions and an abitrary element type. The affine
//! scaling of the values (from the `scl_slope` and `scl_inter` attributes) are
//! also considered in this transformation.
//!
//! A target [element type] needs to be provided at compile time, which is
//! usually the data type that the user wishes to work the array with. If the
//! source and target types do not match, each voxel is cast in a way as to
//! avoid loss of precision.
//!
//! #### Note on memory order
//!
//! NIfTI volumes are usually stored in disk in column major order (also called
//! Fortran order). As such, the array resulting from this operation will also
//! be in this memory order, rather than the usual row major order (AKA C
//! ordering). When accessing the array, one should consider any potential
//! bottlenecks emerging from this ordering in their data processing pipelines.
//! Namely, it might be faster to produce output arrays in column major order
//! as well.
//!
//! [`IntoNdArray`]: ./trait.IntoNdArray.html
//! [`Array`]: ../../../ndarray/type.Array.html
//! [element type]: ../element/trait.DataElement.html
//!
use error::Result;
use ndarray::{Array, Axis, Ix, IxDyn};
use num_traits::AsPrimitive;
use std::ops::{Add, Mul};
use volume::element::DataElement;
use volume::NiftiVolume;

/// Trait for volumes which can be converted to an ndarray.
///
/// Please see the [module-level documentation](index.html) for more details.
pub trait IntoNdArray {
    /// Consume the volume into an ndarray with the same number of dimensions
    /// and the given target element type `T`.
    #[deprecated(
        since = "0.6.0",
        note = "unconventional naming, please use/implement `into_ndarray` instead"
    )]
    fn to_ndarray<T>(self) -> Result<Array<T, IxDyn>>
    where
        Self: Sized,
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: DataElement,
        u8: AsPrimitive<T>,
        i8: AsPrimitive<T>,
        u16: AsPrimitive<T>,
        i16: AsPrimitive<T>,
        u32: AsPrimitive<T>,
        i32: AsPrimitive<T>,
        u64: AsPrimitive<T>,
        i64: AsPrimitive<T>,
        f32: AsPrimitive<T>,
        f64: AsPrimitive<T>,
    {
        self.into_ndarray::<T>()
    }

    /// Consume the volume into an ndarray with the same number of dimensions
    /// and the given target element type `T`.
    fn into_ndarray<T>(self) -> Result<Array<T, IxDyn>>
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: DataElement,
        u8: AsPrimitive<T>,
        i8: AsPrimitive<T>,
        u16: AsPrimitive<T>,
        i16: AsPrimitive<T>,
        u32: AsPrimitive<T>,
        i32: AsPrimitive<T>,
        u64: AsPrimitive<T>,
        i64: AsPrimitive<T>,
        f32: AsPrimitive<T>,
        f64: AsPrimitive<T>;
}

impl<V> IntoNdArray for super::SliceView<V>
where
    V: NiftiVolume + IntoNdArray,
{
    fn into_ndarray<T>(self) -> Result<Array<T, IxDyn>>
    where
        T: Mul<Output = T>,
        T: Add<Output = T>,
        T: DataElement,
        u8: AsPrimitive<T>,
        i8: AsPrimitive<T>,
        u16: AsPrimitive<T>,
        i16: AsPrimitive<T>,
        u32: AsPrimitive<T>,
        i32: AsPrimitive<T>,
        u64: AsPrimitive<T>,
        i64: AsPrimitive<T>,
        f32: AsPrimitive<T>,
        f64: AsPrimitive<T>,
    {
        // TODO optimize this implementation (we don't need the whole volume)
        let volume = self.volume.into_ndarray()?;
        Ok(volume.into_subview(Axis(self.axis as Ix), self.index as usize))
    }
}
