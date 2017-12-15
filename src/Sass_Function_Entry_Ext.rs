// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Function_Entry class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
trait Sass_Function_Entry_Ext
{
	/// Get cookie; this is libsass terminology for the handle or reference to our SassFunction trait object.
	#[inline(always)]
	fn get_cookie(self) -> *mut c_void;
}

impl Sass_Function_Entry_Ext for Sass_Function_Entry
{
	#[inline(always)]
	fn get_cookie(self) -> *mut c_void
	{
		unsafe { sass_function_get_cookie(self) as *mut c_void }
	}
}
