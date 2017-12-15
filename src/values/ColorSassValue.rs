// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A Sass_Value typed as a known color.
#[derive(Debug, Copy, Clone)]
pub struct ColorSassValue<'a>
{
	reference: &'a SassValue,
}

impl<'a> ColorSassValue<'a>
{
	/// Get red value.
	#[inline(always)]
	pub fn red(&self) -> f64
	{
		unsafe { sass_color_get_r(self.reference.0 as *const _) }
	}
	
	/// Set red value.
	#[inline(always)]
	pub fn set_red(&self, value: f64)
	{
		unsafe { sass_color_set_r(self.reference.0, value) }
	}
	
	/// Get green value.
	#[inline(always)]
	pub fn green(&self) -> f64
	{
		unsafe { sass_color_get_g(self.reference.0 as *const _) }
	}
	
	/// Set green value.
	#[inline(always)]
	pub fn set_green(&self, value: f64)
	{
		unsafe { sass_color_set_g(self.reference.0, value) }
	}
	
	/// Get blue value.
	#[inline(always)]
	pub fn blue(&self) -> f64
	{
		unsafe { sass_color_get_b(self.reference.0 as *const _) }
	}
	
	/// Set blue value.
	#[inline(always)]
	pub fn set_blue(&self, value: f64)
	{
		unsafe { sass_color_set_b(self.reference.0, value) }
	}
	
	/// Get alpha (transparency) value.
	#[inline(always)]
	pub fn alpha(&self) -> f64
	{
		unsafe { sass_color_get_a(self.reference.0 as *const _) }
	}
	
	/// Set alpha (transparency) value.
	#[inline(always)]
	pub fn set_alpha(&self, value: f64)
	{
		unsafe { sass_color_set_a(self.reference.0, value) }
	}
}
