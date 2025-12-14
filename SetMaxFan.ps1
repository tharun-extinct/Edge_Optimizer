# Set HP Omen Fan Speed to Maximum

# Start a CIM session
$Session = New-CimSession -Name 'hpq' -SkipTestConnection

# Function to send BIOS WMI commands
Function Send-OmenBiosWmi {
    param (
         [ValidateNotNullOrEmpty()] [UInt32] $Command = 0x20008,
         [Parameter(Mandatory)] [UInt32] $CommandType,
         [Byte[]] $Data = $Null,
         [ValidateNotNullOrEmpty()] [Byte[]] $Sign = @(0x53, 0x45, 0x43, 0x55),
         [ValidateSet('0', '4', '128', '1024', '4096')] [String] $OutputSize = '0'
    )

    # Prepare the request
    If($Data -eq $Null) {
        $BiosDataIn = New-CimInstance -ClassName 'hpqBDataIn' -ClientOnly -Namespace 'root\wmi' -Property @{
            Command = $Command;
            CommandType = $CommandType;
            Size = [UInt32] 0;
            Sign = $Sign;
        }
    } Else {
        $BiosDataIn = New-CimInstance -ClassName 'hpqBDataIn' -ClientOnly -Namespace 'root\wmi' -Property @{
            Command = $Command;
            CommandType = $CommandType;
            hpqBData = $Data;
            Size = [UInt32] $Data.Length;
            Sign = $Sign;
        }
    }

    # Get BIOS method class instance
    $BiosMethods = Get-CimInstance -ClassName 'hpqBIntM' -CimSession $Session -Namespace 'root\wmi'

    # Call the BIOS method
    $Result = Invoke-CimMethod -InputObject $BiosMethods -MethodName ('hpqBIOSInt' + $OutputSize) -Arguments @{InData = [CimInstance] $BiosDataIn}

    # Check result
    If($Result.OutData.rwReturnCode -eq 0) {
        Write-Host '✓ Fan speed set to maximum successfully!' -ForegroundColor Green
    } Else {
        Write-Host "✗ Failed:  Error $($Result.OutData.rwReturnCode)" -ForegroundColor Red
    }
}

# Set Maximum Fan Speed On
Write-Host 'Setting fan speed to maximum...' -ForegroundColor Cyan
Send-OmenBiosWmi -CommandType 0x27 -Data 0x01

# Terminate the CIM session
Remove-CimSession -CimSession $Session