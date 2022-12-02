with open('day_two_input.txt') as f:
    games = f.read().split('\n')
    print(sum('L--D--W'.find('D-W-L-L-D-W-W-L-D'['AXAYAZBXBYBZCXCYCZ'.find(game.replace(' ', ''))]) + '-XYZ'.find(game[-1]) for game in games)) # Part 1
    print(sum('-RPS'.find('S-R-P-R-P-S-P-S-R'['AXAYAZBXBYBZCXCYCZ'.find(game.replace(' ', ''))]) + 'X--Y--Z'.find(game[-1]) for game in games)) # Part 2