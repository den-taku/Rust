fn main() {
    'out:
    for i in 1..10 {
        'inn:
        for j in 1..10 {
            if j == 8 {
                break 'inn;
            } else if i == 7 && j == 8 {
                break 'out;
            } else {
                println!("( i : {}, j : {} )", i, j);
            }
        }
    }
                
    for i in 0..2000 {
        if i == 1000 {
            break;
        }
    }

    println!("Hello, world!");
}
