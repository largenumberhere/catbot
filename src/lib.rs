

#[cfg(test)]
mod cat_name_tests{
    use std::io::Write;
    use blake2::Blake2b512;
    use blake2::digest::FixedOutput;



    //hash tests
    #[test]
    pub fn hash_of_sequentual_writes_equals_full_write(){
        let source_bytes:Vec<u8> = vec![1,2,3,4,5,6];

        let hash1 = {
            let mut hasher = Blake2b512::default();
            for i in source_bytes.clone().into_iter(){
                let arr : [u8;1]= [i];
                hasher.write(&arr).unwrap();
            }
            hasher.finalize_fixed()
        };

        let hash2 = {
            let mut hasher = Blake2b512::default();
            hasher.write(source_bytes.as_slice()).unwrap();
            hasher.finalize_fixed()
        };

        assert_eq!(hash1,hash2)
    }

    #[test]
    pub fn hash_is_reporoducable(){
        let source_bytes_vec:Vec<u8> = (0..100).into_iter().collect();

        let mut results = Vec::with_capacity(100);
        for _i in 0..100 {
            let hash = {
                let mut hasher = Blake2b512::default();
                hasher.write(source_bytes_vec.as_slice()).unwrap();
                hasher.finalize_fixed()
            };

            results.push(hash);
        }

        let base_result = results.first().unwrap().to_owned();
        for i in results{
            assert_eq!(base_result,i);
        }
    }

    #[test]
    pub fn hash_detects_differences(){
        let source_bytes_vec:Vec<u8> = (0..3).into_iter().collect();
        let hash1 = {
            let mut hasher = Blake2b512::default();
            hasher.write(source_bytes_vec.as_slice()).unwrap();
            hasher.finalize_fixed()
        };

        let hash2 = {
            let mut hasher = Blake2b512::default();
            hasher.write(source_bytes_vec.as_slice()).unwrap();
            hasher.write(source_bytes_vec.as_slice() ).unwrap();
            hasher.finalize_fixed()
        };

        assert_ne!(hash2,hash1);
    }





}

