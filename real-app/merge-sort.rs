// Real application example - Merge sort on an unsorted list

// Merge two ordered sublists
fn merge(list1: &Vec<i32>, start: usize, middle: usize, end: usize, list2: &mut Vec<i32>) {
    let mut index1 = start;
    let mut index2 = middle;

    for i in start..end {
        if (index1 < middle) && (index2 >= end || list1[index1] <= list1[index2]) {
            list2[i] = list1[index1];
            index1 += 1;
        } else {
            list2[i] = list1[index2];
            index2 += 1;
        }
    }
}

// Copy all elements from a worker mutable list to a mutable primary list
fn merge_copy(list1: &mut Vec<i32>, start: usize, end: usize, list2: &Vec<i32>) {
    // Here we use a macro called for_each to iterate over each element in the list
    // for_each is a macro that allows you to use an iterator inside the loop
    // which isn't possible using a normal for..in loop
    (start..end).for_each(|i| list1[i] = list2[i]); 
}

// Split a mutable list into two sublists
fn merge_split(list1: &mut Vec<i32>, start: usize, end: usize, list2: &mut Vec<i32>) {
    if end - start > 1 {
        let middle: usize = (end + start) / 2;
        merge_split(list1, start, middle, list2);
        merge_split(list1, middle, end, list2);
        merge(list1, start, middle, end, list2);
        merge_copy(list1, start, end, list2);
    }
}

// Performs merge sort on a mutable vector of `i32` elements.
pub fn sort(list: &mut Vec<i32>) {
    let size: usize = list.len();
    let mut worker: Vec<i32> = vec![0; size];
    merge_split(list, 0, size, &mut worker);
}


// Test our merge sort with an example list
fn main(){
    // Create an example of an unordered list filled with 'i32' elements
    let mut my_list: Vec<i32> = vec![10, 30, 14, 11, 43, -3, 6, -10, 500];

    // Print our original vector
    println!("Unsorted list: {:?}", my_list);

    // Perform merge sort on the list
    sort(&mut my_list); // We can pass references using &mut

    // Print our newly sorted list
    println!("Sorted list: {:?}", my_list);
}