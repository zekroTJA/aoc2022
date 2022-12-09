[ -z $1 ] && {
    for d in day-*; do
        echo "------------------------------------"
        echo "Running $d"
        cargo run -p $d
    done
} || {
    cargo run -p $(printf "day-%02d" $1)
}