#compdef movienator

autoload -U is-at-least

_movienator() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" \
'-h[Print help]' \
'--help[Print help]' \
'-V[Print version]' \
'--version[Print version]' \
":: :_movienator_commands" \
"*::: :->movienator" \
&& ret=0
    case $state in
    (movienator)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:movienator-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" \
'-d+[Date when movie was watched (Format: YYYY-MM-DD)]:WATCH_DATE: ' \
'-r+[0 to 5 rating]:RATING: ' \
'-t+[Thoughts about the movie]:THOUGHTS: ' \
'-h[Print help]' \
'--help[Print help]' \
':title -- Title of the movie:' \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
'-s+[Sort movies by value]:SORT:(title watch-date rating)' \
'--sort=[Sort movies by value]:SORT:(title watch-date rating)' \
'()-c[Print number of stored movies]' \
'()--count[Print number of stored movies]' \
'-d[Print all info about movies]' \
'--debug[Print all info about movies]' \
'-h[Print help]' \
'--help[Print help]' \
'::title -- Matches movies with given pattern:' \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" \
'()-a[Remove *ALL* movies]' \
'()--all[Remove *ALL* movies]' \
'(-a --all)-f[Force removal when multiple movies share a title]' \
'(-a --all)--force[Force removal when multiple movies share a title]' \
'-h[Print help]' \
'--help[Print help]' \
'::title -- Title of the movie to remove:' \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
":: :_movienator__help_commands" \
"*::: :->help" \
&& ret=0

    case $state in
    (help)
        words=($line[1] "${words[@]}")
        (( CURRENT += 1 ))
        curcontext="${curcontext%:*:*}:movienator-help-command-$line[1]:"
        case $line[1] in
            (add)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(list)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(remove)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
(help)
_arguments "${_arguments_options[@]}" \
&& ret=0
;;
        esac
    ;;
esac
;;
        esac
    ;;
esac
}

(( $+functions[_movienator_commands] )) ||
_movienator_commands() {
    local commands; commands=(
'add:Insert a movie' \
'list:Print movie information' \
'remove:Remove a movie' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'movienator commands' commands "$@"
}
(( $+functions[_movienator__add_commands] )) ||
_movienator__add_commands() {
    local commands; commands=()
    _describe -t commands 'movienator add commands' commands "$@"
}
(( $+functions[_movienator__help__add_commands] )) ||
_movienator__help__add_commands() {
    local commands; commands=()
    _describe -t commands 'movienator help add commands' commands "$@"
}
(( $+functions[_movienator__help_commands] )) ||
_movienator__help_commands() {
    local commands; commands=(
'add:Insert a movie' \
'list:Print movie information' \
'remove:Remove a movie' \
'help:Print this message or the help of the given subcommand(s)' \
    )
    _describe -t commands 'movienator help commands' commands "$@"
}
(( $+functions[_movienator__help__help_commands] )) ||
_movienator__help__help_commands() {
    local commands; commands=()
    _describe -t commands 'movienator help help commands' commands "$@"
}
(( $+functions[_movienator__help__list_commands] )) ||
_movienator__help__list_commands() {
    local commands; commands=()
    _describe -t commands 'movienator help list commands' commands "$@"
}
(( $+functions[_movienator__list_commands] )) ||
_movienator__list_commands() {
    local commands; commands=()
    _describe -t commands 'movienator list commands' commands "$@"
}
(( $+functions[_movienator__help__remove_commands] )) ||
_movienator__help__remove_commands() {
    local commands; commands=()
    _describe -t commands 'movienator help remove commands' commands "$@"
}
(( $+functions[_movienator__remove_commands] )) ||
_movienator__remove_commands() {
    local commands; commands=()
    _describe -t commands 'movienator remove commands' commands "$@"
}

if [ "$funcstack[1]" = "_movienator" ]; then
    _movienator "$@"
else
    compdef _movienator movienator
fi
