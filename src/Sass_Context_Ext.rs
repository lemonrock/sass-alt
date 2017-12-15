// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Content class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
trait Sass_Context_Ext
{
	/// Get Sass_Options object.
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options;
	
	/// Get last compiler error status.
	#[inline(always)]
	fn get_error_status(self) -> i32;
	
	/// Get last compiler error message.
	#[inline(always)]
	fn get_error_message<'a>(self) -> Option<&'a CStr>;
	
	/// Get CSS result from SASS compiler.
	#[inline(always)]
	fn get_output_string<'a>(self) -> Option<&'a CStr>;
}

impl Sass_Context_Ext for *mut Sass_Context
{
	#[inline(always)]
	fn get_options(self) -> *mut Sass_Options
	{
		unsafe { sass_context_get_options(self) }
	}
	
	#[inline(always)]
	fn get_error_status(self) -> i32
	{
		unsafe { sass_context_get_error_status(self) }
	}
	
	#[inline(always)]
	fn get_error_message<'a>(self) -> Option<&'a CStr>
	{
		let pointer = unsafe { sass_context_get_error_message(self) };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(pointer) })
		}
	}
	
	#[inline(always)]
	fn get_output_string<'a>(self) -> Option<&'a CStr>
	{
		let pointer = unsafe { sass_context_get_output_string(self) };
		if pointer.is_null()
		{
			None
		}
		else
		{
			Some(unsafe { CStr::from_ptr(pointer) })
		}
	}
}
