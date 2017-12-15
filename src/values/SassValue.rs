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
	
	/// Returns None if null.
	/// Returns an error if this is not a boolean or null.
	#[inline(always)]
	pub fn as_number<'a>(&'a self) -> Result<Option<NumberSassValue<'a>>, ()>
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
			Err(())
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a boolean or null.
	#[inline(always)]
	pub fn as_boolean<'a>(&'a self) -> Result<Option<BooleanSassValue<'a>>, ()>
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
			Err(())
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a warning or null.
	#[inline(always)]
	pub fn as_warning<'a>(&'a self) -> Result<Option<WarningSassValue<'a>>, ()>
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
			Err(())
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a error or null.
	#[inline(always)]
	pub fn as_error<'a>(&'a self) -> Result<Option<ErrorSassValue<'a>>, ()>
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
			Err(())
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a string or null.
	#[inline(always)]
	pub fn as_string<'a>(&'a self) -> Result<Option<StringSassValue<'a>>, ()>
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
			Err(())
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a color or null.
	#[inline(always)]
	pub fn as_color<'a>(&'a self) -> Result<Option<ColorSassValue<'a>>, ()>
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
			Err(())
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a list or null.
	#[inline(always)]
	pub fn as_list<'a>(&'a self) -> Result<Option<ListSassValue<'a>>, ()>
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
			Err(())
		}
	}
	
	/// Returns None if null.
	/// Returns an error if this is not a list or null.
	#[inline(always)]
	pub fn as_map<'a>(&'a self) -> Result<Option<MapSassValue<'a>>, ()>
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
			Err(())
		}
	}
	
	/// Is this owned by Rust
	#[inline(always)]
	pub fn is_owned_by_rust(&self) -> bool
	{
		self.1
	}
	
	/// Wraps a pointer from C but does take ownership; when the returned instance is dropped, the underlying pointer from C ***is not free-ed***.
	#[inline(always)]
	pub fn wrap_from_c(from_c: *mut Sass_Value) -> Self
	{
		SassValue(from_c, false)
	}
	
	#[inline(always)]
	/// Forgets this object and yields up original C pointer.
	/// Does not free this object.
	#[inline(always)]
	pub fn transfer_ownership_to_c(mut self) -> *mut Sass_Value
	{
		self.1 = false;
		let pointer = self.0;
		forget(self);
		pointer
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


/*

bool sass_value_is_list (const union Sass_Value* v);
union Sass_Value* sass_make_list    (size_t len, enum Sass_Separator sep, bool is_bracketed);

// Getters and setters for Sass_List
enum Sass_Separator sass_list_get_separator (const union Sass_Value* v);
void sass_list_set_separator (union Sass_Value* v, enum Sass_Separator value);

// Getters and setters for Sass_List values
union Sass_Value* sass_list_get_value (const union Sass_Value* v, size_t i);
void sass_list_set_value (union Sass_Value* v, size_t i, union Sass_Value* value);


bool sass_value_is_map (const union Sass_Value* v);
union Sass_Value* sass_make_map     (size_t len);

// Getter for the number of items in map
size_t sass_map_get_length (const union Sass_Value* v);
// Getters and setters for Sass_Map keys and values
union Sass_Value* sass_map_get_key (const union Sass_Value* v, size_t i);
void sass_map_set_key (union Sass_Value* v, size_t i, union Sass_Value*);
union Sass_Value* sass_map_get_value (const union Sass_Value* v, size_t i);
void sass_map_set_value (union Sass_Value* v, size_t i, union Sass_Value*);



// Execute an operation for two Sass_Values and return the result as a Sass_Value too
union Sass_Value* sass_value_op (enum Sass_OP op, const union Sass_Value* a, const union Sass_Value* b);








*/
