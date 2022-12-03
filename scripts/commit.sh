current_day=$(ls -r1 | grep day- | head -1)

# ---- CHECKS ----

function abort {
    echo "Abort: $1"
    exit 1
}

grep "TODO:" "$current_day/src/main.rs" && {
    abort "main.rs still contains TODOs!"
}

[ $(cat day-04/challenge.txt | wc -c) == "0" ] && {
    abort "challenge.txt is empty!"
}

cargo run -p "$current_day" || {
    abort "Run fails!"
}

cargo test -p "$current_day" || {
    abort "Unit Tests fail!"
}

# ---- CLEAN UP ----

rm -f "$current_day/test_input.txt"

# ---- COMMIT & PUSH ----

current_day=$(( ${current_day/*-} ))

git add .
git commit -m "add day $current_day solution"
git push