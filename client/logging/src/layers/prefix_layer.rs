// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

use tracing::{span::Attributes, Id, Subscriber};
use tracing_subscriber::{
	layer::{Context},
	registry::{LookupSpan},
	Layer,
};

/// Span name used for the logging prefix. See macro `sc_cli::prefix_logs_with!`
pub const PREFIX_LOG_SPAN: &str = "substrate-log-prefix";

/// TODO doc
pub struct PrefixLayer;

impl<S> Layer<S> for PrefixLayer
where
	S: Subscriber + for<'a> LookupSpan<'a>,
{
	fn new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
		let span = ctx
			.span(id)
			.expect("new_span has been called for this span; qed");

		if span.name() != PREFIX_LOG_SPAN {
			return;
		}

		let mut extensions = span.extensions_mut();

		if extensions.get_mut::<Prefix>().is_none() {
			let mut s = String::new();
			let mut v = PrefixVisitor(&mut s);
			attrs.record(&mut v);

			if !s.is_empty() {
				let fmt_fields = Prefix(s);
				extensions.insert(fmt_fields);
			}
		}
	}
}

struct PrefixVisitor<'a, W: std::fmt::Write>(&'a mut W);

macro_rules! write_node_name {
	($method:ident, $type:ty, $format:expr) => {
		fn $method(&mut self, field: &tracing::field::Field, value: $type) {
			if field.name() == "name" {
				write!(self.0, $format, value).expect("no way to return the err; qed");
			}
		}
	};
}

impl<'a, W: std::fmt::Write> tracing::field::Visit for PrefixVisitor<'a, W> {
	write_node_name!(record_debug, &dyn std::fmt::Debug, "[{:?}] ");
	write_node_name!(record_str, &str, "[{}] ");
	write_node_name!(record_i64, i64, "[{}] ");
	write_node_name!(record_u64, u64, "[{}] ");
	write_node_name!(record_bool, bool, "[{}] ");
}

#[derive(Debug)]
pub(crate) struct Prefix(String);

impl Prefix {
	pub(crate) fn as_str(&self) -> &str {
		self.0.as_str()
	}
}
