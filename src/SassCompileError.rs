// This file is part of sass-alt. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT. No part of predicator, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2017 The developers of sass-alt. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/sass-alt/master/COPYRIGHT.


quick_error!
{
	/// A compilation error.
	#[derive(Debug)]
	pub enum SassCompileError
	{
		/// An unknown reason for compilation failure because the underlying libsass library did not provide a reason.
		Unknown
		{
			description("Could not find compile for unknown reasons")
			display("Could not find compile for unknown reasons")
		}
		
		/// A known reason for compilation failure.
		Known(reason: CString)
		{
			description("Could not compile")
			display("Could not compile because {:?}", reason)
		}
		
		/// A known reason for compilation failure.
		CssWasNotUtf8(cause: Utf8Error)
		{
			cause(cause)
			description("CSS string was not UTF-8")
			display("CSS string was not UTF-8 {:?}", cause)
			from()
		}
	}
}
