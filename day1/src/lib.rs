pub fn bubble_sort(values:  &mut Vec<i64>) {
    loop {
        let mut swapped = false;
        for i in 0..(values.len() - 1) {
            if values[i] > values[i + 1] {
               let temp = values[i].clone();
                values[i] = values[i + 1];
                values[i + 1] = temp;
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

pub fn abs(a: i64) -> i64{
    if a < 0 {  
        a * -1
    } else { 
        a
    }
}
