function run_day {
    cargo build --release -p $1
    ./target/release/$1
}

[ -z $1 ] && {
    for d in day-*; do
        echo "------------------------------------"
        echo "Running $d"
        # cargo run -p $d
        run_day $d
    done
} || {
    # cargo run -p $(printf "day-%02d" $1)
    run_day $(printf "day-%02d" $1)
}