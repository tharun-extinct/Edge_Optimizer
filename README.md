Fan control is owned by EC (Embedded Controller) / BIOS

Windows does not expose a standard API

powercfg, WMI, Win32 → no fan control

Rust, C++, Python → same limitation



6️⃣ Option 2: OEM-specific hacks (UNSAFE, not recommended)
❌ EC register poking

Some people do:

Read/write EC memory

Use undocumented ACPI calls

Problems:

Requires kernel driver

Can brick EC

Can cause thermal runaway

Windows updates break it

If you’re thinking:

“I’ll write a Rust kernel driver”# Edge_Optimizer
