param(
    [int]$Runs = 5
)

$ErrorActionPreference = "Stop"
$repo = Split-Path -Parent $PSScriptRoot
$target = Join-Path $repo "target-codex"
$compilerTarget = Join-Path $repo "target-codex"
$source = Join-Path $repo "examples\gc_benchmark.cs"
$csproj = Join-Path $repo "examples\gc_benchmark.csproj"
$glExecutable = Join-Path $target "gc-benchmark-gl.exe"
$csExecutable = Join-Path $repo "examples\bin\Release\net10.0\gc_benchmark.exe"

# 1. Compile C# version
Write-Host "Compiling C# (.NET) version..."
& dotnet build -c Release $csproj
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

# 2. Compile Gl version
Write-Host "Compiling Gl (LLVM) version..."
$env:CARGO_TARGET_DIR = $compilerTarget
& cargo run -- $source --emit-exe $glExecutable
if ($LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}

# 3. Benchmark runner helper
function Run-Benchmark([string]$name, [string]$executablePath) {
    $results = @()
    for ($run = 1; $run -le $Runs; $run++) {
        $startInfo = New-Object System.Diagnostics.ProcessStartInfo
        $startInfo.FileName = $executablePath
        $startInfo.UseShellExecute = $false
        $startInfo.RedirectStandardOutput = $true

        $process = New-Object System.Diagnostics.Process
        $process.StartInfo = $startInfo
        
        $stopwatch = [System.Diagnostics.Stopwatch]::StartNew()
        [void]$process.Start()

        $maxWorkingSet = 0L
        $maxPrivateMemory = 0L
        while (-not $process.HasExited) {
            $process.Refresh()
            $maxWorkingSet = [Math]::Max($maxWorkingSet, $process.WorkingSet64)
            $maxPrivateMemory = [Math]::Max($maxPrivateMemory, $process.PrivateMemorySize64)
            Start-Sleep -Milliseconds 5
        }
        $stopwatch.Stop()

        $output = $process.StandardOutput.ReadToEnd().Trim()
        $process.WaitForExit()
        $results += [PSCustomObject]@{
            Compiler = $name
            Run = $run
            Output = $output
            LiveAllocExit = $process.ExitCode
            MaxWorkingSetMB = [Math]::Round($maxWorkingSet / 1MB, 2)
            TimeMS = $stopwatch.ElapsedMilliseconds
        }
    }
    return $results
}

# 4. Execute benchmarks
Write-Host "Benchmarking Gl (LLVM) compiled binary..."
$glResults = Run-Benchmark "Gl (LLVM)" $glExecutable

Write-Host "Benchmarking C# (.NET) compiled binary..."
$csResults = Run-Benchmark "C# (.NET)" $csExecutable

# 5. Output comparison
Write-Host "`n=== BENCHMARK COMPARISON ==="
$glResults + $csResults | Format-Table -Property Compiler, Run, Output, LiveAllocExit, MaxWorkingSetMB, TimeMS -AutoSize

# 6. Safety check
$failed = ($glResults + $csResults) | Where-Object {
    $_.LiveAllocExit -ne 0 -or $_.Output -ne "60000"
}
if ($failed.Count -gt 0) {
    Write-Error "Benchmark runs failed validation"
    exit 1
}
