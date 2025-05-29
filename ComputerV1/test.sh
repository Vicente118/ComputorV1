# Second degre delta positif: 2 solutions
cargo run "5 * X^0 + 4 * X^1 - 9.3 * X^2 = 1 * X^0"
sleep 1
# Premier degre
cargo run "5 * X^0 + 4 * X^1 = 4 * X^0"
sleep 1
# Degre superieur a 2
cargo run "8 * X^0 - 6 * X^1 + 0 * X^2 - 5.6 * X^3 = 3 * X^0"
sleep 1
# Solution: {x ∈ R}
cargo run "6 * X^0 = 6 * X^0"
sleep 1
# Equation sans solution
cargo run "10 * X^0 = 15 * X^0"
sleep 1
# Second degre delta negatif: solutions complexes
cargo run "1 * X^0 + 2 * X^1 + 5 * X^2 = 0"
sleep 1
# Second degre discriminant nul
cargo run "1 * X^0 + 2 * X^1 + 1 * X^2 = 0"
sleep 1
# Equation avec terme en desordre
cargo run "3 * X^2 + 1 * X^0 - 4 * X^1 = 0"
sleep 1
# Equation a coef negatifs
cargo run -- "-1 * X^0 - 3 * X^1 - 2 * X^2 = 0"
sleep 1
# Equation avec termes des deux cotes
cargo run "3 * X^2 + 5 * X^0 = 1 * X^1 + 2 * X^0"
sleep 1
# Coef 0
cargo run "0 * X^0 + 0 * X^1 + 1 * X^2 = 0"
sleep 1
# Grands nombres
cargo run "1000000 * X^0 + 500000 * X^1 + 250000 * X^2 = 0"
sleep 1
# Petits nombres
cargo run "0.00001 * X^0 + 0.00002 * X^1 + 0.00003 * X^2 = 0"
sleep 1
# Autre test
cargo run "1 * X^0 + 2 * X^1 + 5 * X^2 = 0"
sleep 1
# Autre test
cargo run "1 * X^0 + 0 * X^1 + 1 * X^2 = 0"
sleep 1
# Errors
cargo run "1 * X^0 + 2 * X^1 + 5 * X^2"
sleep 1
cargo run "1 X^0 + 2 * X^1"