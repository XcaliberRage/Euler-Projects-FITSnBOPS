
// For a given sequence: U[n] = 1 − n + n2 − n3 + n4 − n5 + n6 − n7 + n8 − n9 + n10
// Find all BAD optimum polynomials (BOP)
// For each of the First Incorrect terms (FIT), sum them

use std::time::Instant;

struct UVAL {
    u: i64
}

impl UVAL {
    fn new(n: i64) -> Self {
        Self {
            u: 1 - n + n.pow(2) - n.pow(3) + n.pow(4) - n.pow(5) + n.pow(6) - n.pow(7) + n.pow(8) - n.pow(9) + n.pow(10)
        }
    }
}

trait TheSame {
    fn the_same(&self) -> bool;
}

impl<T: std::cmp::PartialEq> TheSame for Vec<T> {
    fn the_same(&self) -> bool {
        let length = self.len();
        self.iter().filter(|&u| u == &self[0]).collect::<Vec<_>>().len() == length
    }
}

// Starting at n = 1
// Identify the OP(k,n) where K is the number of terms given from the true sequence
// This should be the simplest expression possible
// When k = 1, we can assume that the sequence is constantly the same number i.e. OP(1, n) = u_1
// IF the OP is bad, the FIT will be OP(k, k+1)
// When this is not true, you can assume all further expressions of OP(k, n) will not change

// The goal is to sum each OP(k, k+1) from each BOP found

fn main() {

    let now = Instant::now();

    // So long as OP(k,k+1) returns a FIT
    // Increment k and try again

    let mut fits = Vec::new();
    let mut k = 1;

    'search: loop {

        println!("K = {} ::::", k);
        let truth = get_truth(k+1);
        let test = get_op(get_truth(k));

        println!("    Truth: {:?}", truth);
        println!("    Expected last val = {}", test);


        if test == truth[k as usize] {
            break 'search;
        }

        fits.push(test);
        println!("FITs: {:?}", fits);
        k += 1;
    }

    println!("All FITs: {:?}", fits);
    let sum: i64 = fits.iter().sum();
    println!("The sum of the FITS is {}", sum);
    println!("Solved in {}ms", now.elapsed().as_millis())

}

// Given K terms, return the correct values in the sequence
fn get_truth(k: u64) -> Vec<i64> {

    let range = 1..=k as i64;
    range.into_iter().map(|n| UVAL::new(n).u).collect::<Vec<_>>()

}

// Returns the next value in the sequence as predicted using the given values
fn get_op(seq: Vec<i64>) -> i64 {

    println!("        Given k values: {:?}", seq);
    let length = seq.len();
    if seq.the_same() {
        return seq.last().unwrap().clone();
    }

    let mut i = 0;
    let mut diffs = Vec::new();

    while i < length {
        if (i + 1) >= length {
            break;
        }

        diffs.push(seq[i+1] - seq[i]);
        i += 1;
    }

    seq.last().unwrap() + get_op(diffs)
}

/*
    I was looking here investigating what I might need to know for handling this, but polynomials are actually computationally trivial
    https://brownmath.com/alge/polysol.htm#StandardForm
    1.  If solving an equation, put it in standard form with 0 on one side and simplify.
    2.  Know how many roots to expect.
    3.  If you’re down to a linear or quadratic equation (degree 1 or 2), solve by inspection or the quadratic formula.
        Skip to 7.
    4.  Find one rational factor or root. This is the hard part, but there are lots of techniques to help you.
        If you can find a factor or root, continue with step 5 below; if you can’t, go to step 6.
    5.  Divide by your factor. This leaves you with a new reduced polynomial whose degree is 1 less.
        For the rest of the problem, you’ll work with the reduced polynomial and not the original.
        Continue at step 3.
    6.  If you can’t find a factor or root, turn to numerical methods.
    7.  If this was an equation to solve, write down the roots. If it was a polynomial to factor, write it in factored form, including any constant factors you took out in step 1.
 */