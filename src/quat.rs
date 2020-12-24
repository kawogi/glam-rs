#[allow(unused_imports)]
use num_traits::Float;

use crate::core::traits::{
    quaternion::Quaternion,
    vector::{FloatVector4, MaskVector4, Vector, Vector4, Vector4Const, VectorConst},
};
use crate::{DMat3, DMat4, DVec3, DVec4};
use crate::{Mat3, Mat4, Vec3, Vec3A, Vec4};

#[cfg(all(
    target_arch = "x86",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86::*;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "sse2",
    not(feature = "scalar-math")
))]
use core::arch::x86_64::*;

#[cfg(not(target_arch = "spirv"))]
use core::fmt;
use core::{
    cmp::Ordering,
    ops::{Add, Deref, Div, Mul, MulAssign, Neg, Sub},
};

#[cfg(feature = "std")]
use std::iter::{Product, Sum};

macro_rules! impl_quat {
    ($t:ty, $new:ident, $quat:ident, $vec3:ident, $vec4:ident, $mat3:ident, $mat4:ident, $inner:ident) => {
        /// Creates a quaternion from `x`, `y`, `z` and `w` values.
        ///
        /// This should generally not be called manually unless you know what you are doing. Use
        /// one of the other constructors instead such as `identity` or `from_axis_angle`.
        #[inline]
        pub fn $new(x: $t, y: $t, z: $t, w: $t) -> $quat {
            $quat::from_xyzw(x, y, z, w)
        }

        impl $quat {
            /// Creates a new rotation quaternion.
            ///
            /// This should generally not be called manually unless you know what you are doing.
            /// Use one of the other constructors instead such as `identity` or `from_axis_angle`.
            ///
            /// `from_xyzw` is mostly used by unit tests and `serde` deserialization.
            #[inline(always)]
            pub fn from_xyzw(x: $t, y: $t, z: $t, w: $t) -> Self {
                Self(Vector4::new(x, y, z, w))
            }

            #[inline(always)]
            pub const fn identity() -> Self {
                Self($inner::UNIT_W)
            }

            /// Creates a rotation quaternion from an unaligned `&[$t]`.
            ///
            /// # Preconditions
            ///
            /// The resulting quaternion is expected to be of unit length.
            ///
            /// # Panics
            ///
            /// Panics if `slice` length is less than 4.
            #[inline(always)]
            pub fn from_slice_unaligned(slice: &[$t]) -> Self {
                #[allow(clippy::let_and_return)]
                let q = Vector4::from_slice_unaligned(slice);
                glam_assert!(FloatVector4::is_normalized(q));
                Self(q)
            }

            /// Writes the quaternion to an unaligned `&mut [$t]`.
            ///
            /// # Panics
            ///
            /// Panics if `slice` length is less than 4.
            #[inline(always)]
            pub fn write_to_slice_unaligned(self, slice: &mut [$t]) {
                Vector4::write_to_slice_unaligned(self.0, slice)
            }

            /// Create a quaterion for a normalized rotation axis and angle (in radians).
            #[inline(always)]
            pub fn from_axis_angle(axis: $vec3, angle: $t) -> Self {
                Self($inner::from_axis_angle(axis.0, angle))
            }

            /// Creates a quaternion from the angle (in radians) around the x axis.
            #[inline(always)]
            pub fn from_rotation_x(angle: $t) -> Self {
                Self($inner::from_rotation_x(angle))
            }

            /// Creates a quaternion from the angle (in radians) around the y axis.
            #[inline(always)]
            pub fn from_rotation_y(angle: $t) -> Self {
                Self($inner::from_rotation_y(angle))
            }

            /// Creates a quaternion from the angle (in radians) around the z axis.
            #[inline(always)]
            pub fn from_rotation_z(angle: $t) -> Self {
                Self($inner::from_rotation_z(angle))
            }

            #[inline(always)]
            /// Create a quaternion from the given yaw (around y), pitch (around x) and roll (around z)
            /// in radians.
            pub fn from_rotation_ypr(yaw: $t, pitch: $t, roll: $t) -> Self {
                Self($inner::from_rotation_ypr(yaw, pitch, roll))
            }

            /// Creates a quaternion from a 3x3 rotation matrix.
            #[inline]
            pub fn from_rotation_mat3(mat: &$mat3) -> Self {
                Self(Quaternion::from_rotation_axes(
                    mat.x_axis.0,
                    mat.y_axis.0,
                    mat.z_axis.0,
                ))
            }

            /// Creates a quaternion from a 3x3 rotation matrix inside a homogeneous 4x4 matrix.
            #[inline]
            pub fn from_rotation_mat4(mat: &$mat4) -> Self {
                Self(Quaternion::from_rotation_axes(
                    mat.x_axis.0.into(),
                    mat.y_axis.0.into(),
                    mat.z_axis.0.into(),
                ))
            }

            /// Returns the rotation axis and angle of `self`.
            #[inline(always)]
            pub fn to_axis_angle(self) -> ($vec3, $t) {
                let (axis, angle) = self.0.to_axis_angle();
                ($vec3(axis), angle)
            }

            /// Returns the quaternion conjugate of `self`. For a unit quaternion the
            /// conjugate is also the inverse.
            #[inline(always)]
            pub fn conjugate(self) -> Self {
                Self(self.0.conjugate())
            }

            /// Computes the dot product of `self` and `other`. The dot product is
            /// equal to the the cosine of the angle between two quaterion rotations.
            #[inline(always)]
            pub fn dot(self, other: Self) -> $t {
                Vector4::dot(self.0, other.0)
            }

            /// Computes the length of `self`.
            #[inline(always)]
            pub fn length(self) -> $t {
                FloatVector4::length(self.0)
            }

            /// Computes the squared length of `self`.
            ///
            /// This is generally faster than `length()` as it avoids a square
            /// root operation.
            #[inline(always)]
            pub fn length_squared(self) -> $t {
                FloatVector4::length_squared(self.0)
            }

            /// Computes `1.0 / $quat::length()`.
            ///
            /// For valid results, `self` must _not_ be of length zero.
            #[inline(always)]
            pub fn length_recip(self) -> $t {
                FloatVector4::length_recip(self.0)
            }

            /// Returns `self` normalized to length 1.0.
            ///
            /// For valid results, `self` must _not_ be of length zero.
            #[inline(always)]
            pub fn normalize(self) -> Self {
                Self(FloatVector4::normalize(self.0))
            }

            /// Returns `true` if, and only if, all elements are finite.
            /// If any element is either `NaN`, positive or negative infinity, this will return `false`.
            #[inline(always)]
            pub fn is_finite(self) -> bool {
                FloatVector4::is_finite(self.0)
            }

            #[inline(always)]
            pub fn is_nan(self) -> bool {
                FloatVector4::is_nan(self.0)
            }

            /// Returns whether `self` of length `1.0` or not.
            ///
            /// Uses a precision threshold of `1e-6`.
            #[inline(always)]
            pub fn is_normalized(self) -> bool {
                FloatVector4::is_normalized(self.0)
            }

            #[inline(always)]
            pub fn is_near_identity(self) -> bool {
                self.0.is_near_identity()
            }

            /// Returns true if the absolute difference of all elements between `self` and `other`
            /// is less than or equal to `max_abs_diff`.
            ///
            /// This can be used to compare if two quaternions contain similar elements. It works
            /// best when comparing with a known value. The `max_abs_diff` that should be used used
            /// depends on the values being compared against.
            ///
            /// For more on floating point comparisons see
            /// https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/
            #[inline(always)]
            pub fn abs_diff_eq(self, other: Self, max_abs_diff: $t) -> bool {
                FloatVector4::abs_diff_eq(self.0, other.0, max_abs_diff)
            }

            /// Performs a linear interpolation between `self` and `other` based on
            /// the value `s`.
            ///
            /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
            /// is `1.0`, the result will be equal to `other`.
            #[inline(always)]
            pub fn lerp(self, end: Self, s: $t) -> Self {
                Self(self.0.lerp(end.0, s))
            }

            /// Performs a spherical linear interpolation between `self` and `end`
            /// based on the value `s`.
            ///
            /// When `s` is `0.0`, the result will be equal to `self`.  When `s`
            /// is `1.0`, the result will be equal to `end`.
            ///
            /// Note that a rotation can be represented by two quaternions: `q` and
            /// `-q`. The slerp path between `q` and `end` will be different from the
            /// path between `-q` and `end`. One path will take the long way around and
            /// one will take the short way. In order to correct for this, the `dot`
            /// product between `self` and `end` should be positive. If the `dot`
            /// product is negative, slerp between `-self` and `end`.
            #[inline(always)]
            pub fn slerp(self, end: Self, s: $t) -> Self {
                Self(self.0.slerp(end.0, s))
            }

            #[inline(always)]
            /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
            pub fn mul_vec3(self, other: $vec3) -> $vec3 {
                $vec3(self.0.mul_vector3(other.0))
            }

            #[inline(always)]
            /// Multiplies two quaternions.
            /// If they each represent a rotation, the result will represent the combined rotation.
            /// Note that due to floating point rounding the result may not be perfectly normalized.
            pub fn mul_quat(self, other: Self) -> Self {
                Self(self.0.mul_quaternion(other.0))
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Debug for $quat {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                fmt.debug_tuple(stringify!($quat))
                    .field(&self.x)
                    .field(&self.y)
                    .field(&self.z)
                    .field(&self.w)
                    .finish()
            }
        }

        #[cfg(not(target_arch = "spirv"))]
        impl fmt::Display for $quat {
            fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                write!(fmt, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
            }
        }

        impl Add<$quat> for $quat {
            type Output = Self;
            #[inline]
            /// Adds two quaternions.
            /// The sum is not guaranteed to be normalized.
            ///
            /// NB: Addition is not the same as combining the rotations represented by the two quaternions!
            /// That corresponds to multiplication.
            fn add(self, other: Self) -> Self {
                Self(self.0.add(other.0))
            }
        }

        impl Sub<$quat> for $quat {
            type Output = Self;
            #[inline]
            /// Subtracts the other quaternion from self.
            /// The difference is not guaranteed to be normalized.
            fn sub(self, other: Self) -> Self {
                Self(self.0.sub(other.0))
            }
        }

        impl Mul<$t> for $quat {
            type Output = Self;
            #[inline]
            /// Multiplies a quaternion with an $t.
            /// The product is not guaranteed to be normalized.
            fn mul(self, other: $t) -> Self {
                Self(self.0.scale(other))
            }
        }

        impl Div<$t> for $quat {
            type Output = Self;
            #[inline]
            /// Divides a quaternion by an $t.
            /// The quotient is not guaranteed to be normalized.
            fn div(self, other: $t) -> Self {
                Self(self.0.scale(other.recip()))
            }
        }

        impl Mul<$quat> for $quat {
            type Output = Self;
            #[inline]
            fn mul(self, other: Self) -> Self {
                Self(self.0.mul_quaternion(other.0))
            }
        }

        impl MulAssign<$quat> for $quat {
            #[inline]
            fn mul_assign(&mut self, other: Self) {
                self.0 = self.0.mul_quaternion(other.0);
            }
        }

        impl Mul<$vec3> for $quat {
            type Output = $vec3;
            #[inline]
            fn mul(self, other: $vec3) -> Self::Output {
                $vec3(self.0.mul_vector3(other.0))
            }
        }

        impl Neg for $quat {
            type Output = Self;
            #[inline]
            fn neg(self) -> Self {
                Self(self.0.scale(-1.0))
            }
        }

        impl Default for $quat {
            #[inline]
            fn default() -> Self {
                Self::identity()
            }
        }

        impl PartialEq for $quat {
            #[inline]
            fn eq(&self, other: &Self) -> bool {
                MaskVector4::all(self.0.cmpeq(other.0))
            }
        }

        impl PartialOrd for $quat {
            #[inline]
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                self.as_ref().partial_cmp(other.as_ref())
            }
        }

        impl AsRef<[$t; 4]> for $quat {
            #[inline(always)]
            fn as_ref(&self) -> &[$t; 4] {
                unsafe { &*(self as *const Self as *const [$t; 4]) }
            }
        }

        impl AsMut<[$t; 4]> for $quat {
            #[inline(always)]
            fn as_mut(&mut self) -> &mut [$t; 4] {
                unsafe { &mut *(self as *mut Self as *mut [$t; 4]) }
            }
        }

        impl From<$vec4> for $quat {
            #[inline(always)]
            fn from(v: $vec4) -> Self {
                Self(v.0)
            }
        }

        impl From<$quat> for $vec4 {
            #[inline(always)]
            fn from(q: $quat) -> Self {
                $vec4(q.0)
            }
        }

        impl From<($t, $t, $t, $t)> for $quat {
            #[inline(always)]
            fn from(t: ($t, $t, $t, $t)) -> Self {
                Self(Vector4::from_tuple(t))
            }
        }

        impl From<$quat> for ($t, $t, $t, $t) {
            #[inline(always)]
            fn from(q: $quat) -> Self {
                Vector4::into_tuple(q.0)
            }
        }

        impl From<[$t; 4]> for $quat {
            #[inline(always)]
            fn from(a: [$t; 4]) -> Self {
                Self(Vector4::from_array(a))
            }
        }

        impl From<$quat> for [$t; 4] {
            #[inline(always)]
            fn from(q: $quat) -> Self {
                Vector4::into_array(q.0)
            }
        }

        impl From<$quat> for $inner {
            // TODO: write test
            #[inline(always)]
            fn from(q: $quat) -> Self {
                q.0
            }
        }

        impl From<$inner> for $quat {
            #[inline(always)]
            fn from(inner: $inner) -> Self {
                Self(inner)
            }
        }

        impl Deref for $quat {
            type Target = crate::XYZW<$t>;
            #[inline(always)]
            fn deref(&self) -> &Self::Target {
                self.0.as_ref_xyzw()
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Sum<&'a Self> for $quat {
            fn sum<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self($inner::ZERO), |a, &b| Self::add(a, b))
            }
        }

        #[cfg(feature = "std")]
        impl<'a> Product<&'a Self> for $quat {
            fn product<I>(iter: I) -> Self
            where
                I: Iterator<Item = &'a Self>,
            {
                iter.fold(Self::identity(), |a, &b| Self::mul(a, b))
            }
        }
    };
}

#[cfg(all(target_feature = "sse2", not(feature = "scalar-math")))]
type InnerF32 = __m128;

#[cfg(any(not(target_feature = "sse2"), feature = "scalar-math"))]
type InnerF32 = crate::XYZW<f32>;

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// This type is 16 byte aligned.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Quat(pub(crate) InnerF32);

impl_quat!(f32, quat, Quat, Vec3, Vec4, Mat3, Mat4, InnerF32);

// f32 only implementation
impl Quat {
    #[inline]
    /// Multiplies a quaternion and a 3D vector, returning the rotated vector.
    pub fn mul_vec3a(self, other: Vec3A) -> Vec3A {
        Vec3A(self.0.mul_float4_as_vector3(other.0))
    }
}

impl Mul<Vec3A> for Quat {
    type Output = Vec3A;
    #[inline(always)]
    fn mul(self, other: Vec3A) -> Self::Output {
        self.mul_vec3a(other)
    }
}

type InnerF64 = crate::XYZW<f64>;

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct DQuat(pub(crate) InnerF64);

impl_quat!(f64, dquat, DQuat, DVec3, DVec4, DMat3, DMat4, InnerF64);
