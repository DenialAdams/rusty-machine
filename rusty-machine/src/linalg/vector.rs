//! The vector module.
//!
//! Currently contains all code
//! relating to the vector linear algebra struct.

use std::ops::{Mul, Add, Div, Sub, Index, Neg};
use libnum::{One, Zero, Float, FromPrimitive};
use std::cmp::PartialEq;
use linalg::Metric;
use linalg::utils;

/// The Vector struct.
///
/// Can be instantiated with any type.
pub struct Vector<T> {
    pub size: usize,
    pub data: Vec<T>,
}

impl<T> Vector<T> {
    /// Constructor for Vector struct.
    ///
    /// Requires the vector data.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let vec = Vector::new(vec![1.0,2.0,3.0,4.0]);
    /// ```
    pub fn new(data: Vec<T>) -> Vector<T> {

        let size = data.len();

        Vector {
            size: size,
            data: data,
        }
    }
}

impl<T: Copy + PartialOrd> Vector<T> {

    /// Find the argmax of the Vector.
    ///
    /// Returns the index of the largest value in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let a = Vector::new(vec![1.0,2.0,0.0,5.0]);
    /// let b = a.argmax();
    /// assert_eq!(b, 3);
    /// ```
    pub fn argmax(&self) -> usize {
        utils::argmax(&self.data)
    }

    /// Find the argmin of the Vector.
    ///
    /// Returns the index of the smallest value in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let a = Vector::new(vec![1.0,2.0,0.0,5.0]);
    /// let b = a.argmin();
    /// assert_eq!(b, 2);
    /// ```
    pub fn argmin(&self) -> usize {
        utils::argmin(&self.data)
    }
}

impl<T: Zero + One + Copy> Vector<T> {
    /// Constructs Vector of all zeros.
    ///
    /// Requires the size of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let vec = Vector::<f64>::zeros(10);
    /// ```
    pub fn zeros(size: usize) -> Vector<T> {
        Vector {
            size: size,
            data: vec![T::zero(); size],
        }
    }

    /// Constructs Vector of all ones.
    ///
    /// Requires the size of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let vec = Vector::<f64>::ones(10);
    /// ```
    pub fn ones(size: usize) -> Vector<T> {
        Vector {
            size: size,
            data: vec![T::one(); size],
        }
    }
}

impl<T: Copy + One + Zero + Mul<T, Output = T> + Add<T, Output = T>> Vector<T> {
    /// Compute dot product with specified Vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let a = Vector::new(vec![1.0,2.0,3.0,4.0]);
    /// let b = Vector::new(vec![2.0; 4]);
    ///
    /// let c = a.dot(&b);
    /// assert_eq!(c, 20.0);
    /// ```
    pub fn dot(&self, v: &Vector<T>) -> T {
        utils::dot(&self.data, &v.data)
    }
}

impl<T: Copy + Zero + Add<T, Output = T>> Vector<T> {
    /// The sum of the vector.
    ///
    /// Returns the sum of all elements in the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let a = Vector::new(vec![1.0,2.0,3.0,4.0]);
    ///
    /// let c = a.sum();
    /// assert_eq!(c, 10.0);
    /// ```
    pub fn sum(&self) -> T {
        self.data.iter().fold(T::zero(), |sum, &val| sum + val)
    }
}

impl<T: Copy + Zero + Mul<T, Output = T>> Vector<T> {
    /// The elementwise product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let a = Vector::new(vec![1.0,2.0,3.0,4.0]);
    /// let b = Vector::new(vec![1.0,2.0,3.0,4.0]);
    ///
    /// let c = &a.elemul(&b);
    /// assert_eq!(c.data, vec![1.0, 4.0, 9.0, 16.0]);
    /// ```
    pub fn elemul(&self, v: &Vector<T>) -> Vector<T> {
        assert_eq!(self.size, v.size);
        Vector::new(utils::ele_mul(&self.data, &v.data))
    }
}

impl<T: Copy + Zero + Div<T, Output = T>> Vector<T> {
    /// The elementwise division of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let a = Vector::new(vec![1.0,2.0,3.0,4.0]);
    /// let b = Vector::new(vec![1.0,2.0,3.0,4.0]);
    ///
    /// let c = &a.elediv(&b);
    /// assert_eq!(c.data, vec![1.0; 4]);
    /// ```
    pub fn elediv(&self, v: &Vector<T>) -> Vector<T> {
        assert_eq!(self.size, v.size);
        Vector::new(utils::ele_div(&self.data, &v.data))
    }
}

