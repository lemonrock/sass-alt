// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the Sass_Callee_Entry_Ext class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
pub trait Sass_Callee_Entry_Ext
{
	/// name.
	/// ?Can be null.
	#[inline(always)]
	fn name<'a>(self) -> Option<&'a CStr>;
	
	/// path.
	/// ?Can be null.
	#[inline(always)]
	fn path<'a>(self) -> Option<&'a CStr>;
	
	/// line.
	#[inline(always)]
	fn line(self) -> usize;
	
	/// column.
	#[inline(always)]
	fn column(self) -> usize;
	
	/// type.
	#[inline(always)]
	fn callee_type(self) -> Sass_Callee_Type;
	
	/// environment frame, used to access SASS environment variables.
	#[inline(always)]
	fn environment_frame(self) -> Sass_Env_Frame;
}

impl Sass_Callee_Entry_Ext for Sass_Callee_Entry
{
	#[inline(always)]
	fn name<'a>(self) -> Option<&'a CStr>
	{
		let pointer = unsafe { sass_callee_get_name(self) };
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
	fn path<'a>(self) -> Option<&'a CStr>
	{
		let pointer = unsafe { sass_callee_get_path(self) };
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
	fn line(self) -> usize
	{
		unsafe { sass_callee_get_line(self) }
	}
	
	#[inline(always)]
	fn column(self) -> usize
	{
		unsafe { sass_callee_get_column(self) }
	}
	
	#[inline(always)]
	fn callee_type(self) -> Sass_Callee_Type
	{
		unsafe { sass_callee_get_type(self) }
	}
	
	#[inline(always)]
	fn environment_frame(self) -> Sass_Env_Frame
	{
		unsafe { sass_callee_get_env(self) }
	}
}
