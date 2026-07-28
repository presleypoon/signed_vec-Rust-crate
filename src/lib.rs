#![allow(dead_code)]

/// # SignedVec
///
/// This is an vec that alsohas negative index
///
/// Have these methods:
///
/// * `new()`
/// * `read_from_index()`
/// * `write_from_index()`
/// * `range()`
#[derive(Clone)]
pub struct SignedVec<T> {
	pos_vec: Vec<T>,
	neg_vec: Vec<T>,
}

impl<T: Default> Default for SignedVec<T> {
	/// Create new `SignedVec`
	///
	/// ## Param
	///
	/// None
	///
	/// ## Return
	///
	/// Self: New `SignedVec`
	///
	fn default() -> Self {
		SignedVec {
			pos_vec: Vec::new(),
			neg_vec: Vec::new(),
		}
	}
}

impl<T> SignedVec<T> {
	pub const fn new() -> Self {
		SignedVec {
			pos_vec: Vec::new(),
			neg_vec: Vec::new(),
		}
	}
}

impl<T: Copy + Clone + Default> SignedVec<T> {
	/// Read using the index given
	///
	/// ## Param
	///
	/// `i` (`isize`): The index you want to use to read the vector
	///
	/// ## Return
	///
	/// `T`: The type the vec is storing
	pub fn read_from_index(&self, i: isize) -> T {
		if i >= 0 {
			self.pos_vec[i as usize]
		} else {
			self.neg_vec[i.unsigned_abs()]
		}
	}

	/// Write using the index given
	///
	/// ## Param
	///
	/// `i` (`isize`): The index you want to use to write the vector
	///
	/// `val` (`T`): The value you want to write at the index
	///
	/// `default` (`T`): The value you want to write at the index **if** the largest item (in signed i dir) is smaller than `i`
	///
	///
	/// ## Return
	///
	/// None
	pub fn write_from_index(&mut self, i: isize, val: T, default: T) {
		if i >= 0 {
			let len: usize = self.pos_vec.len();
			let i: usize = i as usize;
			if len >= i {
				self.pos_vec.resize(i + 1, default);
			}
			self.pos_vec[i] = val;
		} else {
			let len: usize = self.neg_vec.len();
			let i: usize = i.unsigned_abs();
			if len < i {
				self.neg_vec.resize(i + 1, default);
			}
			self.neg_vec[i] = val;
		}
	}

	/// Find the range of the `SignedVec`
	///
	/// ## Param
	///
	/// None
	///
	/// ## Return
	///
	/// `(usize, usize)`: 1st one is the length of the vec to the negative, 2nd one is the length of the vec to positive (Notes: the smallest item is the negative of the 1st one, and the largest is 2nd)
	pub fn range(&self) -> (usize, usize) {
		(self.neg_vec.len(), self.pos_vec.len())
	}
}