impl<T: Copy + Zero + Float + FromPrimitive> Vector<T> {
    /// The mean of the vector.
    ///
    /// Returns the arithmetic mean of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let a = Vector::<f32>::new(vec![1.0,2.0,3.0,4.0]);
    ///
    /// let c = a.mean();
    /// assert_eq!(c, 2.5);
    /// ```
    pub fn mean(&self) -> T {
        let sum = self.sum();
        sum / FromPrimitive::from_usize(self.data.len()).unwrap()
    }

    /// The variance of the vector.
    ///
    /// Returns the unbiased sample variance of the vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    ///
    /// let a = Vector::<f32>::new(vec![1.0,2.0,3.0,4.0]);
    ///
    /// let c = a.variance();
    /// assert_eq!(c, 5.0/3.0);
    /// ```
    pub fn variance(&self) -> T {
        let m = self.mean();
        let mut var = T::zero();

        for u in &self.data {
            var = var + (*u - m) * (*u - m);
        }

        var / FromPrimitive::from_usize(self.data.len() - 1).unwrap()
    }
}

/// Multiplies vector by scalar.
impl<T: Copy + One + Zero + Mul<T, Output = T>> Mul<T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, f: T) -> Vector<T> {
        (&self) * (&f)
    }
}

/// Multiplies vector by scalar.
impl<'a, T: Copy + One + Zero + Mul<T, Output = T>> Mul<T> for &'a Vector<T> {
    type Output = Vector<T>;

    fn mul(self, f: T) -> Vector<T> {
        self * (&f)
    }
}

/// Multiplies vector by scalar.
impl<'a, T: Copy + One + Zero + Mul<T, Output = T>> Mul<&'a T> for Vector<T> {
    type Output = Vector<T>;

    fn mul(self, f: &T) -> Vector<T> {
        (&self) * f
    }
}

/// Multiplies vector by scalar.
impl<'a, 'b, T: Copy + One + Zero + Mul<T, Output = T>> Mul<&'b T> for &'a Vector<T> {
    type Output = Vector<T>;

    fn mul(self, f: &T) -> Vector<T> {
        let new_data = self.data.iter().map(|v| (*v) * (*f)).collect();

        Vector {
            size: self.size,
            data: new_data,
        }
    }
}

/// Divides vector by scalar.
impl<T: Copy + One + Zero + PartialEq + Div<T, Output = T>> Div<T> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, f: T) -> Vector<T> {
        (&self) / (&f)
    }
}

/// Divides vector by scalar.
impl<'a, T: Copy + One + Zero + PartialEq + Div<T, Output = T>> Div<T> for &'a Vector<T> {
    type Output = Vector<T>;

    fn div(self, f: T) -> Vector<T> {
        self / (&f)
    }
}

/// Divides vector by scalar.
impl<'a, T: Copy + One + Zero + PartialEq + Div<T, Output = T>> Div<&'a T> for Vector<T> {
    type Output = Vector<T>;

    fn div(self, f: &T) -> Vector<T> {
        (&self) / f
    }
}

/// Divides vector by scalar.
impl<'a, 'b, T: Copy + One + Zero + PartialEq + Div<T, Output = T>> Div<&'b T> for &'a Vector<T> {
    type Output = Vector<T>;

    fn div(self, f: &T) -> Vector<T> {
        assert!(*f != T::zero());
        let new_data = self.data.iter().map(|v| *v / *f).collect();

        Vector {
            size: self.size,
            data: new_data,
        }
    }
}

/// Adds scalar to vector.
impl<T: Copy + One + Zero + Add<T, Output = T>> Add<T> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, f: T) -> Vector<T> {
        (&self) + (&f)
    }
}

/// Adds scalar to vector.
impl<'a, T: Copy + One + Zero + Add<T, Output = T>> Add<T> for &'a Vector<T> {
    type Output = Vector<T>;

    fn add(self, f: T) -> Vector<T> {
        self + (&f)
    }
}

/// Adds scalar to vector.
impl<'a, T: Copy + One + Zero + Add<T, Output = T>> Add<&'a T> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, f: &T) -> Vector<T> {
        (&self) + f
    }
}

/// Adds scalar to vector.
impl<'a, 'b, T: Copy + One + Zero + Add<T, Output = T>> Add<&'b T> for &'a Vector<T> {
    type Output = Vector<T>;

    fn add(self, f: &T) -> Vector<T> {
        let new_data = self.data.iter().map(|v| *v + *f).collect();

        Vector {
            size: self.size,
            data: new_data,
        }
    }
}

