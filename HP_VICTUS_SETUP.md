# Edge Optimizer - HP Victus Setup Guide

## For HP Victus Users

Your HP Victus laptop is now supported! Edge Optimizer uses the same thermal management API as HP OMEN systems.

## Installation Steps

### 1. Install HP OMEN Gaming Hub

HP Victus laptops use **HP OMEN Gaming Hub** for thermal and performance management.

**Option A: Microsoft Store (Recommended)**
1. Open Microsoft Store
2. Search for "HP OMEN Gaming Hub"
3. Click Install
4. Launch the app once to complete setup

**Option B: HP Website**
1. Visit [HP Support](https://support.hp.com/)
2. Enter your HP Victus model number
3. Download "HP OMEN Gaming Hub" from the software section
4. Run the installer

### 2. Verify Installation

After installing HP OMEN Gaming Hub, the required `NativeRpcClient.dll` should be available at:
- `C:\Program Files\HP\OMEN Gaming Hub\NativeRpcClient.dll`
- Or `C:\Program Files (x86)\HP\OMEN Gaming Hub\NativeRpcClient.dll`

### 3. Run Edge Optimizer

```powershell
# Build the project
cargo build --release --bin max_fan

# Run as administrator (recommended)
Start-Process .\target\release\max_fan.exe -Verb RunAs
```

Or simply double-click `max_fan.exe` and approve the admin prompt.

## Expected Behavior

When successful, you'll see:

```
Edge Optimizer - Max Fan Control
=================================

✓ Fan control supported
Available profiles:
  - Default (id=0)
  - Performance (id=1)
  - Cool (id=2)
  - Quiet (id=3)
  - Extreme (Max) (id=4) (MAX)

Setting fan to maximum...
✓ Successfully set fan profile to max (id=4)
```

Your fans will ramp up to maximum speed for optimal gaming performance!

## Troubleshooting

### "DLL not found" Error

**Solution**: Install HP OMEN Gaming Hub (see step 1 above)

### "Access Denied" or Permission Errors

**Solution**: Run as Administrator
```powershell
Start-Process .\target\release\max_fan.exe -Verb RunAs
```

### Profile Change Not Working

1. Close any running HP OMEN Gaming Hub instances
2. Ensure you're not in a BIOS-locked mode
3. Try manually changing thermal profiles in HP OMEN Gaming Hub first to verify it works
4. Restart the system and try again

## HP Victus Models Tested

- HP Victus 16 (16-d0xxx, 16-d1xxx, 16-e0xxx, 16-e1xxx)
- HP Victus 15 (15-fa0xxx, 15-fa1xxx, 15-fb0xxx, 15-fb1xxx)

All HP Victus gaming laptops with HP OMEN Gaming Hub support should work.

## What This Does

Edge Optimizer sets your HP Victus to **Extreme (Max) thermal profile**, which:
- ✅ Maximizes fan speed for best cooling
- ✅ Allows CPU/GPU to maintain higher boost clocks longer
- ✅ Reduces thermal throttling during intense gaming
- ✅ Uses HP's official thermal management API (safe)
- ❌ Does NOT modify BIOS or EC registers (safe)
- ❌ Does NOT void warranty (uses official HP APIs)

## Reverting to Normal

Simply run HP OMEN Gaming Hub and select a different thermal profile (Default, Performance, Cool, or Quiet).

Or set the `HPOMENGAMINGHUB_THERMAL_PROFILE` environment variable and restart.

## Need Help?

- Check [DESIGN.md](DESIGN.md) for technical architecture details
- Open an issue on GitHub with your HP Victus model number
- Ensure HP OMEN Gaming Hub is up to date (Microsoft Store auto-updates)
