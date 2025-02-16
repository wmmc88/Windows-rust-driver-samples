// Copyright (c) Microsoft Corporation.
// License: MIT OR Apache-2.0

//! # Abstract
//!
//! This KMDF sample contains an intentional error that is designed to
//! demonstrate the capabilities and features of Driver Verifier and the Device
//! Fundamental tests.
//!     
//! The driver is designed to allocate memory using ExAllocatePool2 to its
//! Device Context buffer when a device is added by the PnP manager. However,
//! this buffer is not freed anywhere in the driver, including the driver unload
//! function.
//!
//! By enabling Driver Verifier on this driver, the pool leak
//! violation can be caught when the driver is unloaded and with an active KDNET
//! session, the bug can be analyzed further.

#![no_std]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::doc_markdown)]

#[cfg(not(test))]
extern crate wdk_panic;

#[cfg(not(test))]
use wdk_alloc::WdkAllocator;

#[cfg(not(test))]
#[global_allocator]
static GLOBAL_ALLOCATOR: WdkAllocator = WdkAllocator;

use wdk_sys::{GUID, PVOID};

// {A1B2C3D4-E5F6-7890-1234-56789ABCDEF0}
const GUID_DEVINTERFACE: GUID = GUID {
    Data1: 0xA1B2_C3D4u32,
    Data2: 0xE5F6u16,
    Data3: 0x7890u16,
    Data4: [
        0x12u8, 0x34u8, 0x56u8, 0x78u8, 0x9Au8, 0xBCu8, 0xDEu8, 0xF0u8,
    ],
};

// Global Buffer for the driver
static mut GLOBAL_BUFFER: PVOID = core::ptr::null_mut();

mod driver;
