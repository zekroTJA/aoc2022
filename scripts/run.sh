[ "$RELEASE" == "true" ] && RELEASE="--release" || RELEASE=""

function run_day {
    [ "$RELEASE" == "true" ] && TARGET=release || TARGET=debug
    cargo build $RELEASE -p $1
    ./target/$TARGET/$1 $2
}

[ -z $1 ] || [ "$1" == "--test" ] && {
    current_day=$(ls -r1 | grep day- | head -1)
    run_day $current_day $1
} || {
    # cargo run -p $(printf "day-%02d" $1)
    run_day $(printf "day-%02d" $1) $2
}