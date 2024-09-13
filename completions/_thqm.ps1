
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'thqm' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'thqm'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'thqm' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'The port to listen on')
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'The port to listen on')
            [CompletionResult]::new('-U', '-U ', [CompletionResultType]::ParameterName, 'The username to authenticate with')
            [CompletionResult]::new('--username', '--username', [CompletionResultType]::ParameterName, 'The username to authenticate with')
            [CompletionResult]::new('-P', '-P ', [CompletionResultType]::ParameterName, 'The password to authenticate with')
            [CompletionResult]::new('--password', '--password', [CompletionResultType]::ParameterName, 'The password to authenticate with')
            [CompletionResult]::new('-S', '-S ', [CompletionResultType]::ParameterName, 'The entry separator')
            [CompletionResult]::new('--separator', '--separator', [CompletionResultType]::ParameterName, 'The entry separator')
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'The page title')
            [CompletionResult]::new('--title', '--title', [CompletionResultType]::ParameterName, 'The page title')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'The page style')
            [CompletionResult]::new('--style', '--style', [CompletionResultType]::ParameterName, 'The page style')
            [CompletionResult]::new('--save-qrcode', '--save-qrcode', [CompletionResultType]::ParameterName, 'Save the qrcode image to file')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Show the qrcode in terminal')
            [CompletionResult]::new('--qrcode', '--qrcode', [CompletionResultType]::ParameterName, 'Show the qrcode in terminal')
            [CompletionResult]::new('-u', '-u', [CompletionResultType]::ParameterName, 'Show the page url')
            [CompletionResult]::new('--url', '--url', [CompletionResultType]::ParameterName, 'Show the page url')
            [CompletionResult]::new('-o', '-o', [CompletionResultType]::ParameterName, 'Shutdown server after first selection')
            [CompletionResult]::new('--oneshot', '--oneshot', [CompletionResultType]::ParameterName, 'Shutdown server after first selection')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Show custom input field')
            [CompletionResult]::new('--custom-input', '--custom-input', [CompletionResultType]::ParameterName, 'Show custom input field')
            [CompletionResult]::new('--list-styles', '--list-styles', [CompletionResultType]::ParameterName, 'List available page styles')
            [CompletionResult]::new('--no-shutdown', '--no-shutdown', [CompletionResultType]::ParameterName, 'Don''t allow the server to be shutdown from the page')
            [CompletionResult]::new('--no-qrcode', '--no-qrcode', [CompletionResultType]::ParameterName, 'Don''t allow the qrcode to be shown in the page')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
