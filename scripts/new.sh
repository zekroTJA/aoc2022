current_day=$(ls -r1 | grep day- | head -1)
current_day=${current_day/*-}
next_day=$((current_day + 1))
next_day_padded=$(printf "%02d" $next_day)

awk -F, "/\"day-03\",/"' { print; print "    \"day-'"$next_day_padded"'\","; next }1' Cargo.toml | tee .tmp
mv .tmp Cargo.toml

cargo new --bin --vcs=none "day-$next_day_padded"
cargo add -p "day-$next_day_padded" lib --path="lib"

cat > "day-$next_day_padded/src/main.rs" << EOF
fn main() {
    // let input = lib::read_input!();
    let input = lib::read_test_input!(); // TODO: Remove
}

#[cfg(test)]
mod test {
    use super::*;
}
EOF

curl -Lo "day-$next_day_padded/input.txt" "https://adventofcode.com/2022/day/${next_day}/input"

touch "day-$next_day_padded/challenge.txt"
touch "day-$next_day_padded/test_input.txt"