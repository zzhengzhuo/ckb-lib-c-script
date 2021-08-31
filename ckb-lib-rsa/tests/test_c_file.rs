#[cfg(feature = "c_file")]
#[cfg(not(feature = "default"))]
mod test_c_file {

    #[test]
    fn test_get_c_bin() {
        use ckb_lib_rsa::get_librsa_bin;
        println!("{:?}", get_librsa_bin());
    }
}
