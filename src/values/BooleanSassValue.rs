// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A Sass_Value typed as a known boolean
#[derive(Debug, Copy, Clone)]
pub struct BooleanSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> BooleanSassValue<'a>
{
	/// get boolean value
	#[inline(always)]
	pub fn value(&self) -> bool
	{
		unsafe { sass_boolean_get_value(self.reference.0 as *const _) }
	}
	
	/// set boolean value
	#[inline(always)]
	pub fn set_value(&self, value: bool)
	{
		unsafe { sass_boolean_set_value(self.reference.0, value) }
	}
}
