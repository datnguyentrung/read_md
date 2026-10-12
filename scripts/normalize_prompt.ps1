param(
    [Parameter(Mandatory = $true)]
    [string]$InputPath,

    [Parameter(Mandatory = $true)]
    [string]$OutputPath
)

$physicalLines = [System.IO.File]::ReadAllLines($InputPath, [System.Text.Encoding]::UTF8)
$logical = [System.Text.StringBuilder]::new()
$buffer = ''

# Rejoin YAML-style folded physical lines before decoding serialized escapes.
foreach ($physicalLine in $physicalLines) {
    $line = $physicalLine
    if ($buffer.Length -gt 0) {
        $line = [regex]::Replace($line, '^\s*\\', '')
    }

    if ($line.EndsWith('\')) {
        $buffer += $line.Substring(0, $line.Length - 1)
        continue
    }

    if ($buffer.Length -gt 0) {
        [void]$logical.AppendLine($buffer + $line)
        $buffer = ''
    } else {
        [void]$logical.AppendLine($line)
    }
}
if ($buffer.Length -gt 0) {
    [void]$logical.Append($buffer)
}

$text = $logical.ToString()
$text = $text.Replace('\\n', "`n")
$text = $text.Replace('\n', "`n")
$text = [regex]::Replace($text, '\\+\s+n', "`n")
$text = [regex]::Replace($text, '(?m)^n(?=\s*<)', '')
$text = $text.Replace('\\"', '"')
$text = $text.Replace('\"', '"')
$text = [regex]::Replace(
    $text,
    '\\U([0-9A-Fa-f]{8})',
    { param($match) [char]::ConvertFromUtf32([Convert]::ToInt32($match.Groups[1].Value, 16)) }
)

# The shared A2UI contract was embedded as concatenated string literals. Restore
# it to ordinary Markdown while retaining every example payload verbatim.
$cleaned = [System.Collections.Generic.List[string]]::new()
$inSerializedContract = $false
foreach ($rawLine in ($text -split "`r?`n")) {
    $line = $rawLine
    if ($line -eq '# === CRITICAL RULES ===') {
        $inSerializedContract = $true
    } elseif ($inSerializedContract -and $line.Trim() -eq '--------------') {
        $inSerializedContract = $false
    }

    if ($inSerializedContract) {
        $line = $line.Replace('n"  1) Card', '"  1) Card')
        if ($line.Trim() -in @('"', "'")) {
            $line = ''
        } elseif ($line -match '^\s*"') {
            $line = [regex]::Replace($line, '^(\s*)"', '$1')
            $line = [regex]::Replace($line, '"\s*$', '')
        } elseif ($line -match "^\s*'") {
            $line = [regex]::Replace($line, "^(\s*)'", '$1')
            $line = [regex]::Replace($line, "'\s*$", '')
        }
    }
    $cleaned.Add($line)
}
$text = $cleaned -join "`n"
$text = $text.Replace('{"      type":', '{"type":')
$text = [regex]::Replace($text, '"A2UIDataTable"\s+,', '"A2UIDataTable",')

$directory = Split-Path -Parent $OutputPath
if ($directory) {
    [System.IO.Directory]::CreateDirectory($directory) | Out-Null
}

[System.IO.File]::WriteAllText($OutputPath, $text, [System.Text.UTF8Encoding]::new($false))
