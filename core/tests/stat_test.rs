#[cfg(test)]
mod stat_test {
    use textexcelport_core::statcrunching::{parse};
    use textexcelport_core::statcrunching::iqr::iqr_calc_string;

    #[test]
    fn parse_test() {
        let test_input = "304,354,356,358,397,402,418,419,425,426,430,437,453,462,477,478,492,500,502,521";
        let a = parse(test_input,",");
        //assert_eq!(a,vec![304,354,356,358,397,402,418,419,425,426,430,437,453,462,477,478,492,500,502,521])
    }
    #[test]
    fn iqr_test(){
        let test_input = "304,354,356,358,397,402,418,419,425,426,430,437,453,462,477,478,492,500,502,521";
        let a = parse(test_input,",");
        let b = iqr_calc_string(a);

    }
}