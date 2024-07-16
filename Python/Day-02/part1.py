MAX_BLUE = 14
MAX_GREEN = 13
MAX_RED = 12

def main():
    with open("./input.txt", "r") as file:
        games = file.read().split("\n")
        
        total_possible = 0
        for game in games:
            possible = True
            current_game = game.split(":")
            current_game_number = int(current_game[0].split(" ")[1])
            current_game = current_game[1]
            
            for round in current_game.split(";"):
                cubes = round.split(", ")

                for color in cubes:
                    color = color.strip().split(" ")
                    color = (int(color[0]), color[1])
                    
                    match color[1]:
                        case "blue":
                            if color[0] > 14:
                                possible = False
                                break
                        case "red":
                            if color[0] > 12:
                                possible = False
                                break
                        case "green":
                            if color[0] > 13:
                                possible = False
                                break
            if possible:
                total_possible += current_game_number
        print(total_possible)



if __name__ == "__main__":
    main() 
