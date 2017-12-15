// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// Type of a SassValue. Useful for matching on.
/// Do not rely on the numeric value or that it is `repr(u8)`.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum SassValueType
{
	/// Boolean.
	Boolean = 0,
	
	/// Number.
	Number = 1,

	/// Color.
	Color = 2,

	/// String.
	String = 3,

	/// List.
	List = 4,

	/// Map.
	Map = 5,
	
	/// Null.
	Null = 6,
	
	/// Error.
	Error = 7,
	
	/// Warning.
	Warning = 8,
}

impl Default for SassValueType
{
	#[inline(always)]
	fn default() -> Self
	{
		SassValueType::Null
	}
}
