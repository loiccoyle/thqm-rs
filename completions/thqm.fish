complete -c thqm -s p -l port -d 'The port to listen on' -r
complete -c thqm -s U -l username -d 'The username to authenticate with' -r
complete -c thqm -s P -l password -d 'The password to authenticate with' -r
complete -c thqm -s S -l separator -d 'The entry separator' -r
complete -c thqm -s t -l title -d 'The page title' -r
complete -c thqm -s s -l style -d 'The page style' -r
complete -c thqm -l style-dir -d 'Specify style with its root directory' -r -F
complete -c thqm -l save-qrcode -d 'Save the qrcode image to file' -r -F
complete -c thqm -s q -l qrcode -d 'Show the qrcode in terminal'
complete -c thqm -s u -l url -d 'Show the page url'
complete -c thqm -s o -l oneshot -d 'Shutdown server after first selection'
complete -c thqm -s c -l custom-input -d 'Show custom input field'
complete -c thqm -l list-styles -d 'List available page styles'
complete -c thqm -l no-shutdown -d 'Don\'t allow the server to be shutdown from the page'
complete -c thqm -l no-qrcode -d 'Don\'t allow the qrcode to be shown in the page'
complete -c thqm -l install-styles -d 'Download and install styles to the user data directory'
complete -c thqm -s h -l help -d 'Print help'
complete -c thqm -s V -l version -d 'Print version'
