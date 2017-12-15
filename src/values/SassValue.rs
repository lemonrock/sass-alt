// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A wrapper type for *mut Sass_Value that also reflects ownership.
#[derive(Debug)]
pub struct SassValue(pub(crate) *mut Sass_Value, pub(crate) bool);

impl Drop for SassValue
{
	#[inline(always)]
	fn drop(&mut self)
	{
		if self.is_owned_by_rust() && !self.0.is_null()
		{
			self.0.delete()
		}
	}
}

impl Clone for SassValue
{
	#[inline(always)]
	fn clone(&self) -> Self
	{
		Self::made_by_c(unsafe { sass_clone_value(self.0 as *const _) })
	}
}

impl SassValue
{
	/// Is this a null SassValue?
	#[inline(always)]
	pub fn is_null(&self) -> bool
	{
		unsafe { sass_value_is_null(self.0 as *const _) }
	}
	
	/// Returns an error if this is not a boolean.
	#[inline(always)]
	pub fn as_boolean<'a>(&'a self) -> Result<BooleanSassValue<'a>, SassValueError>
	{
		match self.as_boolean_or_null()?
		{
			None => Err(SassValueError::IsNotABoolean),
			Some(value) => Ok(value),
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a boolean or null.
	#[inline(always)]
	pub fn as_boolean_or_null<'a>(&'a self) -> Result<Option<BooleanSassValue<'a>>, SassValueError>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_boolean(self.0 as *const _) }
		{
			Ok(Some(BooleanSassValue { reference: self }))
		}
		else
		{
			Err(SassValueError::IsNotABooleanOrNull)
		}
	}
	
	/// Returns an error if this is not a color.
	#[inline(always)]
	pub fn as_color<'a>(&'a self) -> Result<ColorSassValue<'a>, SassValueError>
	{
		match self.as_color_or_null()?
		{
			None => Err(SassValueError::IsNotAColor),
			Some(value) => Ok(value),
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a color or null.
	#[inline(always)]
	pub fn as_color_or_null<'a>(&'a self) -> Result<Option<ColorSassValue<'a>>, SassValueError>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_color(self.0 as *const _) }
		{
			Ok(Some(ColorSassValue { reference: self }))
		}
		else
		{
			Err(SassValueError::IsNotAColorOrNull)
		}
	}
	
	/// Returns an error if this is not an error.
	#[inline(always)]
	pub fn as_error<'a>(&'a self) -> Result<ErrorSassValue<'a>, SassValueError>
	{
		match self.as_error_or_null()?
		{
			None => Err(SassValueError::IsNotAnError),
			Some(value) => Ok(value),
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a error or null.
	#[inline(always)]
	pub fn as_error_or_null<'a>(&'a self) -> Result<Option<ErrorSassValue<'a>>, SassValueError>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_error(self.0 as *const _) }
		{
			Ok(Some(ErrorSassValue { reference: self }))
		}
		else
		{
			Err(SassValueError::IsNotAnErrorOrNull)
		}
	}
	
	/// Returns an error if this is not a list.
	#[inline(always)]
	pub fn as_list<'a>(&'a self) -> Result<ListSassValue<'a>, SassValueError>
	{
		match self.as_list_or_null()?
		{
			None => Err(SassValueError::IsNotAList),
			Some(value) => Ok(value),
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a list or null.
	#[inline(always)]
	pub fn as_list_or_null<'a>(&'a self) -> Result<Option<ListSassValue<'a>>, SassValueError>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_list(self.0 as *const _) }
		{
			Ok(Some(ListSassValue { reference: self }))
		}
		else
		{
			Err(SassValueError::IsNotAListOrNull)
		}
	}
	
	/// Returns an error if this is not a map.
	#[inline(always)]
	pub fn as_map<'a>(&'a self) -> Result<MapSassValue<'a>, SassValueError>
	{
		match self.as_map_or_null()?
		{
			None => Err(SassValueError::IsNotAMap),
			Some(value) => Ok(value),
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a list or null.
	#[inline(always)]
	pub fn as_map_or_null<'a>(&'a self) -> Result<Option<MapSassValue<'a>>, SassValueError>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_map(self.0 as *const _) }
		{
			Ok(Some(MapSassValue { reference: self }))
		}
		else
		{
			Err(SassValueError::IsNotAMapOrNull)
		}
	}
	
	/// Returns an error if this is not a number.
	#[inline(always)]
	pub fn as_number<'a>(&'a self) -> Result<NumberSassValue<'a>, SassValueError>
	{
		match self.as_number_or_null()?
		{
			None => Err(SassValueError::IsNotANumber),
			Some(value) => Ok(value),
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a number or null.
	#[inline(always)]
	pub fn as_number_or_null<'a>(&'a self) -> Result<Option<NumberSassValue<'a>>, SassValueError>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_number(self.0 as *const _) }
		{
			Ok(Some(NumberSassValue { reference: self }))
		}
		else
		{
			Err(SassValueError::IsNotANumberOrNull)
		}
	}
	
	/// Returns an error if this is not a string.
	#[inline(always)]
	pub fn as_string<'a>(&'a self) -> Result<StringSassValue<'a>, SassValueError>
	{
		match self.as_string_or_null()?
		{
			None => Err(SassValueError::IsNotAString),
			Some(value) => Ok(value),
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a string or null.
	#[inline(always)]
	pub fn as_string_or_null<'a>(&'a self) -> Result<Option<StringSassValue<'a>>, SassValueError>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_string(self.0 as *const _) }
		{
			Ok(Some(StringSassValue { reference: self }))
		}
		else
		{
			Err(SassValueError::IsNotAStringOrNull)
		}
	}
	
	/// Returns an error if this is not a warning.
	#[inline(always)]
	pub fn as_warning<'a>(&'a self) -> Result<WarningSassValue<'a>, SassValueError>
	{
		match self.as_warning_or_null()?
		{
			None => Err(SassValueError::IsNotAWarning),
			Some(value) => Ok(value),
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a warning or null.
	#[inline(always)]
	pub fn as_warning_or_null<'a>(&'a self) -> Result<Option<WarningSassValue<'a>>, SassValueError>
	{
		if self.is_null()
		{
			Ok(None)
		}
		else if unsafe { sass_value_is_warning(self.0 as *const _) }
		{
			Ok(Some(WarningSassValue { reference: self }))
		}
		else
		{
			Err(SassValueError::IsNotAWarningOrNull)
		}
	}
	
	/// Is this owned by Rust?
	#[inline(always)]
	fn is_owned_by_rust(&self) -> bool
	{
		self.1
	}
	
	/// Forgets this object and yields up original C pointer.
	/// Does not free this object.
	#[inline(always)]
	pub(crate) fn transfer_ownership_to_c(mut self) -> *mut Sass_Value
	{
		self.1 = false;
		let pointer = self.0;
		forget(self);
		pointer
	}
	
	/// Create a new string SassValue using this object which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn tag(self) -> Sass_Tag
	{
		self.0.tag()
	}
	
	/// Create a new string SassValue using this object which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn stringify(self, compressed: bool, precision: u8) -> Self
	{
		let stringified = self.0.stringify(compressed, precision);
		Self::made_by_c(stringified)
	}
	
	#[inline(always)]
	fn made_by_c(pointer: *mut Sass_Value) -> Self
	{
		assert!(!pointer.is_null(), "libsass make could not allocate memory");
		SassValue(pointer, true)
	}
	
	/// Creates a new null SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_null() -> Self
	{
		Self::made_by_c(unsafe { sass_make_null() })
	}
	
	/// Creates a new boolean SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_boolean(value: bool) -> Self
	{
		Self::made_by_c(unsafe { sass_make_boolean(value) })
	}
	
	/// Creates a new number SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_number(value: f64, unit: &CStr) -> Self
	{
		Self::made_by_c(unsafe { sass_make_number(value, unit.as_ptr()) })
	}
	
	/// Creates a new unquoted string SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_unquoted_string(value: &CStr) -> Self
	{
		Self::made_by_c(unsafe { sass_make_string(value.as_ptr()) })
	}
	
	/// Creates a new quoted string SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_quoted_string(value: &CStr) -> Self
	{
		Self::made_by_c(unsafe { sass_make_qstring(value.as_ptr()) })
	}
	
	/// Creates a new warning SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_warning(message: &CStr) -> Self
	{
		Self::made_by_c(unsafe { sass_make_warning(message.as_ptr()) })
	}
	
	/// Creates a new error SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_error(message: &CStr) -> Self
	{
		Self::made_by_c(unsafe { sass_make_error(message.as_ptr()) })
	}
	
	/// Creates a new color SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_color(red: f64, green: f64, blue: f64, alpha: f64) -> Self
	{
		Self::made_by_c(unsafe { sass_make_color(red, green, blue, alpha) })
	}
	
	/// Creates a new list SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_list(size: usize, separator: ListSeparator, _is_bracketed: bool) -> Self
	{
		Self::made_by_c(unsafe { sass_make_list(size, separator.into()/*, _is_bracketed */) })
	}
	
	/// Creates a new map SassValue which will be ***free-ed*** when it is dropped unless `transfer_ownership_to_c()` is called.
	#[inline(always)]
	pub fn new_map(size: usize) -> Self
	{
		Self::made_by_c(unsafe { sass_make_map(size) })
	}
}
