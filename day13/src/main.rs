use std::cmp::Ordering;

use serde_json::Value;

struct Pair {
    left: Value,
    right: Value,
}

fn parser(s: String) -> Vec<Pair> {
    s.split("\n\n")
        .map(|p| {
            let (left, right) = p.split_once('\n').unwrap();
            Pair {
                left: serde_json::from_str(left).expect("Failed to parse"),
                right: serde_json::from_str(right).expect("Failed to parse"),
            }
        })
        .collect()
}

enum StopProcessing {
    InOrder,
    NotInOrder,
    _Error,
}

fn compare(l: &Value, r: &Value) -> Result<Ordering, StopProcessing> {
    match l {
        Value::Number(ref ln) => match r {
            Value::Number(rn) => {
                /*
                If both values are integers, the lower integer should come first.
                If the left integer is lower than the right integer, the inputs are in the right order.
                If the left integer is higher than the right integer, the inputs are not in the right order.
                Otherwise, the inputs are the same integer; continue checking the next part of the input.
                */
                match ln.as_u64().cmp(&rn.as_u64()) {
                    Ordering::Less => {
                        // Left < Right -- In order!
                        Err(StopProcessing::InOrder)
                    }
                    Ordering::Greater => {
                        // Left > Right -- Not in order!
                        Err(StopProcessing::NotInOrder)
                    }
                    Ordering::Equal => {
                        // Equal, check next
                        Ok(Ordering::Equal)
                    }
                }
            }
            Value::Array(_) => {
                /*
                If exactly one value is an integer, convert the integer to a list which contains that integer
                 as its only value, then retry the comparison.
                For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2);
                 the result is then found by instead comparing [0,0,0] and [2].
                */
                let new_l = Value::Array(vec![l.clone()]);
                compare(&new_l, r)
            }
            _ => panic!("Should not happen"),
        },
        Value::Array(la) => match r {
            Value::Array(ra) => {
                /*
                If both values are lists, compare the first value of each list, then the second value, and so on.
                If the left list runs out of items first, the inputs are in the right order.
                If the right list runs out of items first, the inputs are not in the right order.
                If the lists are the same length and no comparison makes a decision about the order, continue checking the next part of the input.
                */
                let mut li = la.iter();
                let mut ri = ra.iter();
                loop {
                    let li_item = li.next();
                    let ri_item = ri.next();
                    if li_item.is_none() && ri_item.is_none() {
                        return Ok(Ordering::Equal);
                    } else if li_item.is_none() {
                        // Left ran out first
                        return Err(StopProcessing::InOrder);
                    } else if ri_item.is_none() {
                        // Right ran out first
                        return Err(StopProcessing::NotInOrder);
                    }
                    // Keep going...
                    compare(li_item.unwrap(), ri_item.unwrap())?;
                }
            }
            Value::Number(_) => {
                /*
                If exactly one value is an integer, convert the integer to a list which contains that integer
                 as its only value, then retry the comparison.
                For example, if comparing [0,0,0] and 2, convert the right value to [2] (a list containing 2);
                 the result is then found by instead comparing [0,0,0] and [2].
                */
                let new_r = Value::Array(vec![r.clone()]);
                compare(&Value::Array(la.clone()), &new_r)
            }
            _ => panic!("Should not happen"),
        },
        _ => panic!("Should not happen"),
    }
}

fn solve(pairs: Vec<Pair>) -> usize {
    let mut indices_sum = 0;
    for (ndx, pair) in pairs.into_iter().enumerate() {
        match compare(&pair.left, &pair.right) {
            Ok(_) => {
                panic!("This should also not happen... (The pair are exactly equal)")
            }
            Err(e) => match e {
                StopProcessing::InOrder => indices_sum += ndx + 1,
                StopProcessing::NotInOrder => (),
                StopProcessing::_Error => panic!("Something bad happened"),
            },
        }
    }
    indices_sum
}

fn solve2(pairs: Vec<Pair>) -> usize {
    let mut packets: Vec<Value> = pairs.iter().flat_map(|p| vec![p.left.clone(), p.right.clone()]).collect();
    // Inject extra packets
    let packet_2: Value = serde_json::from_str("[[2]]").expect("Failed to parse");
    let packet_6: Value = serde_json::from_str("[[6]]").expect("Failed to parse");
    packets.insert(0, packet_2.clone());
    packets.insert(0, packet_6.clone());

    let mut ndx = 0;
    loop {
        if ndx + 1 >= packets.len() {
            break;
        }
        match compare(packets.get(ndx).unwrap(), packets.get(ndx+1).unwrap()) {
            Ok(_) => {
                panic!("This should also not happen... (The two are exactly equal)")
            }
            Err(e) => match e {
                StopProcessing::InOrder => {
                    ndx += 1;
                },
                StopProcessing::NotInOrder => {
                    packets.swap(ndx, ndx+1);
                    ndx = ndx.saturating_sub(1);
                },
                StopProcessing::_Error => panic!("Something bad happened"),
            },
        }
    }

    let item_2_ndx = packets.iter().position(|x| *x == packet_2).unwrap() + 1;
    let item_6_ndx = packets.iter().position(|x| *x == packet_6).unwrap() + 1;
    item_2_ndx * item_6_ndx
}

fn main() {
    let pairs = utils::load_puzzle_data(13, parser);
    let indices_sum = solve(pairs);
    println!("Solution 1: The sum of the indices that are in order: {indices_sum}");

    let pairs = utils::load_puzzle_data(13, parser);
    let decoder_key = solve2(pairs);
    println!("Solution 2: {decoder_key} is the decoder key.");
}

#[cfg(test)]
mod tests {
    use crate::{parser, solve, solve2};

    #[test]
    fn test_puzzle() {
        let test_data = utils::load_puzzle_test(13, parser);
        let solution = solve(test_data);
        assert_eq!(solution, 13);
    }

    #[test]
    fn test_puzzle2() {
        let test_data = utils::load_puzzle_test(13, parser);
        let solution = solve2(test_data);
        assert_eq!(solution, 140);
    }
}
