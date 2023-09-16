fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    // the issue is we don't know what the largest int in the Vec is until we've seen every value.
    // so we could for instance run into this:
    //
    //      candies = [3,3,3,3,3,20]
    //      extra_candies = 5
    //
    // in this case we get
    //      [false, false, false, ...., true]
    //
    // if we cycle over it once and find the largest num,
    // how fast can we finish? O(n) is the best we can do,
    // so O(n^2) overall, ewww.
    //
    // We could discard impossible values as we reach them,
    // so for something like this we would save some time:
    //
    //      candies = [7,2,9,1,11,2,14]
    //      extra_candies = 4
    //
    //      step 1: candies[0] is still possible, so push to temp_candies array, gc = 11
    //      step 2: candies[1] is not possible because candies[1] + extra_candies < gc
    //
    // But plucking the first item from the Vec every iteration will be
    // really inefficient because the compiler has to shift all items
    // to the left every time an item is taken from the front. So we'd be better
    // off reading the items from the front rather than copying the data over
    // to a new Vec
    let mut gc = 0;

    let candidates = candies.iter().filter(move |&cdy| {
        if cdy > &gc {
            gc = *cdy;
        } 
        print!("{} ", gc);
        // case where cdy is gte than gc
        *cdy + extra_candies >= gc
    });
    println!("{:?}", candidates.clone().collect::<Vec<_>>());
    println!("{}", gc);

    let res = candidates
        .map(|cdy| {
            if *cdy + extra_candies < gc {
                false
            } else {
                true
            }
        })
        .collect();

    // [true].to_vec()
    res
}

fn main() {
    let candies = Vec::from([2,2,2,2,2,20]);
    let extra_candies = 5;

    let res = kids_with_candies(candies, extra_candies);
    println!("{:?}", res);
}
