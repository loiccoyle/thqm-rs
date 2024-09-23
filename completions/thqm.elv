
use builtin;
use str;

set edit:completion:arg-completer[thqm] = {|@words|
    fn spaces {|n|
        builtin:repeat $n ' ' | str:join ''
    }
    fn cand {|text desc|
        edit:complex-candidate $text &display=$text' '(spaces (- 14 (wcswidth $text)))$desc
    }
    var command = 'thqm'
    for word $words[1..-1] {
        if (str:has-prefix $word '-') {
            break
        }
        set command = $command';'$word
    }
    var completions = [
        &'thqm'= {
            cand -p 'The port to listen on'
            cand --port 'The port to listen on'
            cand -U 'The username to authenticate with'
            cand --username 'The username to authenticate with'
            cand -P 'The password to authenticate with'
            cand --password 'The password to authenticate with'
            cand -S 'The entry separator'
            cand --separator 'The entry separator'
            cand -t 'The page title'
            cand --title 'The page title'
            cand -s 'The page style'
            cand --style 'The page style'
            cand --save-qrcode 'Save the qrcode image to file'
            cand -q 'Show the qrcode in terminal'
            cand --qrcode 'Show the qrcode in terminal'
            cand -u 'Show the page url'
            cand --url 'Show the page url'
            cand -o 'Shutdown server after first selection'
            cand --oneshot 'Shutdown server after first selection'
            cand -c 'Show custom input field'
            cand --custom-input 'Show custom input field'
            cand --list-styles 'List available page styles'
            cand --no-shutdown 'Don''t allow the server to be shutdown from the page'
            cand --no-qrcode 'Don''t allow the qrcode to be shown in the page'
            cand --install-styles 'Download and install styles to the user data directory'
            cand -h 'Print help'
            cand --help 'Print help'
            cand -V 'Print version'
            cand --version 'Print version'
        }
    ]
    $completions[$command]
}
