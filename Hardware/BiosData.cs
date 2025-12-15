// Graphics power settings data structure
[StructLayout(LayoutKind.Sequential, Pack = 1, Size = 4)]
public struct GpuPowerData {
    public GpuCustomTgp CustomTgp;  // Custom Total Graphics Power (TGP) limit
    public GpuPpab Ppab;            // Processing Power AI Boost (PPAB)
    public GpuDState DState;        // GPU device power state
    public byte PeakTemperature;    // Sensor threshold

    // Initializes the GPU power state structure based on a preset
    public GpuPowerData(GpuPowerLevel Level) {
        CustomTgp = Level == GpuPowerLevel. Minimum ? GpuCustomTgp.Off : GpuCustomTgp.On;
        Ppab = Level == GpuPowerLevel.Maximum ? GpuPpab. On : GpuPpab.Off;
        DState = GpuDState.D1;
        PeakTemperature = 0;
    }
}