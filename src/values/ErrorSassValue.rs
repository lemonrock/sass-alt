// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A Sass_Value typed as a known error.
#[derive(Debug, Copy, Clone)]
pub struct ErrorSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> ErrorSassValue<'a>
{
	/// get error message
	#[inline(always)]
	pub fn message(&self) -> &'a CStr
	{
		unsafe
		{
			let message = sass_error_get_message(self.reference.0 as *const _);
			CStr::from_ptr(message)
		}
	}
	
	/// set error message
	#[inline(always)]
	pub fn set_message(&self, message: &CStr)
	{
		unsafe { sass_error_set_message(self.reference.0, sass_copy_c_string(message.as_ptr())) }
	}
}
