/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

use crate::endpoint_lib::diagnostic::DiagnosticCollector;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

pub(crate) const BASE_SET: &AsciiSet = &CONTROLS
    .add(b' ')
    .add(b'/')
    .add(b':')
    .add(b',')
    .add(b'?')
    .add(b'#')
    .add(b'[')
    .add(b']')
    .add(b'{')
    .add(b'}')
    .add(b'|')
    .add(b'@')
    .add(b'!')
    .add(b'$')
    .add(b'&')
    .add(b'\'')
    .add(b'(')
    .add(b')')
    .add(b'*')
    .add(b'+')
    .add(b';')
    .add(b'=')
    .add(b'%')
    .add(b'<')
    .add(b'>')
    .add(b'"')
    .add(b'^')
    .add(b'`')
    .add(b'\\');

// Returns `Option` for forwards compatibility
pub(crate) fn uri_encode<'a, 'b>(
    s: &'a str,
    _e: &'b mut DiagnosticCollector,
) -> std::borrow::Cow<'a, str> {
    utf8_percent_encode(s, BASE_SET).into()
}
