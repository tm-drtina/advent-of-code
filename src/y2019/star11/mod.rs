pub mod part1;
pub mod part2;

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let expected = 2415;
        let actual = super::part1::run(include_str!("input.txt"));
        assert_eq!(expected, actual);
    }
    #[test]
    fn part2() {
        let expected = "\
###  #### ###  #  # #### #  # ###   ## 
#  # #    #  # #  #    # #  # #  # #  #
###  ###  #  # #  #   #  #  # #  # #   
#  # #    ###  #  #  #   #  # ###  #   
#  # #    #    #  # #    #  # #    #  #
###  #    #     ##  ####  ##  #     ## \
";
        let actual = super::part2::run(include_str!("input.txt"));
        println!("{}", actual);
        assert_eq!(expected, actual);
    }
}
