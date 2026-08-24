// `break` and `continue` only affect the NEAREST loop. To leave an OUTER loop
// from inside an inner one, give the outer loop a label (`'name:`) and
// `break 'name`.

fn main() {
    let rows = [[1, 2], [3, 4], [5, 6]];

    // Label the outer loop `'search`. Walk every number; the first one greater
    // than 3, print "found: {number}" and `break 'search` to leave BOTH loops.
    // (A plain `break` here would only leave the inner loop and keep scanning
    // later rows.)
    // your code here                 // should print: found: 4
}
