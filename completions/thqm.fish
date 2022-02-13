complete -c thqm -s p -l port -d 'Set the server\'s port.' -r
complete -c thqm -s u -l username -d 'Authentication username.' -r
complete -c thqm -s P -l password -d 'Authentication password.' -r
complete -c thqm -s S -l separator -d 'Entry separator.' -r
complete -c thqm -s t -l title -d 'Page title.' -r
complete -c thqm -s s -l style -d 'Page style.' -r -f -a "(thqm --list-styles)"
complete -c thqm -l interface -d 'Network interface to use to determine ip.' -r
complete -c thqm -l save-qrcode -d 'Save the qrcode image to file.' -r
complete -c thqm -s h -l help -d 'Print help information'
complete -c thqm -s V -l version -d 'Print version information'
complete -c thqm -l list-styles -d 'List available page styles.'
complete -c thqm -s q -l show-qrcode -d 'Show the qrcode in terminal.'
complete -c thqm -s U -l show-url -d 'Show the page url.'
complete -c thqm -l oneshot -d 'Shutdown server after first selection.'
complete -c thqm -l custom-input -d 'Show custom input field.'
complete -c thqm -l no-shutdown -d 'Don\'t allow the server to be shutdown from the page.'
complete -c thqm -l no-qrcode -d 'Don\'t show the qrcode on the page.'
