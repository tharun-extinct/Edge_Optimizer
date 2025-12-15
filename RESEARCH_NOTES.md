# HP Victus Fan Control Research Notes

## DLL Located
✅ **Found**: `C:\Program Files (x86)\HP\HP Support Framework\Modules\NativeRpcClient.dll`

## Services Running
- HP Omen HSA Service (Running) ← Key service for thermal management

## Architecture Discovery

### NativeRpcClient.dll
- **Type**: Native C/C++ DLL (not .NET managed)
- **Size**: 21,880 bytes
- **Purpose**: RPC bridge to HP Omen HSA Service

### RpcClient.dll (.NET)
- **Namespace**: `Hp.Bridge.Client`
- **Methods**: ExecuteCommandAsync, RunNativeDLLCommandAsync
- **Architecture**: JSON-RPC over named pipes or TCP to HP services

## Challenges

1. **No Direct Function Exports**: NativeRpcClient.dll doesn't export standard C functions like `SetThermalProfile`
2. **RPC-Based**: Uses HP's proprietary RPC protocol to communicate with HP Omen HSA Service
3. **Command-Based**: Likely uses string commands sent via RpcClient

## Alternative Approaches

### Option 1: WMI/BIOS Settings
- Check `root\HP\InstrumentedBIOS` for thermal settings
- May require BIOS-level access

### Option 2: Windows Power Plans
- Use Windows power plan API
- Map profiles: Power Saver → Quiet, Balanced → Default, High Performance → Performance/Max
- **Limitation**: Doesn't directly control fans, relies on Windows thermal management

### Option 3: Reverse Engineer RPC Protocol
- Monitor HP OMEN Gaming Hub with Process Monitor
- Capture RPC calls when changing thermal profiles
- Reimplement protocol

### Option 4: Direct Service Communication
- Connect to HP Omen HSA Service endpoint
- Send thermal profile change commands
- **Status**: Need to discover service endpoint and command format

## Next Steps

1. Use Process Monitor to capture HP service communication
2. Install HP OMEN Gaming Hub on Victus and monitor its API calls
3. Check if HP exposes thermal control through registry keys
4. Implement Windows Power Plan fallback as interim solution
