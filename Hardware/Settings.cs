// GPU power (Custom TGP & PPAB)
public BiosData.GpuPowerData GetGpuPower(bool forceUpdate = false) {
    if(forceUpdate || this.GpuPower == null)
        this.GpuPower = Hw.BiosGet<BiosData.GpuPowerData>(Hw.Bios.GetGpuPower);
    return (BiosData.GpuPowerData) this.GpuPower;
}

// Sets the GPU power (Custom TGP & PPAB)
public void SetGpuPower(BiosData.GpuPowerData value) {
    Hw.BiosSetStruct(Hw.Bios.SetGpuPower, value);
}

// Queries the Processing Power AI Boost state
public BiosData.GpuPpab GetGpuPpab(bool forceUpdate = false) {
    return GetGpuPower(forceUpdate).Ppab;
}