// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// An extension type for the *mut Sass_Value class to make its methods Object-Orientated.
#[allow(non_camel_case_types)]
trait Sass_Value_Ext
{
	/// Delete (drop) this object. Guards that this object is not null.
	#[inline(always)]
	fn delete(self);
	
	/// Create a new string version of this object.
	#[inline(always)]
	fn stringify(self, compressed: bool, precision: u8) -> *mut Sass_Value;
	
	/// Enum tag of this object which states its type.
	#[inline(always)]
	fn tag(self) -> Sass_Tag;
}

impl Sass_Value_Ext for *mut Sass_Value
{
	#[inline(always)]
	fn delete(self)
	{
		if !self.is_null()
		{
			unsafe { sass_delete_value(self) }
		}
	}
	
	#[inline(always)]
	fn stringify(self, compressed: bool, precision: u8) -> *mut Sass_Value
	{
		unsafe { sass_value_stringify(self, compressed, precision as i32) }
	}
	
	#[inline(always)]
	fn tag(self) -> Sass_Tag
	{
		unsafe { sass_value_get_tag(self) }
	}
}
