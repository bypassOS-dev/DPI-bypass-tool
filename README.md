# DPI-bypass-tool

> ⚠️ **Work in progress.** DPI circumvention tool for bypassOS.

## About

`DPI-bypass-tool` is a network-level tool designed to bypass DPI-based blocking without using a VPN.

It modifies network traffic to make it harder for DPI systems to correctly identify and reconstruct the requested data.

## Current status

### Domain splitting

The tool searches for the target domain inside outgoing packets and splits the packet inside the domain.

**Before:**
[ ... example.com ... ]
**After:**
[ ... exam ][ ple.com ... ]

This prevents the complete domain from appearing in a single packet, which can bypass simple DPI signature matching.

If the domain cannot be found, the packet is split approximately in half.

### Garbage injection

Currently being developed:
[ domain_part_1 ][ garbage ][ domain_part_2 ]
The injected segment uses TTL and TCP SEQ manipulation so that DPI and the destination server end up processing different versions of the traffic.

## Tech stack

Written in Rust, developed as part of **bypassOS**.