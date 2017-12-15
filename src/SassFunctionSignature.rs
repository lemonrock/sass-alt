// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// Represents a Sass Function signature.
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum SassFunctionSignature
{
	/// A signature like `foo()`.
	Ordinary(Cow<'static, str>),
	
	/// `*` - Fallback implementation.
	/// The fallback implementation will be given the name of the called function as the first argument, before all the original function arguments.
	Fallback,
	
	/// `@debug` - Overload debug statements.
	OverloadDebugStatements,
	
	/// `@warn` - Overload warn statements.
	OverloadWarnStatements,
	
	/// `@error` - Overload error statements.
	OverloadErrorStatements,
}

impl SassFunctionSignature
{
	/// Create a SassFunctionSignature from a static str
	#[inline(always)]
	pub fn from_static_str(signature: &'static str) -> Self
	{
		use self::SassFunctionSignature::*;
		
		match signature
		{
			"*" => Fallback,
			
			"@debug" => OverloadDebugStatements,
			
			"@warn" => OverloadWarnStatements,
			
			"@error" => OverloadErrorStatements,
			
			_ => Ordinary(Cow::Borrowed(signature))
		}
	}
	
	/// Create a SassFunctionSignature from a String
	#[inline(always)]
	pub fn from_string(signature: String) -> Self
	{
		use self::SassFunctionSignature::*;
		
		match signature.as_ref()
		{
			"*" => Fallback,
			
			"@debug" => OverloadDebugStatements,
			
			"@warn" => OverloadWarnStatements,
			
			"@error" => OverloadErrorStatements,
			
			_ => Ordinary(Cow::Owned(signature))
		}
	}
	
	#[inline(always)]
	fn as_str(&self) -> &str
	{
		use self::SassFunctionSignature::*;
		
		match *self
		{
			Ordinary(ref cow) => cow.as_ref(),
			
			Fallback => "*",
			
			OverloadDebugStatements => "@debug",
			
			OverloadWarnStatements => "@warn",
			
			OverloadErrorStatements => "@error",
		}
	}
}
