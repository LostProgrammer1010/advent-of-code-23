def main():
    with open("./input.txt", "r") as file:
        games = file.read().split("\n")
    
        total_power = 0
        for game in games:
            green = 0
            red = 0
            blue = 0
            current_game = game.split(":")
            current_game = current_game[1]
            
            for round in current_game.split(";"):
                cubes = round.split(", ")

                for color in cubes:
                    color = color.strip().split(" ")
                    color = (int(color[0]), color[1])
                    
                    match color[1]:
                        case "blue":
                            if blue == 0 or color[0] > blue:
                                blue = color[0]
                        case "red":
                            if red == 0 or color[0] > red:
                                red = color[0]
                        case "green":
                            if green == 0 or color[0] > green:
                                green = color[0]

            total_power += blue * red * green
            print(total_power)





if __name__ == "__main__":
    main() 
