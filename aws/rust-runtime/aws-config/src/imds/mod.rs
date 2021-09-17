/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0.
 */

//! IMDSv2 Client, credential, and region provider
//!
//! See [`client`] for more information.
//!
//! **NOTE:** An IMDS credentials provider is not currently implemented. This module currently only
//! contains an IMDS client.
//!
pub mod client;

#[doc(inline)]
pub use client::Client;
