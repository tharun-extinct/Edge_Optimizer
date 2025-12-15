// Updates the GPU power settings to those passed in a structure
public void SetGpuPower(GpuPowerData data) {
    Check(Send(Cmd.Default, 0x22, Conv.GetByteArray(data)));
}

// Updates the GPU power settings to one of the presets
public void SetGpuPower(GpuPowerLevel value) {
    SetGpuPower(new GpuPowerData(value));
}