fn extend(vec: &mut Vec<f64>, slice: &[f64]) {
    for elt in slice {
        vec.push(*elt);
    }
}

fn main() {
    let v = vec![4, 8, 19, 27, 34, 10];
    {
        let r = &v;
        r[0];
    }
    let aside = v;

    let mut wave: Vec<f64> = Vec::new();
    let head  = vec![0.0, 1.0];
    let tail = [0.0, -1.0];

    extend(&mut wave, &head);
    extend(&mut wave, &tail);
    
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0]);

    extend(&mut wave, &wave);
    assert_eq!(wave, vec![0.0, 1.0, 0.0, -1.0, 0.0, 1.0, 0.0, -1.0]);
}
