pub fn _get_sample_input() -> &'static str {
    "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############"
}

pub fn _get_sample_input2() -> &'static str {
    "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################"
}

pub fn get_input() -> &'static str {
    "#############################################################################################################################################
#.........#.......................#.#.........#.....#.....................#.............#...#...#.........#.........#.............#........E#
#.###.###.#.#.#################.#.#.#.#####.###.#.#.#####.#######.#######.#.#.#####.#.#.#.#.###.#.###.###.#.###.#.###.#######.#####.###.#####
#.#...#.....#.#.....#...#...........#.....#.#...#.#.#...#.#.....#.#...........................#.#.#.....#...#...#.#...#.....#...#...#.#.....#
#.#####.#.###.###.#.#.###.#####.#.#.#.###.#.#.###.#.#.#.#.#.###.#.#####.#####.###.#.#.#####.#.#.#.#.#.#.#.###.#.#.#.###.###.###.#.###.#####.#
#.#...#...#...#...#...#.....#...#.#.....#.#...#...#.#.#.#.#.....#.....#.....#.#...#.#.......#.#.#.#.#.............#.#.....#...#...#.....#...#
#.#.#.###.#.###.#######.#####.#.#.#.#.###.#####.###.#.#.#.#.#.###.###.#####.#.#.###.#.#.###.#.#.#.#.#.###.###.#.#.#.#.###.###.#######.###.#.#
#...#.....#.#...#.......#.#...#.#.#.#.........#...#...#.#...#.........................#.#.....#.#...#.#.#.....#...#...#.....#.#.....#.#...#.#
#.#######.#.#.#.#.#######.#.###.###.#.#####.#####.#####.#########.#.###.#######.#####.#.#.#.###.#####.#.#.###.#.#######.###.#.#.###.#.#.#####
#.#...#.....#.#.#...#...#.#...#.#.....#...#.#...#.#.....#.........#.#...#.....#.........................#.....#...#...#.....#.#...#...#.....#
#.###.#.#.#####.###.#.#.#.###.#.#.#.#.#.#.###.#.#.#.#####.#######.#.#.###.#.#####.#.#.###.#.#.#####.#.###.#####.#.#.#.#.###.#.#.#.###.#####.#
#.............#.......#...#.............#.....#...#.#.....#.......#.................#...#.....#.....#...#.#.....#...#.#.#.#...#.#.#.#.....#.#
#####.#.#.#.#.###########.#.#######.###############.#.#####.#######.#.#.#####.#.###.###.#.#.###.#####.#.#.#.#########.#.#.#######.#.#.#####.#
#.#...#.#.#.#.......#.....#...#.....#.............#.#.....#...#.....#...#...#...#...#...#.#.#.....#...#...#...#...#...#.#.........#...#.....#
#.#.###.#.#.#######.#######.#.#.#####.#########.#.#.#####.###.#.#.###.###.#.#####.###.#####.#.#####.#.###.###.###.#.#.#.###.#.#####.###.###.#
#.#.#.#.#.........#...........#.#.....#...#.....#.#.......#.#.#...#.#.#.#.#.#.....#.......#.#...#...#.........#...#.#.......#.#...#.#.#.#.#.#
#.#.#.#.#.#####.#.#############.#.#######.#.#####.#########.#.#.###.#.#.#.#.#.#.#########.#.###.#.###.#########.###.#######.#.#.#.#.#.#.#.#.#
#...........#...#...#.........#.#...#.....#.#.....#.#.......#...#...#.#...#...#...............#.#.....#.......#...#.......#...#...#...#.#...#
#.#.#.#######.#####.#.#######.#.###.#.#.###.###.#.#.#.#####.#####.###.#.###########.###########.###########.#.###.#####.#########.###.#.###.#
#...#...#.#...#.....#.......#.#...#.#.#...#...#...#.#...#.#.#.....#...#.....#.....#.#...........#.#.........#.........#...........#...#...#.#
#.###.#.#.#.###.#.#####.#.#.#.###.#.#.#.#.###.#.###.###.#.#.#.###.#.#######.#.#.#.#.#.###########.#.#.#####################.#####.#.#####.###
#.....#...#...#.........#.#.#.....#.#...#...#.#.....#...#.#.#.#.#...#.....#...#.#.#.........#.......#.#...............#.....#...#.#.#...#...#
#####.###.###.###.#######.#.#######.#####.###.#######.###.#.#.#.#####.#########.#.#########.#.#######.#.#############.#.###.#.###.#.#.#.###.#
#...#.....#...#...#...#...#...#...........#...#.....#...#.#.#.#.......#...........#.......#...#...#...#.#.....#.......#.#...#.....#.#.#...#.#
#.#.#.#####.###.#.###.#.#.###.#############.###.###.###.#.#.#.#.#####.#.###########.#####.#.#####.#.#.#.###.###.#######.#.#.#####.#.#####.#.#
#.#.#.#...#...#.#...#.#.#...#...#...#.....#...#...#...#.#.#...#...#...#...#.........#...#.#.#.....#.#.#...#.........#...#.#.........#.....#.#
#.###.#.#.###.#.###.#.#.###.###.#.#.#.###.###.###.#.#.#.#.#######.#.#####.#.#########.###.#.###.###.#####.#########.#.#.#.#######.###.###.#.#
#...#.#.#...#...#...#...#.#...#...#...#.#.......#.#.#.#.#...#.....#.#.....#...#...#.......#.....#...#.....#...........#.#.#.....#.....#...#.#
#.#.#.###.#.#####.###.###.###.#####.###.###.#####.#.#.#.###.#.#.#####.#######.###.#.###########.#.###.#####.###.###.#####.#.#.#.#.#########.#
#.#.......#.#.......#.#.....#.....#.........#.....#.#.#...#...#.......#.......#...#.....#.......#.#...#.....#.....#.#...#.#.#.#...#...#.....#
#.###.#######.#.###.#.#.#######.#.#.#.#.#####.#####.#####.#.#########.###.#####.#######.#######.#.#.###.#####.#.#.###.#.#.###.###.#.#.#.###.#
#.#...........#.....#...#.....#.#.#.#.#.#.....#...#.......#...#.....#...#.....#.......#.......#.#.#.#.................#.#.....#...#.#...#.#.#
#.#.#.###################.###.###.#.###.#.#####.#.###########.#.###.#.#.#####.#.#.###.#######.#.#.#.#.#####.#.#.#######.#####.#.#.#.#####.#.#
#.#.#.............#.......#.......#...#.#.......#.#.......#...#.#...#.........#.#...#.#...#...#.#.#...#...#...#.......#.....#.#.#.#...#...#.#
#.#.#.###.#####.###.#.###############.#.#####.###.#####.#.#.###.#.#####.#.#####.#.#.#.#.#.#.#####.#####.#.#########.#####.###.#.#.###.#.###.#
#.#.....#.#...#.....#.....#...#.......#.......#...#...#.#.#...#...#...#...#.....#...#...#.#.......#.#...#.#.......#.#.....#...#.#.....#.....#
#.###.#.#.#.#.#.#########.#.#.#.#.#.#####.#####.###.#.#.#.###.#.#.#.#.###########.#########.#####.#.#.###.#.###.#.#.#.###.#.###.#####.#####.#
#.#.#...#.#.#.#...#.....#...#.#.#.#.#.....#...#.#...#.#.#.....#.#...#.....#.....#.....#.....#...#...#...#.#.....#...#.#...#.#.#.#.....#...#.#
#.#.#.#####.#.###.#.###.#####.#.#.###.#####.#.#.###.#.#########.#########.#.###.#.###.#.#####.#.#.#####.#.#######.#.#.###.#.#.#.#####.#.#.#.#
#.#.#.....#.#...#...#.....#...#.#.......#...#.#.....#.............#.......#.#...#...#.......#.#.#.#.....#.......#...#...#.....#.........#.#.#
#.#.#####.#.###.#.#######.#.#######.#####.###.###################.#.#######.#.#####.#######.#.#.###.#####.#####.###.###.#####.#######.###.#.#
#...#.....#.#...#.#.....#.........#...#.....#...#.................#.#.......#.....#.#.....#.#.#...#.#.....#...#...#.#...#.....#...#.#.....#.#
###.#.#####.#.###.#.###.#.#######.#####.#######.#.#######.#.###.#.#.###.#########.###.###.###.###.#.#####.#.#.###.#.#.###.#####.#.#.#.#####.#
#...#...#...#...#.#...#.#...#...#.........#...#.#.......#.#.#.....#.....#.#.....#.....#.#...#...#.#.#...#.#.#.#.#.#.#.....#.....#...#...#...#
#.#####.#.#####.#####.#.#####.#.#.#########.#.#.#####.###.###.###.#######.#.#.#.#######.###.###.#.#.#.#.#.#.#.#.#.#.#####.#####.###.#.#.#.#.#
#...#.#...#...#.#.....#.......#.#.....#.....#...#.....#...#...#.#.........#.#...........#...#...#.#...#.#.#.#.#...#.#...#.......#.#.#.#.#...#
###.#.#######.#.#.#############.#.#####.#######.#.#####.###.###.#######.###.#.#######.###.###.###.#.#.#.###.#.###.###.#.#########.#.#.#.#####
#.#...............#.....#.....#.#.#.....#.#.....#...#.#.....#...#.....#.#...#.....#...#...#...#.#.#.....#...#...#.#...#.....#.........#.....#
#.#.#.#####.#.#######.#.###.###.###.#####.#.#######.#.#######.###.###.#.#.#######.#####.###.###.#.###.#.#.#####.#.#.#######.#.#.###.#.#####.#
#.#...#.#...#.#.....#.#...#.#...#...#.#...#.#.....#.#.........#...#.#.#.........#...#...#...#.#...#...#.#.....#.#...#.....#.#.#...#.......#.#
#.#####.#.#.#.#.#.#.#.###.#.#.###.#.#.#.#.#.#.###.#.#######.###.###.#.#.###########.#.###.#.#.#.###.###.#####.#.###.#.#.#.#.#####.###.###.#.#
#.......#.#...#.#.#...#.....#...#...#.#.#...#.#...#.....#...#...#...#.#...#.........#.#.#.#.#.#.....#.....#...#.#.......#.#.#...#.........#.#
#.#####.#.#.#.#.#.#####.#######.###.#.#.#####.#.#######.#.#.#.#####.#.#####.#######.#.#.#.#.#.#.###.#######.#.#.#.#####.###.#.#.#.###.#####.#
#.#...#.#.#.#...#.....#.#.......#...#.....#...#...#...#...#.#.#...#...#.....#.....#.#.#...#.#.#...#.......#.#...#.#...#.......#.#.#.........#
#.#.#.#.#.#.#####.###.#.#.#######.###.#####.#####.#.#.#.###.#.#.#.###.#.#.###.#####.#.#####.#.###.#.#####.#.#.###.###.#########.###.#.###.#.#
#.#.#.....#.#...#...#.#.#.......#.#.....#.......#...#.....#.....#...#...#.#...#.....#.#...#.#.#...#...#.#...#.#.......#.......#.....#.....#.#
#.#.###.#.#.#.#.###.#.#.#######.#.###.###.###############.#.#######.#####.#.#.#.#####.#.#.#.#.#.#####.#.#####.###.#####.#####.###.#####.###.#
#.#.#.#.#.#.#.#.....#.#.#...#...#...#.#.....#.............#.......#.#.......#.......#...#...#.#...#.#.#.....#.....#.....#...#...#.#...#.....#
###.#.#.#.#.#.#######.#.#.#.#.###.#.###.#####.#################.#.#.#.#############.#########.###.#.#.#.###.###.#.#.#######.###.###.#.#####.#
#...#.#.#.#.#.......#.#...#.#.#...#...#.#...#.......#.........#...#.#...#.......#.............#.........#...#...#.#.#...#...#.#.#...#.#.....#
#.###.#.#.#.#######.#.###.###.#.#####.#.#.#.#.#####.#.###.###.#.###.#####.#####.###########.###.###########.#.#####.#.#.#.#.#.#.#.###.#.#####
#.#.....#.#.......#.#.....#...#...#.#...#.#.#.....#.....#.#...#.#.#.......#...#.....#.....#.#...#...........#.......#.#...#...#...#...#.#...#
#.###.###.#.#####.#.###.###.#####.#.#.###.#.#####.#######.#.###.#.###.#.###.#.#.###.#.###.###.###.#######.#.#######.#.###.###.#####.###.###.#
#.......#.......#.#...#.#...#...#.......#.#.....#.#...#...#...#.....#...#...#.#...#.....#.#...#...#.......#.#.....#.....#.#...#...#...#.....#
#.#.#######.###.#.###.#.#.#.#.#.#######.#.#####.#.#.#.#.#.#########.#####.###.#.#####.###.#.#.#.#.#.#########.###.#####.#.#.#.#.#.###.#####.#
#.#.........#...#.#.#.#.#.#...#.......#.#.#...#.#.#.#.#...#.........#.....#...#.#...#.#...#.#...#.#.........#...#.#...#...#.#...#.....#...#.#
#.#########.#.###.#.#.###.#########.#.#.#.###.#.#.###.###.#.#########.#####.###.#.#.###.#.#.#####.#########.###.#.###.#.###.#######.#####.#.#
#.......#.......#...#...#...........#...#.....#.#.....#...#.#...#.....#.........#.#...#.#.#...#...........#.....#...#...#...#.....#.#...#...#
#######.#.#.###.###.#.#.#####.#.#.#.#########.#.#####.#.###.#.#.###.#########.###.###.#.#.###.#.#######.###########.###.#.###.###.###.#.#####
#...#.#.#.#.#...#.#...#.....#...#.#.....#...#.#.....#.#.......#...#...#...#...#.........#...#.#.......#.........#.#...#.#.....#...#...#.....#
#.#.#.#.#.#.#.###.###.###.#.#####.#.#.#.#.#.#.#####.#.###.#######.###.#.#.#.###.#######.#.###.###.###.#.#####.#.#.###.#.#######.#.#.#######.#
#.#...#...#...#.#.....#...#.#.....#.#.#.#.#.#.#...#.#.#...#.....#.....#.#.#...#.......#.#...#...#.#...#.#...#.#.....#.#.....#...#.#.........#
#.#######.#####.#.#####.#.#.#.#######.#.#.#.#.#.###.#.#.#.#.###.###.#####.###.#######.###.#.###.#.#.###.#.###.#######.#.#####.#####.#.#.###.#
#...............#.....#...#.#.....#...#.#.#...#.#...#.#.#...#...#.#.#.....#.........#.....#...#.#...#.#.#...#.........#.#.....#...#.#.#.#...#
#.#.#.#.#####.#######.###.#.#.###.#.###.#.#####.#.###.#.#####.###.#.#.#####.#######.#.#####.#.#.#.#.#.#.#.#.###########.#.#####.#.#.#.#.#.###
#...#.#.....#.......#...#.#.#...#.#...#...#.....#.#...#.#...#.#...#.#.......#.....#.#...#...#.#.#.#.#...#.#.......#.....#...#...#...#.#...#.#
#.###.#####.#.#####.###.#.#.###.#.#.#.#####.#####.#.#.#.###.#.###.#.###########.#.#.###.#.###.#.###.#.###.###.###.#####.###.#.#######.#####.#
#...#.....#.#...#...#...#.#.....#.#.#.......#.....#.#.#...#.#.....#.#...#.....#.#.#...#.#...#.#.....#.#.#.#.#...#.#...#.#...#.#.....#.#...#.#
#######.###.#####.###.###.###.#.#.#.#.#####.#.#.###.#.###.#.#####.#.#.#.#.###.#.#.###.#.###.#.#######.#.#.#.###.#.#.#.###.#.#.#.###.#.#.#.#.#
#.......#...#...#.#...#.#.....#.#.#...#.....#.....#.....#.#.......#.#.#...#.....#.#...#...#.#.#...#...#...#...#.#...#...#.#.#.....#.#.#.#.#.#
#.#.#.###.###.#.#.#.###.#####.###.#.#############.#######.#.#####.#.#.#############.#######.#.#.#.#.###.###.###.#######.#.#.#######.#.#.#.#.#
#.#...#...#...#...#.#...#...#.#...#.#.............#.......#.#...#...#.#...#...#...#.#.....#.#...#.#...#.....#...#.......#.#...#.....#...#.#.#
#.#####.#####.###.#.#.#.#.###.#.###.#.#############.#########.#.###.#.#.#.#.#.#.#.#.#.#.#.#.###.#.###.#######.#.#####.###.#.###.###.#####.#.#
#.#.....#.....#...#...#...#...#.....#.#.......................#.#...#...#...#...#.#...#.#...#...#...#.........#.#...#.....#.....#.#.....#...#
#.#.#####.###.###########.#.#.#####.#.###########.#############.#.###.###########.#.###.#.#####.#.#.#####.#######.#.#####.#.#####.#.#.#####.#
#...#.....#...#...........#.....#...#.....#.....#.#.....#.....#.#...#...........#.#.#...#.#...#...#...#...#.....#.#.....#.#.........#.......#
#.#####.#.#####.###.#.###.#.###.#.#######.#.###.#.###.#.#.###.#.###.#############.#.#.#####.#.#########.###.###.#.#####.#######.#.#.#########
#.....#.#...#...#.#.......#...#.#.........#...#.#.#...#.#...#.....................#.#.#.....#...........#...#.#.#.#...#...#...#.#...#.......#
#####.#####.#.###.#####.#####.#.#.#.#####.###.#.#.#.#.#####.###.#.#.###############.#.#.#############.###.###.#.#.#.#.###.#.#.#.###.#.###.###
#.....#...#.......#...#.#...#.#...#.#.#.....#.#.#...#.#...#.#...#.#...............#...#.......#.....#...#.#...#.....#...#...#...#.#.#...#...#
#.#####.#.#######.#.#.###.#.#.#####.#.#.#####.#.#######.#.#.#####.#.#############.#.#.###.#####.###.#####.#.#######.###.#########.#.#######.#
#.......#.#.....#.#.#.#...#.#.....#.#.....#...#...#...#.#.#.#.......#.#.......#...#.#.....#.....#.#.......#.#.......#.#...#.......#...#.....#
#########.#.###.###.#.#.###.#####.#.#######.#####.#.#.#.#.#.#.#####.#.#.#####.#.###.###.###.#####.#########.#.#####.#.###.#######.###.#.###.#
#.............#.#...#...#.........#.........#...#...#.#.#...#.#...#.#...#...#.#.....#.......#...............#...#.#.#...#.........#...#...#.#
#.#####.#####.#.#.#######.#######.###########.#.#####.#.###.#.#.###.#####.#.#.#######.#######.#.###########.###.#.#.#.#########.###.#####.#.#
#...#...#.....#.........#.#...#.........#.#...#.#.....#.....#.#...#.......#.....#.....#.......#.....#...#...#.#...#.#.........#.#...#.....#.#
#####.#.#.#.#.#####.###.#.#.#.#####.#.#.#.#.###.#.#####.#.###.###.#############.#.#.###.###.#.#####.#.#.###.#.#.###.#.#####.#.#.#.#####.###.#
#.....#.#.#.#.#...#...#.#.#.#.#...#...#...#.#.....#.....#.........#.......#...#...#...#...#.#.....#...#...#.#.#.#...#.#.....#...#.#...#...#.#
#.#####.###.#.#.#.#####.#.#.#.#.#.#######.#.#####################.#.#####.###.#.#####.#.#.#.#####.#######.#.#.#.#.###.#.#.#.#.###.#.#.###.###
#.#...#.....#...#.......#.#.#...#.....#.#.#.....................#...#...#.#...#.#...#.#.#.#.....#...#...#.#...#.#...#...#.#.#...#...#...#...#
#.#.#.#.###############.#.#.#########.#.#.###############.###.#.#######.#.#.###.#.#.#.###.#####.#####.#.#.#.###.###.#.###.#.#.#########.#.#.#
#.#.#.#.........#.#...#.#.#...#.....#.#.#...#.........#...#.....#.......#.#...#.#.#...#...#...#...#...#...#.#...#.#.#.#...#.#.........#.#.#.#
#.#.#.#.#.###.#.#.#.#.#.#.###.#.#.###.#.###.#.#.#####.#.#.#.#####.#######.###.#.#.#####.#####.###.#.#######.#.#.#.#.###.###.#.#######.#.###.#
#.#.#.#.....#.....#.#...#.#...#.#.....#.........#...#.#.#.#.#.#.....#.........#...#...#.....#.......#.....#.#.#...#.....#.#...#.....#.#.#...#
#.#.#.#.###.#.#####.#####.#.#######.###.#######.#.#.#.#.###.#.#.###.#.#############.###.###.#####.###.###.#.#.#.#.#######.#########.#.#.#.###
#...#...........#.....#...#...#...#...#.......#...#.#...#...#...#.#.....#...............#.#.....#.#.....#.#.#.#.#...#.......#.......#...#...#
#####.#.#.###.#.#.#####.#####.#.#.###.#.#####.#.###.#####.###.###.#######.#########.#####.#####.###.#####.#.#.#.###.#.###.#.###.###.#######.#
#.....#.......#...#...#.#...#...#.#...#.#.#...#...#...#...#.#.....#...#...#.....#.......#.....#...#.#.#...#.#.#.#...#.#.#.#...#...#.......#.#
#.#.#########.#####.#.#.#.#.#####.###.#.#.#.#.###.###.#.###.#####.#.###.#####.###.###.#.#.#######.#.#.#.#.###.#.#####.#.#.###.###.#######.#.#
#.#.#.....#.......#.#.#...#.....#...#...#.#.......#.#...#...#...#.#...#.#...#...#.#...#.#.......#.#...#.#.#...#...#.....#...#...#.....#...#.#
#.###.###.###.#####.#.#########.###.#####.#########.#####.###.#.#.###.#.#.#.###.#.#.###.#####.###.###.#.###.###.#.#.#.#####.###.#####.#.###.#
#.....#...#.........#.#.......#.#...#.......#.....#...#.......#...#...#...#...#...#...#.#...#.#.......#.#...#...#...#.....#...............#.#
#.#####.###.#.#.#####.#.#.#####.#.###.###.###.###.#.#.#.###########.#########.#######.#.#.#.#.#.#####.#.#.###########.###.###.#.#.#######.#.#
#.#...#.....#...#...#.#.#.......#...#...#.#.....#...#...#.........#.....#...#.......#.#...#...#...#...#.....#.......#.#.#.#.................#
#.###.#######.###.#.#.#.###########.###.#.#.#####.#######.#######.#.#.###.#.#######.#.#########.#.#.###.###.#.#####.#.#.#.#.#######.###.###.#
#...#...........#.#.#...#...#...#.#.....#...#...#...#...#.......#...#.....#.......#.#...........#.#.#.#...#.#...#...#.#...#.....#...#.#.#...#
###.#.#######.#.#.#######.#.#.#.#.###.#######.#.###.#.#.#.#########.###.###.###.###.#.#####.#.#.#.#.#.#.###.#.#.#.###.#.###.###.#.###.#.#.###
#...#...#.....#.#.........#...#.....#...#.....#.#.....#.#.#...........#...#...#.#...#.#...#.#.#.#.....#...#.#.#.#.#...#...#.....#.#.........#
#.#####.#.###.#.#.###.#########.#######.#.#####.#######.###.#########.###.###.#.#.#.###.#.###.#########.#.#.###.#.###.###.#.#####.#.###.#.#.#
#.#.............#.#...#.........#.....#...#...#...#...#.....#...........#...#.#.#.....#.#.#...#...#...#.#.#...#.#.#...#...#.....#.#.#...#...#
#.#.#.###.###.#.###.#############.#.#.#####.#.###.#.###.#####.#.#.#####.###.#.#######.#.#.#.#.#.#.#.#.###.###.#.#.#.###.#.###.#.#.#.#.#####.#
#.#.#.....#...#...#...............#.#...#...#...#.#.....#...#.#...#...#...#.#.......#...#.#.#.#.#...#.#.....#...#.#...#.#...#.......#.#...#.#
#.#.#####.#.#####.###########.#####.#####.#.###.#.#######.#.#.###.#.#.###.#####.###.#####.#.###.#####.#.#.#######.###.#.#.#.###.###.#.#.#.#.#
#.#.#...#...#...............#.....#...#...#.#...#.#.......#...#.#...#.#.......#...#...#...#.#...#.......#.#.....#...#.#...#...#.#...#...#...#
#.#.#.#.###.###############.#########.#.###.#.#.#.#.###########.#####.#######.#######.#.###.#.#############.###.###.#.#.#.###.#.###.#########
#.#...#...#.#.....#.#.....#.#.......#.....#.#...#...#...................#...#...#.....#...#.#...........#...#...#.#...#.#.....#...#...#.....#
#.###.###.###.###.#.#.#.#.#.#.#####.#####.#.###.#.###.#.#####.#######.#.#.###.#.#.#######.#.###########.#.###.#.#.#####.#######.#.###.#.#####
#...#...#.#.....#.#.#.#.#.#.#...#.#.#...#.....#.......#...#.#.#.....#.#.#.#.....#.#.....#...#.........................#...#...#.#.#.....#...#
###.#####.#.#####.#.#.#.#.#.###.#.#.#.#.#.###.###.#######.#.#.#.###.###.#.#.#.###.#.###.###.#.###.###.#.#.#.#####.#######.#.#.#.#.#.#####.#.#
#.#.....#.......................#.#...#.#.#.....#...........#.#.#.#.....#...#...#.#.#.#...#.#.....#.#.#...#.........#.....#.#...#.#...#...#.#
#.#####.#####.#.#.#.###.#.#######.#####.#.#.#########.#######.#.#.#######.###.#.#.#.#.#.###.#.#####.#.#########.#.#.#.#####.#.###.###.#.###.#
#.#...#.......#...#...#.#.....#.......#.#...#.......#.#.......#.#...#.....#...#...#...#...#.#.......#.....#.....#.#.#.#.....#.#.#...#.#.....#
#.#.#.#############.#.#.#####.###.###.#.###.#.#####.###.###.###.#.###.#####.#.#######.###.#.#######.#.#####.###.#.#.#.#.#####.#.###.###.#.###
#...#...............#.#.#...#...#.#...#...#.#.#...#.....#.#.#...#.....#.......#.....#.#...#...#.....#.....#...#.#.#.#.#...#...#...#.....#...#
#.#####.#.#########.#.#.#.#.#.#.#.#.#####.#.#.#.#########.#.#.###.###########.#.###.#.#.#.###.#######.###.###.###.#.#.###.#.#####.#######.#.#
#.#.....#.#.......#.#.#...#.#.....#.....#.#.#.#...............................#...#...#.#...#...........#...........#...#.........#.....#.#.#
###.#.#####.#####.###.#####.#.#####.#.###.#.#.#.#####.#.#.#####.#.###########.###.#####.#############.#.#.#.#####.#.###.#####.#####.###.#.#.#
#...#.....#...#...#...#.......................#.#...#.#.#.#...#.#...#...#.......#.#...#.#...........#...#.#.#.........#.....#.#...#...#.....#
#.###.###.###.#.###.#####.###.#########.###.#.#.#.#.#.###.###.#.#.#.#.#.#########.#.#.#.#.###.#######.#####.#.###.#########.###.#.###.#####.#
#S....................................#.......#...#.#.........#...#...#.....................................#...................#.....#.....#
#############################################################################################################################################"
}
