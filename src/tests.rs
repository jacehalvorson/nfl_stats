mod tests {
    use crate::utils::get_page;
    use tokio::test;

    #[test]
    async fn test_get_page() {
        let response = get_page().await.unwrap();
        let expected_data = vec! ["Rk", "Player", "Tm", "Age", "Pos",
        "G", "GS", "QBrec", "Cmp", "Att", "Cmp%", "Yds", "TD", "TD%", "Int", "Int%",
        "1D", "Lng", "Y/A", "AY/A", "Y/C", "Y/G", "Rate", "QBR", "Sk", "Yds", "Sk%",
        "NY/A", "ANY/A", "4QC", "GWD"];

        assert!( response == expected_data );
    }
}