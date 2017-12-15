// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A Sass_Value typed as a known string.
#[derive(Debug, Copy, Clone)]
pub struct StringSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> StringSassValue<'a>
{
	/// Get string value.
	#[inline(always)]
	pub fn value(&self) -> &'a CStr
	{
		unsafe
		{
			let value = sass_string_get_value(self.reference.0 as *const _);
			CStr::from_ptr(value)
		}
	}
	
	/// Set string value.
	#[inline(always)]
	pub fn set_value(&self, value: &CStr)
	{
		unsafe { sass_string_set_value(self.reference.0, sass_copy_c_string(value.as_ptr()) ) }
	}
	
	/// Get whether this is a quoted string.
	#[inline(always)]
	pub fn is_quoted(&self) -> bool
	{
		unsafe { sass_string_is_quoted(self.reference.0 as *const _) }
	}
	
	/// Set whether this is a quoted string.
	#[inline(always)]
	pub fn set_is_quoted(&self, is_quoted: bool)
	{
		unsafe { sass_string_set_quoted(self.reference.0, is_quoted) }
	}
}
