// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A Sass_Value typed as a known (floating point) number.
#[derive(Debug, Copy, Clone)]
pub struct NumberSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> NumberSassValue<'a>
{
	/// Get number value.
	#[inline(always)]
	pub fn value(&self) -> f64
	{
		unsafe { sass_number_get_value(self.reference.0 as *const _) }
	}
	
	/// Set number value.
	#[inline(always)]
	pub fn set_value(&self, value: f64)
	{
		unsafe { sass_number_set_value(self.reference.0, value) }
	}
	
	/// Get number units (as a C string, eg "px").
	#[inline(always)]
	pub fn unit(&self) -> &'a CStr
	{
		unsafe
		{
			let unit = sass_number_get_unit(self.reference.0 as *const _);
			CStr::from_ptr(unit)
		}
	}
	
	/// Set number units (as a C string, eg "px").
	#[inline(always)]
	pub fn set_unit(&self, unit: &CStr)
	{
		unsafe { sass_number_set_unit(self.reference.0, sass_copy_c_string(unit.as_ptr())) }
	}
}
