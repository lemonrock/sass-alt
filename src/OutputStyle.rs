// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// SASS output style
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OutputStyle
{
	/// Nested.
	Nested,
	
	/// Expanded.
	Expanded,
	
	/// Compact.
	Compact,
	
	/// Compressed.
	Compressed,
}

impl Default for OutputStyle
{
	#[inline(always)]
	fn default() -> Self
	{
		OutputStyle::Compressed
	}
}
