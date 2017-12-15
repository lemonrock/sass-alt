// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


/// A type to model SASS functions.
pub trait SassFunction: Debug
{
	/// The signature of this SASS function including its name, eg `foo()`.
	/// There are also four special signatures. See `SassFunctionSignature`.
	/// The fallback implementation will be given the name of the called function as the first argument, before all the original function arguments.
	#[inline(always)]
	fn signature(&self) -> SassFunctionSignature;
	
	/// The implementation of this SASS function.
	fn callback(&mut self, arguments: SassValue, compiler: SassCompiler) -> SassValue;
}
