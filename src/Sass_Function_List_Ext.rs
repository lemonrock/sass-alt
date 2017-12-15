// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Function_List class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
trait Sass_Function_List_Ext
{
	/// Make a new Sass_Function_List with space for `size` functions.
	#[inline(always)]
	fn make(size: usize) -> Self;
	
	/// Set an entry at an index < size.
	#[inline(always)]
	fn set_list_entry(self, index: usize, function_entry: Sass_Function_Entry);
	
	/// Delete (drop) this object. Guards that this object is not null.
	#[inline(always)]
	fn delete(self);
}

impl Sass_Function_List_Ext for Sass_Function_List
{
	#[inline(always)]
	fn make(size: usize) -> Self
	{
		unsafe { sass_make_function_list(size) }
	}
	
	#[inline(always)]
	fn set_list_entry(self, index: usize, function_entry: Sass_Function_Entry)
	{
		unsafe { sass_function_set_list_entry(self, index, function_entry) }
	}
	
	#[inline(always)]
	fn delete(self)
	{
		if !self.is_null()
		{
			unsafe { sass_delete_function_list(self) }
		}
	}
}