/// Adds vector to vector.
impl<T: Copy + One + Zero + Add<T, Output = T>> Add<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, v: Vector<T>) -> Vector<T> {
        (&self) + (&v)
    }
}

/// Adds vector to vector.
impl<'a, T: Copy + One + Zero + Add<T, Output = T>> Add<Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    fn add(self, v: Vector<T>) -> Vector<T> {
        self + (&v)
    }
}

/// Adds vector to vector.
impl<'a, T: Copy + One + Zero + Add<T, Output = T>> Add<&'a Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn add(self, v: &Vector<T>) -> Vector<T> {
        (&self) + v
    }
}

/// Adds vector to vector.
impl<'a, 'b, T: Copy + One + Zero + Add<T, Output = T>> Add<&'b Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    fn add(self, v: &Vector<T>) -> Vector<T> {
        assert!(self.size == v.size);

        let new_data = utils::vec_sum(&self.data, &v.data);

        Vector {
            size: self.size,
            data: new_data,
        }
    }
}

/// Subtracts scalar from vector.
impl<T: Copy + One + Zero + Sub<T, Output = T>> Sub<T> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, f: T) -> Vector<T> {
        (&self) - (&f)
    }
}

/// Subtracts scalar from vector.
impl<'a, T: Copy + One + Zero + Sub<T, Output = T>> Sub<T> for &'a Vector<T> {
    type Output = Vector<T>;

    fn sub(self, f: T) -> Vector<T> {
        self - (&f)
    }
}

/// Subtracts scalar from vector.
impl<'a, T: Copy + One + Zero + Sub<T, Output = T>> Sub<&'a T> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, f: &T) -> Vector<T> {
        (&self) - f
    }
}

/// Subtracts scalar from vector.
impl<'a, 'b, T: Copy + One + Zero + Sub<T, Output = T>> Sub<&'b T> for &'a Vector<T> {
    type Output = Vector<T>;

    fn sub(self, f: &T) -> Vector<T> {
        let new_data = self.data.iter().map(|v| *v - *f).collect();

        Vector {
            size: self.size,
            data: new_data,
        }
    }
}

/// Subtracts vector from vector.
impl<T: Copy + One + Zero + Sub<T, Output = T>> Sub<Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, v: Vector<T>) -> Vector<T> {
        (&self) - (&v)
    }
}

/// Subtracts vector from vector.
impl<'a, T: Copy + One + Zero + Sub<T, Output = T>> Sub<Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    fn sub(self, v: Vector<T>) -> Vector<T> {
        self - (&v)
    }
}

/// Subtracts vector from vector.
impl<'a, T: Copy + One + Zero + Sub<T, Output = T>> Sub<&'a Vector<T>> for Vector<T> {
    type Output = Vector<T>;

    fn sub(self, v: &Vector<T>) -> Vector<T> {
        (&self) - v
    }
}

/// Subtracts vector from vector.
impl<'a, 'b, T: Copy + One + Zero + Sub<T, Output = T>> Sub<&'b Vector<T>> for &'a Vector<T> {
    type Output = Vector<T>;

    fn sub(self, v: &Vector<T>) -> Vector<T> {
        assert!(self.size == v.size);

        let new_data = utils::vec_sub(&self.data, &v.data);

        Vector {
            size: self.size,
            data: new_data,
        }
    }
}

/// Gets negative of vector.
impl<T: Neg<Output=T> + Copy> Neg for Vector<T> {
    type Output = Vector<T>;

    fn neg(self) -> Vector<T> {
        let new_data = self.data.iter().map(|v| -*v).collect();

        Vector::new(new_data)
    }
}

/// Gets negative of vector.
impl<'a, T: Neg<Output=T> + Copy> Neg for &'a Vector<T> {
    type Output = Vector<T>;

    fn neg(self) -> Vector<T> {
        let new_data = self.data.iter().map(|v| -*v).collect();

        Vector::new(new_data)
    }
}

/// Indexes vector.
impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        assert!(idx < self.size);
        unsafe { &self.data.get_unchecked(idx) }
    }
}

impl<T: Float> Metric<T> for Vector<T> {
    /// Compute euclidean norm for vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_machine::linalg::vector::Vector;
    /// use rusty_machine::linalg::Metric;
    ///
    /// let a = Vector::new(vec![3.0,4.0]);
    /// let c = a.norm();
    ///
    /// assert_eq!(c, 5.0);
    /// ```
    fn norm(&self) -> T {
        let mut s = T::zero();

        for u in &self.data {
            s = s + (*u) * (*u);
        }

        s.sqrt()
    }
}
