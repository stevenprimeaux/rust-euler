pub fn perm_lex(mut vec: Vec<u64>) -> () {
    let mut k: Option<usize>;
    let mut count: u64 = 0;

    loop {
        count +=1;

        if count == 1000000 {
            
            for e in &vec {
                print!("{} ", e);
            }
        }

        k = vec.windows(2).map(|x| x[0] < x[1]).rposition(|x| x == true);


        match k {
            None => return (),
            Some(k) => {
                let a_k = vec[k];

                let l: usize = vec
                    .iter()
                    .map(|x| x > &a_k)
                    .rposition(|x| x == true)
                    .unwrap();

                // println!("{}", l);

                vec.swap(k, l);

                vec[(k + 1)..].reverse();

                // for e in &vec {
                //     print!("{} ", e);
                // }
            }
        }
    }
}
